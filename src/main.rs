use serde::Deserialize;
use std::env;
use reqwest::Error;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Price {
    priceChangePercent: String,
    lastPrice: String,
}

async fn get_price(symbol: String) -> Result<Price, Error> {
    let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}USDT", symbol);
    let response = reqwest::get(&url).await?.json::<Price>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("Getting prices for the {:?} coins...", args.len());
    for arg in &args {
        match get_price(arg.to_uppercase()).await {
            Ok(data) => println!("{} price: ${} ({}%)", arg.to_uppercase(), data.lastPrice, data.priceChangePercent),
            Err(_e) => eprintln!("{} does not exist.", arg.to_uppercase()),
        }
    }
}