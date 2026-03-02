//! Integration tests for spot deploy actions.
//!
//! These tests require a funded testnet wallet with spot deploy permissions.
//! Run with:
//! ```bash
//! HL_PRIVATE_KEY=0x... cargo test --features integration-tests integration_spot_deploy
//! ```

#![cfg(feature = "integration-tests")]

mod common;

use rust_decimal_macros::dec;

use hl_rs::actions::{
    Genesis, RegisterHyperliquidity, RegisterSpot, RegisterToken, SetDeployerFees, UserGenesis,
};

use crate::common::{log_action, log_response, send_action, signer_address};

// ============================================================================
// RegisterToken
// ============================================================================

#[tokio::test]
async fn test_register_token() {
    let action = RegisterToken::new(
        "TESTTOKEN",  // name
        8,            // sz_decimals
        18,           // wei_decimals
        100_000,      // max_gas
        "Test Token", // full_name
    );
    log_action("RegisterToken", &action);

    let result = send_action(action).await;
    log_response("RegisterToken", &result);
}

// ============================================================================
// Genesis
// ============================================================================

#[tokio::test]
async fn test_genesis() {
    // Token ID would need to be obtained from a previous RegisterToken
    let token_id: u32 = 999; // Placeholder - use actual registered token ID

    let action = Genesis::new(token_id, "1000000000000000000000000", false); // 1M tokens
    log_action("Genesis", &action);

    let result = send_action(action).await;
    log_response("Genesis", &result);
}

#[tokio::test]
async fn test_genesis_no_hyperliquidity() {
    let token_id: u32 = 999; // Placeholder

    let action = Genesis::new(token_id, "1000000000000000000000000", true);
    log_action("Genesis (no hyperliquidity)", &action);

    let result = send_action(action).await;
    log_response("Genesis (no hyperliquidity)", &result);
}

// ============================================================================
// UserGenesis
// ============================================================================

#[tokio::test]
async fn test_user_genesis() {
    let token_id: u32 = 999; // Placeholder

    let action = UserGenesis::new(
        token_id,
        vec![
            // Distribute to specific addresses
            (
                signer_address().to_string(),
                "500000000000000000000000".to_string(),
            ), // 500k tokens
        ],
        vec![], // No existing token contributions
    );
    log_action("UserGenesis", &action);

    let result = send_action(action).await;
    log_response("UserGenesis", &result);
}

// ============================================================================
// RegisterSpot
// ============================================================================

#[tokio::test]
async fn test_register_spot() {
    // Tokens [base, quote] - quote is typically USDC (token 0)
    let base_token: u32 = 999; // Placeholder
    let quote_token: u32 = 0; // USDC

    let action = RegisterSpot::new(base_token, quote_token);
    log_action("RegisterSpot", &action);

    let result = send_action(action).await;
    log_response("RegisterSpot", &result);
}

// ============================================================================
// RegisterHyperliquidity
// ============================================================================

#[tokio::test]
async fn test_register_hyperliquidity() {
    let spot_id: u32 = 999; // Placeholder - use actual spot market ID

    let action = RegisterHyperliquidity::new(
        spot_id,
        dec!(1.0),   // start_px
        dec!(100.0), // order_sz
        10,          // n_orders
    );
    log_action("RegisterHyperliquidity", &action);

    let result = send_action(action).await;
    log_response("RegisterHyperliquidity", &result);
}

#[tokio::test]
async fn test_register_hyperliquidity_with_seeded_levels() {
    let spot_id: u32 = 999; // Placeholder

    let action =
        RegisterHyperliquidity::new(spot_id, dec!(1.0), dec!(100.0), 10).with_seeded_levels(5); // 5 seeded price levels
    log_action("RegisterHyperliquidity (with seeded levels)", &action);

    let result = send_action(action).await;
    log_response("RegisterHyperliquidity (with seeded levels)", &result);
}

// ============================================================================
// SetDeployerFees
// ============================================================================

#[tokio::test]
async fn test_set_deployer_fees() {
    let token_id: u32 = 999; // Placeholder

    let action = SetDeployerFees::new(token_id, "0.001"); // 0.1% fee share
    log_action("SetDeployerFees", &action);

    let result = send_action(action).await;
    log_response("SetDeployerFees", &result);
}
