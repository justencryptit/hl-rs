//! Shared test utilities for integration tests.
//!
//! These tests require a funded testnet wallet. Run with:
//! ```bash
//! PRIVATE_KEY=${PRIVATE_KEY} cargo test --features integration-tests
//! ```

use std::env;

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use serde::Serialize;

use hl_rs::actions::Action;
use hl_rs::{BaseUrl, Error, ExchangeClient, ExchangeResponse};

/// Load the private key from the environment variable `PRIVATE_KEY`.
pub fn load_private_key() -> PrivateKeySigner {
    let key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set to run integration tests");
    key.parse::<PrivateKeySigner>()
        .expect("PRIVATE_KEY must be a valid private key")
}

/// Get the signer address for logging purposes.
pub fn signer_address() -> Address {
    load_private_key().address()
}

/// Create a configured exchange client for testnet.
pub fn testnet_client() -> ExchangeClient {
    ExchangeClient::new(BaseUrl::Testnet).with_signer(load_private_key())
}

/// Send an action to the exchange and return the response.
pub async fn send_action<T: Action + Serialize + std::fmt::Debug>(
    action: T,
) -> Result<ExchangeResponse, Error> {
    let client = testnet_client();
    client.send_action(action).await
}

/// Helper to print action details before sending.
pub fn log_action<T: std::fmt::Debug>(name: &str, action: &T) {
    println!("\n=== {} ===", name);
    println!("Signer: {}", signer_address());
    println!("Action: {:#?}", action);
}

/// Helper to print response after receiving.
pub fn log_response(name: &str, result: &Result<ExchangeResponse, Error>) {
    match result {
        Ok(response) => println!("{} response: {:#?}", name, response),
        Err(e) => {
            println!("{} error: {:#?}", name, e);
            panic!("{} error: {:#?}", name, e);
        }
    }
}

/// Constants for testing - use addresses that won't cause real transfers.
pub mod test_addresses {
    use alloy::primitives::{address, Address};

    /// A burn address for testing transfers that should fail validation.
    pub fn burn_address() -> Address {
        Address::repeat_byte(0x00)
    }

    /// A test destination address (non-zero but not a real user).
    pub fn test_destination() -> Address {
        address!("0x0EDa5eAc8f4C1F225E59fD6F23BDBd8097876a1e")
    }

    /// Another test address for multi-address scenarios.
    pub fn test_destination_2() -> Address {
        address!("0xc54dab54be3dC2721331105536060DC3A304f70c")
    }
}

/// Test DEX identifiers.
pub mod test_dex {
    /// A test DEX name for perp deploy actions.
    /// This should be a DEX the test wallet has deploy permissions for.
    pub const PERP_DEX: &str = "slob";

    /// A test coin/asset name.
    pub const TEST_COIN: &str = "TEST0";
}
