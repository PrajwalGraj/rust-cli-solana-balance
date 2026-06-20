use std::io;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize,Debug)]
struct ResponseValue{
    value: u64
}

#[derive(Deserialize,Debug)]
struct Response{
    result: Option<ResponseValue>,
    error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

#[tokio::main]
async fn main() {
    let mut wallet_address = String::new();
    println!("Enter Solana Wallet Address:");
    io::stdin().read_line(&mut wallet_address).expect("Failed to read address");
    let wallet_address = wallet_address.trim();

    let url = "https://api.devnet.solana.com";

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [wallet_address]
    });

    let client = reqwest::Client::new();
    let res = client   
        .post(url)
        .json(&payload)
        .header("User-Agent", "rust-app")
        .send()
        .await
        .unwrap();


    let data : Response= res.json().await.unwrap();

    match data.result{
        Some(balance_data) =>{
            println!("{} SOL",balance_data.value as f64/1_000_000_000.0);
        },
        None => {
            if let Some(error) = data.error{
                println!("RPC Error {}:{}", error.code,error.message);
            }else{
                println!("RPC returned unexpected response");
            }
        }

    }
    
}
