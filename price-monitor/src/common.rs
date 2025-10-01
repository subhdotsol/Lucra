use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PriceError {
    pub message: String,
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Price Error: {}", self.message)
    }
}

impl Error for PriceError {}

#[derive(Debug, Clone)]
pub struct PriceData {
    pub exchange: String,
    pub price: f64,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

impl PriceData {
    pub fn new(exchange: &str, price: f64) -> Self {
        Self {
            exchange: exchange.to_string(),
            price,
            timestamp: chrono::Local::now(),
        }
    }
}

pub fn calculate_price_difference(price1: f64, price2: f64) -> f64 {
    (price1 - price2).abs()
}

pub fn calculate_percentage_difference(price1: f64, price2: f64) -> f64 {
    let avg = (price1 + price2) / 2.0;
    if avg != 0.0 {
        ((price1 - price2).abs() / avg) * 100.0
    } else {
        0.0
    }
}