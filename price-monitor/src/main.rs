mod binance;
mod hyperliquid;
mod common;

use chrono::Local;
use tokio::time::{sleep, Duration};
use common::{calculate_price_difference, calculate_percentage_difference};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SOL price monitor...");
    println!("Comparing prices between Binance and Hyperliquid");
    println!("Press Ctrl+C to stop\n");

    loop {
        match fetch_and_compare_prices().await {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
        
        // Wait 5 seconds before next iteration
        sleep(Duration::from_secs(5)).await;
    }
}

async fn fetch_and_compare_prices() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let datetime_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Fetch prices from both exchanges concurrently
    let (binance_result, hyperliquid_result) = tokio::join!(
        binance::get_sol_price(),
        hyperliquid::get_sol_price()
    );
    
    match (binance_result, hyperliquid_result) {
        (Ok(binance_price), Ok(hyperliquid_price)) => {
            let price_diff = calculate_price_difference(binance_price, hyperliquid_price);
            let percentage_diff = calculate_percentage_difference(binance_price, hyperliquid_price);
            
            println!("[{}]", datetime_str);
            println!("  Binance:    ${:.4}", binance_price);
            println!("  Hyperliquid: ${:.4}", hyperliquid_price);
            println!("  Difference:  ${:.4} ({:.2}%)", price_diff, percentage_diff);
            
            if percentage_diff > 1.0 {
                println!("  ⚠️  Large price difference detected!");
            }
            println!();
        },
        (Err(binance_err), Ok(hyperliquid_price)) => {
            println!("[{}] Binance error: {} | Hyperliquid: ${:.4}", datetime_str, binance_err, hyperliquid_price);
        },
        (Ok(binance_price), Err(hyperliquid_err)) => {
            println!("[{}] Binance: ${:.4} | Hyperliquid error: {}", datetime_str, binance_price, hyperliquid_err);
        },
        (Err(binance_err), Err(hyperliquid_err)) => {
            println!("[{}] Both APIs failed - Binance: {} | Hyperliquid: {}", datetime_str, binance_err, hyperliquid_err);
        }
    }
    
    Ok(())
}
