use std::str::FromStr;

use alloy::signers::local::PrivateKeySigner;
use hl_rs::{BaseUrl, ExchangeClient, SetOracle};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let url = BaseUrl::Testnet;
    let dex_name = "slob";

    // SetOracle updates oracle prices, mark prices, and external perp prices
    // - oraclePxs: sorted list of (asset, oracle_price)
    // - markPxs: outer list can be length 0, 1, or 2; median with local mark price is used
    // - externalPerpPxs: sorted list of (asset, external_price) to prevent sudden mark deviations
    //
    // SetOracle can be called multiple times but there must be at least 2.5 seconds between calls.
    // Stale mark prices will fall back to local mark price after 10 seconds of no updates.
    // Deployers are expected to call setOracle every 3 seconds even with no changes.
    let yams_px = "420.0".to_string();

    let test0_px = "10.45".to_string();
    let action = SetOracle {
        dex: dex_name.to_string(),
        oracle_pxs: vec![
            ("slob:TEST0".to_string(), test0_px.clone()),
            ("slob:TEST1".to_string(), "10".to_string()),
            ("slob:TEST2".to_string(), "10".to_string()),
            ("slob:BOOF".to_string(), "420.69".to_string()),
            ("slob:YAMS".to_string(), yams_px.clone()),
        ],
        // markPxs outer list can be length 0, 1, or 2
        // The median of these inputs along with the local mark price is used
        mark_pxs: vec![vec![
            ("slob:TEST0".to_string(), test0_px.clone()),
            ("slob:TEST1".to_string(), "10".to_string()),
            ("slob:TEST2".to_string(), "10".to_string()),
            ("slob:BOOF".to_string(), "420.69".to_string()),
            ("slob:YAMS".to_string(), yams_px.clone()),
        ]],
        // externalPerpPxs prevents sudden mark price deviations
        // Must include all assets
        external_perp_pxs: vec![
            ("slob:TEST0".to_string(), test0_px.clone()),
            ("slob:TEST1".to_string(), "10".to_string()),
            ("slob:TEST2".to_string(), "10".to_string()),
            ("slob:BOOF".to_string(), "420.69".to_string()),
            ("slob:YAMS".to_string(), yams_px),
        ],
        nonce: None,
    };

    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    let wallet = PrivateKeySigner::from_str(&private_key).unwrap();
    println!("wallet: {}", wallet.address());

    let client = ExchangeClient::new(url).with_signer(wallet);

    let result = client.send_action(action).await.unwrap();

    println!("Set oracle result: {:?}", result);
}
