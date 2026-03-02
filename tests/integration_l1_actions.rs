//! Integration tests for general L1 actions (non-trading, non-deploy).
//!
//! These tests require a funded testnet wallet.
//! Run with:
//! ```bash
//! HL_PRIVATE_KEY=0x... cargo test --features integration-tests integration_l1_actions
//! ```

#![cfg(feature = "integration-tests")]

mod common;

use alloy::primitives::Address;
use rust_decimal_macros::dec;

use hl_rs::actions::{
    AgentEnableDexAbstraction, CreateSubAccount, NoOp, SetReferrer, SubAccountSpotTransfer,
    SubAccountTransfer, ToggleBigBlocks, VaultTransfer,
};

use crate::common::{log_action, log_response, send_action, test_addresses};

// ============================================================================
// NoOp
// ============================================================================

#[tokio::test]
async fn test_no_op() {
    let action = NoOp::new();
    log_action("NoOp", &action);

    let result = send_action(action).await;
    log_response("NoOp", &result);

    // NoOp should always succeed
    assert!(result.is_ok());
}

// ============================================================================
// ToggleBigBlocks
// ============================================================================

#[tokio::test]
async fn test_toggle_big_blocks_enable() {
    let action = ToggleBigBlocks::enable();
    log_action("ToggleBigBlocks (enable)", &action);

    let result = send_action(action).await;
    log_response("ToggleBigBlocks (enable)", &result);
}

#[tokio::test]
async fn test_toggle_big_blocks_disable() {
    let action = ToggleBigBlocks::disable();
    log_action("ToggleBigBlocks (disable)", &action);

    let result = send_action(action).await;
    log_response("ToggleBigBlocks (disable)", &result);
}

#[tokio::test]
async fn test_toggle_big_blocks_builder() {
    let action = ToggleBigBlocks::builder()
        .using_big_blocks(true)
        .build()
        .unwrap();
    log_action("ToggleBigBlocks (builder)", &action);

    let result = send_action(action).await;
    log_response("ToggleBigBlocks (builder)", &result);
}

// ============================================================================
// SetReferrer
// ============================================================================

#[tokio::test]
async fn test_set_referrer() {
    let action = SetReferrer::new("TESTCODE");
    log_action("SetReferrer", &action);

    let result = send_action(action).await;
    log_response("SetReferrer", &result);

    // May fail if referrer code doesn't exist or already set
}

// ============================================================================
// CreateSubAccount
// ============================================================================

#[tokio::test]
async fn test_create_sub_account() {
    let action = CreateSubAccount::new("test_sub_account");
    log_action("CreateSubAccount", &action);

    let result = send_action(action).await;
    log_response("CreateSubAccount", &result);

    // May fail if sub-account already exists
}

// ============================================================================
// SubAccountTransfer
// ============================================================================

#[tokio::test]
async fn test_sub_account_transfer_deposit() {
    // Use a test sub-account address (would need to be created first)
    let sub_account = test_addresses::test_destination();

    let action = SubAccountTransfer::deposit(sub_account, 10_000); // $.01
    log_action("SubAccountTransfer (deposit)", &action);

    let result = send_action(action).await;
    log_response("SubAccountTransfer (deposit)", &result);
}

#[tokio::test]
async fn test_sub_account_transfer_withdraw() {
    let sub_account = test_addresses::test_destination();

    let action = SubAccountTransfer::withdraw(sub_account, 500_000); // $0.50
    log_action("SubAccountTransfer (withdraw)", &action);

    let result = send_action(action).await;
    log_response("SubAccountTransfer (withdraw)", &result);
}

// ============================================================================
// SubAccountSpotTransfer
// ============================================================================

#[tokio::test]
async fn test_sub_account_spot_transfer_deposit() {
    let sub_account = test_addresses::test_destination();

    let action = SubAccountSpotTransfer::deposit(sub_account, "USDC", dec!(1.0));
    log_action("SubAccountSpotTransfer (deposit)", &action);

    let result = send_action(action).await;
    log_response("SubAccountSpotTransfer (deposit)", &result);
}

#[tokio::test]
async fn test_sub_account_spot_transfer_withdraw() {
    let sub_account = test_addresses::test_destination();

    let action = SubAccountSpotTransfer::withdraw(sub_account, "USDC", dec!(0.5));
    log_action("SubAccountSpotTransfer (withdraw)", &action);

    let result = send_action(action).await;
    log_response("SubAccountSpotTransfer (withdraw)", &result);
}

// ============================================================================
// VaultTransfer
// ============================================================================

#[tokio::test]
async fn test_vault_transfer_deposit() {
    // Use a test vault address
    let vault = Address::repeat_byte(0x55);

    let action = VaultTransfer::deposit(vault, 1_000_000); // $1
    log_action("VaultTransfer (deposit)", &action);

    let result = send_action(action).await;
    log_response("VaultTransfer (deposit)", &result);

    // May fail if vault doesn't exist
}

#[tokio::test]
async fn test_vault_transfer_withdraw() {
    let vault = Address::repeat_byte(0x55);

    let action = VaultTransfer::withdraw(vault, 500_000); // $0.50
    log_action("VaultTransfer (withdraw)", &action);

    let result = send_action(action).await;
    log_response("VaultTransfer (withdraw)", &result);
}

// ============================================================================
// AgentEnableDexAbstraction
// ============================================================================

#[tokio::test]
async fn test_agent_enable_dex_abstraction() {
    let action = AgentEnableDexAbstraction::new();
    log_action("AgentEnableDexAbstraction", &action);

    let result = send_action(action).await;
    log_response("AgentEnableDexAbstraction", &result);
}
