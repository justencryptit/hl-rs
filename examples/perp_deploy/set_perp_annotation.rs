use std::str::FromStr;

use alloy::signers::local::PrivateKeySigner;
use hl_rs::{BaseUrl, ExchangeClient, SetPerpAnnotation};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let url = BaseUrl::Mainnet;

    // For a DEX-deployed perp (formats coin as "dex:SYMBOL"):
    // let action = SetPerpAnnotation::new("mydex", "BTC", "Crypto", "Bitcoin perpetual contract");

    // For a DEX-deployed perp, use ::new which formats coin as "dex:SYMBOL":
    let action = SetPerpAnnotation::new(
        "km",
        "USOIL",
        "commodities",
        "Provides exposure to U.S. listed instrument(s) that seek to track daily price movements of WTI crude oil through near-month futures contracts. This listing does not reflect the spot price of crude oil. Please refer to km docs for further information on oracle design. Primary oracle pricing reference: NYSE:USO.",
    );

    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    let wallet = PrivateKeySigner::from_str(&private_key).unwrap();
    println!("wallet: {}", wallet.address());

    let client = ExchangeClient::new(url).with_signer(wallet);

    let result = client.send_action(action).await.unwrap();

    println!("Set perp annotation result: {:?}", result);
}
