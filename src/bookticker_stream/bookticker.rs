use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};
use urlencoding::encode;

#[derive(Debug, Clone, Deserialize)]
pub struct BestPrices {
    pub bid: f64,
    pub ask: f64,
}

#[derive(Deserialize, Debug)]
pub struct StreamBookTicker {
    pub stream: String,
    pub data: BookTicker,
}

#[derive(Deserialize, Debug)]
pub struct BookTicker {
    #[serde(rename = "e")]
    pub event: String,
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid: String,
    #[serde(rename = "B")]
    pub bid_qty: String,
    #[serde(rename = "a")]
    pub best_ask: String,
    #[serde(rename = "A")]
    pub ask_qty: String,
    #[serde(rename = "T")]
    pub trans_time: u64,
    #[serde(rename = "E")]
    pub event_time: u64,
}

#[derive(Debug, Clone)]
pub struct BookTickerStream {
    pub book_ticker: Arc<std::sync::Mutex<HashMap<String, BestPrices>>>,
}

// Default trait
impl Default for BookTickerStream {
    fn default() -> Self {
        Self::new()
    }
}

impl BookTickerStream {
    pub fn new() -> Self {
        BookTickerStream {
            book_ticker: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub async fn listen_one_coin_bookticker(
        &self,
        url: &String,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        loop {
            let (ws_stream, _) = match connect_async(url).await {
                Ok(stream) => {
                    info!("Listen to Book Ticker Stream");
                    stream
                }
                Err(e) => {
                    error!("Failed to connect: {}, retrying...", e);
                    continue; // Retry immediately without delay
                }
            };
            let (mut write, mut read) = ws_stream.split();
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let ticker: StreamBookTicker =
                            serde_json::from_str(&text).expect("JSON was not well format!");
                        let bid: f64 = ticker
                            .data
                            .best_bid
                            .parse::<f64>()
                            .expect("Failed to parse as f64");
                        let ask: f64 = ticker
                            .data
                            .best_ask
                            .parse::<f64>()
                            .expect("Failed to parse as f64");

                        let mut book_ticker = self.book_ticker.lock().unwrap();
                        book_ticker.insert(ticker.data.symbol.clone(), BestPrices { bid, ask });
                    }
                    Ok(Message::Ping(payload)) => {
                        if let Err(e) = write.send(Message::Pong(payload)).await {
                            error!("Failed to send Pong response: {}", e);
                        }
                    }
                    Ok(Message::Close(close)) => {
                        info!("Receive Close frame, retry immediately {:?}", close);
                        break;
                    }
                    Ok(Message::Binary(binary_data)) => {
                        info!("Received Binary data {:?}", binary_data);
                    }
                    Ok(Message::Pong(pong)) => {
                        info!("Pong received {:?}", pong);
                    }
                    Ok(Message::Frame(frame)) => {
                        info!("Frame received {:?}", frame)
                    }
                    Err(e) => {
                        error!("Error Message {}", e);
                        break;
                    }
                }
            }
            error!("Book Ticker Connection lost, retrying immediately...");
        }
    }

    pub async fn listen_all_coins_bookticker(&self, names: Vec<String>, parition: usize) {
        let urls = generate_bookticker_url_in_n_pieces(names, parition);
        for url in &urls {
            println!("Url {:?} \n", &url);
        }
        let mut tasks = vec![];
        for url in urls {
            let self_clone = self.clone();
            tasks.push(tokio::spawn(async move {
                if let Err(e) = self_clone.listen_one_coin_bookticker(&url).await {
                    info!(
                        "Unable to connect the websockets stream URL {:?} {:?}",
                        &url, e
                    );
                };
            }))
        }
        for task in tasks {
            task.await.unwrap();
        }
    }

    pub async fn show_bookticker(&self) {
        loop {
            time::sleep(time::Duration::new(1800, 0)).await;
            let book_ticker = self.book_ticker.lock().unwrap();
            info!("Current Book Ticker:");
            for (symbol, prices) in book_ticker.iter() {
                info!("{}: Bid: {}, Ask: {}", symbol, prices.bid, prices.ask);
            }
        }
    }

    pub async fn show_btc_only(&self) {
        loop {
            time::sleep(time::Duration::new(5, 0)).await;
            let book_ticker = self.book_ticker.lock().unwrap();

            book_ticker
                .iter()
                .filter(|&(symbol, _)| symbol == &"BTCUSDT")
                .for_each(|(_, prices)| {
                    info!(
                        "Special notice for BTCUSDT: Bid: {}, Ask: {}",
                        prices.bid, prices.ask
                    );
                });
        }
    }
}

fn create_websocket_url(coin_names: &[String]) -> String {
    let streams: Vec<String> = coin_names
        .iter()
        .map(|coin| {
            let binding = coin.to_lowercase();
            let encoded_coin = encode(&binding);
            format!("{}@bookTicker", encoded_coin)
        })
        .collect();
    format!(
        "wss://fstream.binance.com/stream?streams={}",
        streams.join("/")
    )
}

fn generate_bookticker_url_in_n_pieces(coin_names: Vec<String>, n: usize) -> Vec<String> {
    let total_length = coin_names.len();
    let piece_size = total_length / n;
    let remainder = total_length % n;
    let mut urls = Vec::new();
    let mut start = 0;

    for i in 0..n {
        let current_piece_size = piece_size + if i < remainder { 1 } else { 0 };
        let end = start + current_piece_size;
        let piece = &coin_names[start..end];
        urls.push(create_websocket_url(piece.to_vec().as_slice()));
        start = end; // Update the start index for the next piece
    }
    urls
}
