use tracing::{info, Level};
use tracing_subscriber;

pub mod async_binance;
pub mod aws_resources;
pub mod order_stream;
use async_binance::client_async::AsyncBinanceClient;
use aws_resources::clients::get_ssm_client;
use aws_resources::ssm_params::get_param_value;
use order_stream::order_update::UserDataStream;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let ssm_client = get_ssm_client().await?;
    let binance_api_key = get_param_value(&ssm_client, "binance-api-key".to_string()).await?;
    let binance_secret_key = get_param_value(&ssm_client, "binance-secret-key".to_string()).await?;
    let binance_future_client = AsyncBinanceClient::new(
        binance_api_key,
        binance_secret_key,
        "https://fapi.binance.com/fapi/v1/".to_string(),
        Some(30),
    );
    let listen_key: String = binance_future_client.get_listen_key().await?;
    let user_data_stream = UserDataStream {
        listen_key: listen_key.clone(),
    };
    let user_data_listener_task = {
        let user_data_stream_clone = user_data_stream.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = user_data_stream_clone.listen_user_data().await {
                    info!("Error listening to user data stream: {:?}", e);
                    continue;
                }
            }
        })
    };

    let keep_listen_key_alive_task = tokio::spawn(async move {
        let interval = tokio::time::Duration::from_secs(1800);
        tokio::time::sleep(interval).await;
        loop {
            if let Err(e) = binance_future_client
                .keep_listen_key_alive(&listen_key.clone())
                .await
            {
                info!("Error in keeping ListenKey alive {:?}", e);
                continue;
            }
            info!("Send Keep Alive message every 1800 seconds");
            tokio::time::sleep(interval).await;
        }
    });

    let _ = tokio::try_join!(user_data_listener_task, keep_listen_key_alive_task);
    Ok(())
}
