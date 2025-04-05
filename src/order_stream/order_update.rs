use crate::order_stream::messages::UserDataUpdate;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};

#[derive(Clone, Debug)]
pub struct UserDataStream {
    pub listen_key: String,
}

impl UserDataStream {
    pub async fn listen_user_data(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        loop {
            let url = format!("wss://fstream.binance.com/ws/{}", self.listen_key);
            let (ws_stream, _) = match connect_async(&url).await {
                Ok(stream) => {
                    info!("Listen to User Data Stream");
                    stream
                }
                Err(e) => {
                    error!("Failed to connect: {}, retrying...", e);
                    continue;
                }
            };
            let (mut write, mut read) = ws_stream.split();
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        self.handle_user_data_update(&text).await;
                    }
                    Ok(Message::Ping(payload)) => {
                        if let Err(e) = write.send(Message::Pong(payload)).await {
                            error!("Failed to send Pong response: {}", e);
                        }
                    }
                    Ok(Message::Binary(binary)) => {
                        info!("{:?}", &binary);
                    }
                    Ok(Message::Close(close_frame)) => {
                        info!(
                            "Close Frame received {:?}, retrying immediately",
                            close_frame
                        );
                    }
                    Ok(non_text_message) => {
                        info!("Received Non-Text Message: {:?}", non_text_message);
                    }
                    Err(e) => {
                        error!("Error Message: {}", e);
                        break;
                    }
                }
            }
            error!("User Data Connection lost, retrying immediately...");
        }
    }
    async fn handle_user_data_update(&self, text: &str) {
        match serde_json::from_str::<UserDataUpdate>(text) {
            Ok(update) => {
                self.process_update(update).await;
            }
            Err(e) => {
                info!("Failed to deserialize message: {}\n, text: {}\n", e, text);
            }
        }
    }

    async fn process_update(&self, update: UserDataUpdate) {
        match update {
            UserDataUpdate::OrderTradeUpdate(order_update) => {
                info!("Received OrderTradeUpdate: {:?}", order_update);
            }
            UserDataUpdate::ListenKeyExpired(key_expired) => {
                info!("Listen Key is expired. Resend a key {:?}", key_expired);
                // TODO: Add &binance_client to
            }
            UserDataUpdate::AccountConfigUpdate(acc_update) => {
                info!("Received Account Update {:?}", acc_update);
            }
            UserDataUpdate::TradeLite(trade_lite) => {
                info!("Received Trade Lite report {:?}", trade_lite);
            }
            _ => {
                // info!("Received update: {:?}", update);
            }
        }
    }
}
