use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct BinanceTicker {
    pub symbol: String,
    pub price: String,
}

pub async fn get_sol_price() -> Result<f64, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT";
    
    let response = client.get(url).send().await?;
    let ticker: BinanceTicker = response.json().await?;
    
    let price = ticker.price.parse::<f64>()?;
    Ok(price)
}