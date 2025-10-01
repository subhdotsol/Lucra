use chrono::Local;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;

#[derive(Deserialize, Debug)]
struct BinanceTicker {
    c: String, // Close price as string
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://stream.binance.com:9443/ws/solusdt@ticker";

    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to Binance WS for SOL/USDT price");

    let (_write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;

        if msg.is_text() {
            let text = msg.to_text()?;…

            #[derive(Deserialize)]
            struct TickerMessage {
                c: String,
            }

            let ticker: TickerMessage = serde_json::from_str(text)?;

            // Get local date and time formatted as YYYY-MM-DD HH:MM
            let now = Local::now();
            let datetime_str = now.format("%Y-%m-%d %H:%M").to_string();

            println!("At {}, SOL is around ${}", datetime_str, ticker.c);
        }
    }

    Ok(())
}
