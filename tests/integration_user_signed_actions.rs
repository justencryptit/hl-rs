//! Integration tests for user-signed actions.
//!
//! These tests require a funded testnet wallet.
//! Run with:
//! ```bash
//! HL_PRIVATE_KEY=0x... cargo test --features integration-tests integration_user_signed_actions
//! ```

#![cfg(feature = "integration-tests")]

mod common;

use alloy::primitives::Address;
use rust_decimal_macros::dec;

use hl_rs::actions::{
    ApproveAgent, ApproveBuilderFee, ConvertToMultiSigUser, MultiSigSigners, SendAsset,
    SpotTransfer, TokenDelegate, UsdClassTransfer, UsdSend, UserDexAbstraction, Withdraw,
};

use crate::common::{log_action, log_response, send_action, signer_address, test_addresses};

// ============================================================================
// UsdSend
// ============================================================================

#[tokio::test]
async fn test_usd_send() {
    let action = UsdSend::builder()
        .destination(test_addresses::test_destination())
        .amount(dec!(0.01))
        .build()
        .unwrap();
    log_action("UsdSend", &action);

    let result = send_action(action).await;
    log_response("UsdSend", &result);
}

// ============================================================================
// Withdraw
// ============================================================================

#[tokio::test]
async fn test_withdraw() {
    let action = Withdraw::builder()
        .destination(signer_address())
        .amount(dec!(0.01))
        .build()
        .unwrap();
    log_action("Withdraw", &action);

    let result = send_action(action).await;
    log_response("Withdraw", &result);

    // May fail due to insufficient balance
}

// ============================================================================
// SpotTransfer
// ============================================================================

#[tokio::test]
async fn test_spot_transfer() {
    let action = SpotTransfer::builder()
        .destination(test_addresses::test_destination())
        .token("USDC")
        .amount(dec!(0.01))
        .build()
        .unwrap();
    log_action("SpotTransfer", &action);

    let result = send_action(action).await;
    log_response("SpotTransfer", &result);
}

// ============================================================================
// UsdClassTransfer
// ============================================================================

#[tokio::test]
async fn test_usd_class_transfer_to_perp() {
    let action = UsdClassTransfer::builder()
        .amount(dec!(1.0))
        .to_perp(true)
        .build()
        .unwrap();
    log_action("UsdClassTransfer (to perp)", &action);

    let result = send_action(action).await;
    log_response("UsdClassTransfer (to perp)", &result);
}

#[tokio::test]
async fn test_usd_class_transfer_to_spot() {
    let action = UsdClassTransfer::builder()
        .amount(dec!(1.0))
        .to_perp(false)
        .build()
        .unwrap();
    log_action("UsdClassTransfer (to spot)", &action);

    let result = send_action(action).await;
    log_response("UsdClassTransfer (to spot)", &result);
}

// ============================================================================
// SendAsset
// ============================================================================

#[tokio::test]
async fn test_send_asset() {
    let action = SendAsset::builder()
        .destination(test_addresses::test_destination())
        .source_dex("spot")
        .destination_dex("")
        .token("USDC")
        .amount(dec!(0.01))
        .build()
        .unwrap();
    log_action("SendAsset", &action);

    let result = send_action(action).await;
    log_response("SendAsset", &result);
}

// ============================================================================
// ApproveAgent
// ============================================================================

#[tokio::test]
async fn test_approve_agent() {
    let agent = Address::repeat_byte(0x77);

    let action = ApproveAgent::new(agent, "test-agent");
    log_action("ApproveAgent", &action);

    let result = send_action(action).await;
    log_response("ApproveAgent", &result);
}

#[tokio::test]
async fn test_approve_agent_without_name() {
    let agent = Address::repeat_byte(0x88);

    let action = ApproveAgent::without_name(agent);
    log_action("ApproveAgent (without name)", &action);

    let result = send_action(action).await;
    log_response("ApproveAgent (without name)", &result);
}

#[tokio::test]
async fn test_approve_agent_builder() {
    let agent = Address::repeat_byte(0x99);

    let action = ApproveAgent::builder()
        .agent_address(agent)
        .agent_name("builder-agent".to_string())
        .build()
        .unwrap();
    log_action("ApproveAgent (builder)", &action);

    let result = send_action(action).await;
    log_response("ApproveAgent (builder)", &result);
}

// ============================================================================
// ApproveBuilderFee
// ============================================================================

#[tokio::test]
async fn test_approve_builder_fee() {
    let builder = Address::repeat_byte(0xAA);

    let action = ApproveBuilderFee::builder()
        .builder(builder)
        .max_fee_rate(dec!(0.001)) // 0.1% max fee
        .build()
        .unwrap();
    log_action("ApproveBuilderFee", &action);

    let result = send_action(action).await;
    log_response("ApproveBuilderFee", &result);
}

// ============================================================================
// TokenDelegate
// ============================================================================

#[tokio::test]
async fn test_token_delegate() {
    let validator = Address::repeat_byte(0xBB);

    let action = TokenDelegate::delegate(validator, 1_000_000_000_000_000_000); // 1 token
    log_action("TokenDelegate (delegate)", &action);

    let result = send_action(action).await;
    log_response("TokenDelegate (delegate)", &result);
}

#[tokio::test]
async fn test_token_undelegate() {
    let validator = Address::repeat_byte(0xBB);

    let action = TokenDelegate::undelegate(validator, 500_000_000_000_000_000); // 0.5 token
    log_action("TokenDelegate (undelegate)", &action);

    let result = send_action(action).await;
    log_response("TokenDelegate (undelegate)", &result);
}

#[tokio::test]
async fn test_token_delegate_builder() {
    let validator = Address::repeat_byte(0xCC);

    let action = TokenDelegate::builder()
        .validator(validator)
        .wei(1_000_000_000_000_000_000u64)
        .is_undelegate(false)
        .build()
        .unwrap();
    log_action("TokenDelegate (builder)", &action);

    let result = send_action(action).await;
    log_response("TokenDelegate (builder)", &result);
}

// ============================================================================
// UserDexAbstraction
// ============================================================================

#[tokio::test]
async fn test_user_dex_abstraction_enable() {
    let action = UserDexAbstraction::builder()
        .user(signer_address())
        .enabled(true)
        .build()
        .unwrap();
    log_action("UserDexAbstraction (enable)", &action);

    let result = send_action(action).await;
    log_response("UserDexAbstraction (enable)", &result);
}

#[tokio::test]
async fn test_user_dex_abstraction_disable() {
    let action = UserDexAbstraction::builder()
        .user(signer_address())
        .enabled(false)
        .build()
        .unwrap();
    log_action("UserDexAbstraction (disable)", &action);

    let result = send_action(action).await;
    log_response("UserDexAbstraction (disable)", &result);
}

// ============================================================================
// ConvertToMultiSigUser
// ============================================================================

#[tokio::test]
async fn test_convert_to_multisig_user() {
    let signers = MultiSigSigners {
        authorized_users: vec![
            signer_address(),
            Address::repeat_byte(0xDD),
            Address::repeat_byte(0xEE),
        ],
        threshold: 2, // 2-of-3 multisig
    };

    let action = ConvertToMultiSigUser::builder()
        .signers(signers)
        .build()
        .unwrap();
    log_action("ConvertToMultiSigUser", &action);

    let result = send_action(action).await;
    log_response("ConvertToMultiSigUser", &result);

    // This is destructive - converts the account to multisig!
    // Only run if you understand the implications
}
