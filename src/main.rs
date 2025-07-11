use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct AddressInfo {
    chain_stats: ChainStats,
}

#[derive(Deserialize)]
struct ChainStats {
    funded_txo_sum: u64,
    spent_txo_sum: u64,
}

async fn btcbalance_from_mempool_space(address: &str) -> Result<f64, Error> {
    // Construct the mempool.space API URL
    let url = format!("https://mempool.space/api/address/{}", address);

    // Make the HTTP request
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    // Parse the JSON response
    let address_info: AddressInfo = response.json().await?;

    // Calculate balance in satoshis
    let balance = address_info.chain_stats.funded_txo_sum - address_info.chain_stats.spent_txo_sum;

    // Convert to BTC (1 BTC = 100,000,000 satoshis)
    Ok(balance as f64 / 100_000_000.0)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get Bitcoin address from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <bitcoin_address>", args[0]);
        std::process::exit(1);
    }
    let address = &args[1];

    println!("Address: {}", address);
    match btcbalance_from_mempool_space(address).await {
        Ok(balance) => println!("Balance: {} BTC", balance),
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }

    Ok(())
}