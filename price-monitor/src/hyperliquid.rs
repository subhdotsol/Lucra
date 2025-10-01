use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize)]
pub struct HyperliquidRequest {
    #[serde(rename = "type")]
    pub request_type: String,
}

pub async fn get_sol_price() -> Result<f64, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://api.hyperliquid.xyz/info";
    
    let request_body = HyperliquidRequest {
        request_type: "allMids".to_string(),
    };
    
    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?;
    
    let data: HashMap<String, Value> = response.json().await?;
    
    if let Some(sol_price) = data.get("SOL") {
        if let Some(price_str) = sol_price.as_str() {
            let price = price_str.parse::<f64>()?;
            return Ok(price);
        } else if let Some(price_num) = sol_price.as_f64() {
            return Ok(price_num);
        }
    }
    
    Err("SOL price not found in Hyperliquid response".into())
}