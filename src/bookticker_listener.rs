pub mod bookticker_stream;
use bookticker_stream::bookticker::BookTickerStream;
use tracing::Level;
use tracing_subscriber;

pub mod async_binance;
use async_binance::client_async::AsyncBinanceClient;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let bookticker_partition: usize = 4;
    let binance_future_client = AsyncBinanceClient::new(
        None,
        None,
        "https://fapi.binance.com/fapi/v1/".to_string(),
        Some(30),
    );
    let coins_name = binance_future_client.get_available_coins_name().await;
    let bookticker_stream = BookTickerStream::new();
    bookticker_stream
        .listen_all_coins_bookticker(coins_name, bookticker_partition)
        .await;
    Ok(())
}
