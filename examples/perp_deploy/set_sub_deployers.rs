use std::str::FromStr;

use alloy::primitives::{address, Address};
use alloy::signers::local::PrivateKeySigner;
use hl_rs::{BaseUrl, ExchangeClient, SetSubDeployers, SubDeployerVariant};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let url = BaseUrl::Testnet;
    let dex_name = "slob";

    // Address to grant or revoke permissions (set SUB_DEPLOYER_ADDRESS in .env)
    let sub_deployer_address = address!("0xEcC1e0731Ca1cbd237c0564935637c4c9e899e41");

    // Enable permissions for the sub-deployer (e.g. SetOracle, HaltTrading)
    let action = SetSubDeployers::new(dex_name)
        .enable_permissions(sub_deployer_address, vec![SubDeployerVariant::HaltTrading]);

    // To revoke permissions instead, use:
    // let action = SetSubDeployers::new(dex_name)
    //     .disable_permissions(sub_deployer_address, vec![SubDeployerVariant::SetOracle]);

    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    let wallet = PrivateKeySigner::from_str(&private_key).unwrap();
    println!("wallet: {}", wallet.address());

    let client = ExchangeClient::new(url).with_signer(wallet);

    let result = client.send_action(action).await.unwrap();

    println!("Set sub deployers result: {:?}", result);
}
