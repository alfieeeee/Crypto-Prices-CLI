use serde::Deserialize;
use std::env;
use reqwest::Error;

#[derive(Deserialize)]
struct Price {
    price: String,
}

async fn get_price(symbol: String) -> Result<String, Error> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}USDT", symbol);
    let response = reqwest::get(&url).await?.json::<Price>().await?;
    Ok(response.price)
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("Getting prices for the {:?} coins...", args.len());
    for arg in &args {
        match get_price(arg.to_uppercase()).await {
            Ok(price) => println!("Current {} price: ${}", arg.to_uppercase(), price),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}