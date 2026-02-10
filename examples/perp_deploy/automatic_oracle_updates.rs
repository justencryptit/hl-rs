//! Automatic oracle updates: every 2.5s, increase the tracked mark price by 0.5%.
//! Initial mark is read once before the loop (from env); subsequent updates use in-memory state only.

use std::str::FromStr;
use std::time::Duration;

use alloy::signers::local::PrivateKeySigner;
use hl_rs::{BaseUrl, ExchangeClient, SetOracle};

const TICK_INTERVAL: Duration = Duration::from_millis(2500);
const INCREMENT_FACTOR: f64 = 1.0; // +0.9%

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let url = BaseUrl::Testnet;
    let dex_name = "slob";

    // Check current mark once before execution: use it as initial state.
    // In production you might fetch this from your oracle or the info API.
    let initial_mark: f64 = std::env::var("INITIAL_MARK_PRICE")
        .unwrap_or_else(|_| "2164".to_string())
        .parse()
        .expect("INITIAL_MARK_PRICE must be a number");

    // State: we only update this in memory each tick (increment by 0.5%).
    let mut state_mark = initial_mark;

    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    let wallet = PrivateKeySigner::from_str(&private_key).unwrap();
    println!("wallet: {}", wallet.address());
    println!(
        "Starting automatic oracle updates: initial mark {state_mark}, incrementing by 0.9% every {:?}",
        TICK_INTERVAL
    );

    let client = ExchangeClient::new(url).with_signer(wallet);

    // Fixed prices for other assets (same as set_oracle example).
    let test0_px = "10".to_string();
    let test1_px = "10".to_string();
    let test2_px = "10".to_string();
    let boof_px = "420.69".to_string();

    loop {
        let yams_px = format!("{state_mark:.4}");

        println!("Setting oracle with mark price: {yams_px}");
        let action = SetOracle {
            dex: dex_name.to_string(),
            oracle_pxs: vec![
                ("slob:TEST0".to_string(), test0_px.clone()),
                ("slob:TEST1".to_string(), test1_px.clone()),
                ("slob:TEST2".to_string(), test2_px.clone()),
                ("slob:BOOF".to_string(), boof_px.clone()),
                ("slob:YAMS".to_string(), yams_px.clone()),
            ],
            mark_pxs: vec![vec![
                ("slob:TEST0".to_string(), test0_px.clone()),
                ("slob:TEST1".to_string(), test1_px.clone()),
                ("slob:TEST2".to_string(), test2_px.clone()),
                ("slob:BOOF".to_string(), boof_px.clone()),
                ("slob:YAMS".to_string(), yams_px.clone()),
            ]],
            external_perp_pxs: vec![
                ("slob:TEST0".to_string(), test0_px.clone()),
                ("slob:TEST1".to_string(), test1_px.clone()),
                ("slob:TEST2".to_string(), test2_px.clone()),
                ("slob:BOOF".to_string(), boof_px.clone()),
                ("slob:YAMS".to_string(), yams_px.clone()),
            ],
            nonce: None,
        };

        match client.send_action(action).await {
            Ok(result) => println!("set_oracle ok @ mark {state_mark:.4}: {:?}", result),
            Err(e) => eprintln!("set_oracle error: {:?}", e),
        }

        // Increment in state for next tick (only in-memory; no refetch).
        state_mark *= INCREMENT_FACTOR;
        tokio::time::sleep(TICK_INTERVAL).await;
    }
}
