//! Integration tests for perp deploy actions.
//!
//! These tests require a funded testnet wallet with perp deploy permissions.
//! Run with:
//! ```bash
//! HL_PRIVATE_KEY=0x... cargo test --features integration-tests integration_perp_deploy
//! ```

#![cfg(feature = "integration-tests")]

mod common;

use alloy::primitives::Address;

use hl_rs::actions::{
    AssetRequest, InsertMarginTable, PerpDexSchema, RegisterAsset, SetFeeRecipient,
    SetFundingMultipliers, SetGrowthModes, SetMarginTableIds, SetOpenInterestCaps, SetOracle,
    SetSubDeployers, SubDeployer, SubDeployerVariant, ToggleTrading,
};
use rust_decimal_macros::dec;

use crate::common::{
    log_action, log_response, send_action, signer_address, test_addresses::test_destination,
    test_dex,
};

// ============================================================================
// RegisterAsset
// ============================================================================

#[tokio::test]
async fn test_register_asset_on_existing_dex() {
    // Register a new asset on an existing DEX
    let asset = AssetRequest::new("NEWCOIN", 4, dec!(1.0), 1);
    let action = RegisterAsset::new(test_dex::PERP_DEX, asset);

    log_action("RegisterAsset (existing dex)", &action);

    assert_eq!(action.dex, test_dex::PERP_DEX);
    assert_eq!(action.asset_request.coin, "NEWCOIN");
    assert!(action.schema.is_none());

    let result = send_action(action).await;
    log_response("RegisterAsset (existing dex)", &result);
}

#[tokio::test]
async fn test_register_asset_on_new_dex() {
    // Register a new asset on a new DEX (with schema)
    let schema = PerpDexSchema::new("Test DEX", "@1"); // @1 = USDC collateral

    let asset = AssetRequest::new("BTC", 4, dec!(50000), 1);
    let action = RegisterAsset::new("testdex123", asset).new_dex(schema);

    log_action("RegisterAsset (new dex)", &action);

    assert_eq!(action.dex, "testdex123");
    assert!(action.schema.is_some());
    let schema = action.schema.as_ref().unwrap();
    assert_eq!(schema.full_name, "Test DEX");
    assert_eq!(schema.collateral_token, "@1");

    let result = send_action(action).await;
    log_response("RegisterAsset (new dex)", &result);
}

#[tokio::test]
async fn test_register_asset_with_oracle_updater() {
    // Register with a custom oracle updater
    let oracle_updater = signer_address();
    let schema = PerpDexSchema::new("Oracle Test DEX", "@1").with_oracle_updater(oracle_updater);

    let asset = AssetRequest::new("ETH", 4, dec!(3000), 1).only_isolated(true);
    let action = RegisterAsset::new("oracledex", asset).new_dex(schema);

    log_action("RegisterAsset (with oracle updater)", &action);

    assert!(action.schema.as_ref().unwrap().oracle_updater.is_some());
    assert!(action.asset_request.only_isolated);

    let result = send_action(action).await;
    log_response("RegisterAsset (with oracle updater)", &result);
}

#[tokio::test]
async fn test_register_asset_with_max_gas() {
    // Register with max gas limit
    let asset = AssetRequest::new("GASCOIN", 2, dec!(0.01), 1);
    let action = RegisterAsset::new(test_dex::PERP_DEX, asset).max_gas(1_000_000);

    log_action("RegisterAsset (with max gas)", &action);

    assert_eq!(action.max_gas, Some(1_000_000));

    let result = send_action(action).await;
    log_response("RegisterAsset (with max gas)", &result);
}

// ============================================================================
// SetOpenInterestCaps
// ============================================================================

#[tokio::test]
async fn test_set_open_interest_caps_new() {
    // Set caps for multiple assets using the new() constructor
    let action = SetOpenInterestCaps::new(
        test_dex::PERP_DEX,
        vec![(test_dex::TEST_COIN, 10 * 1_000_000)], // $10M cap
    );
    log_action("SetOpenInterestCaps (new)", &action);

    assert_eq!(action.caps.len(), 1);

    let result = send_action(action).await;
    log_response("SetOpenInterestCaps (new)", &result);

    // Note: May fail if wallet doesn't have deploy permissions
    assert!(result.is_ok() || format!("{:?}", result).contains("permission"));
}

#[tokio::test]
async fn test_set_open_interest_caps_single() {
    // Set cap for a single asset
    let action = SetOpenInterestCaps::set_single(
        test_dex::PERP_DEX,
        test_dex::TEST_COIN,
        5 * 1_000_000, // $5M cap
    );
    log_action("SetOpenInterestCaps (single)", &action);

    assert_eq!(action.caps.len(), 1);

    let result = send_action(action).await;
    log_response("SetOpenInterestCaps (single)", &result);
}

#[tokio::test]
async fn test_set_open_interest_caps_builder() {
    // Build caps incrementally using the builder pattern
    let action = SetOpenInterestCaps::default()
        .set_cap(test_dex::PERP_DEX, test_dex::TEST_COIN, 10_000_000)
        .set_cap(test_dex::PERP_DEX, "TEST1", 5_000_000);
    log_action("SetOpenInterestCaps (builder)", &action);

    assert_eq!(action.caps.len(), 2);

    let result = send_action(action).await;
    log_response("SetOpenInterestCaps (builder)", &result);
}

// ============================================================================
// ToggleTrading
// ============================================================================

#[tokio::test]
async fn test_toggle_trading_halt() {
    // Halt trading for the test coin
    let action = ToggleTrading::halt(test_dex::PERP_DEX, test_dex::TEST_COIN);
    log_action("ToggleTrading (halt)", &action);

    let result = send_action(action).await;
    log_response("ToggleTrading (halt)", &result);
}

#[tokio::test]
async fn test_toggle_trading_resume() {
    // Resume trading for the test coin
    let action = ToggleTrading::resume(test_dex::PERP_DEX, test_dex::TEST_COIN);
    log_action("ToggleTrading (resume)", &action);

    let result = send_action(action).await;
    log_response("ToggleTrading (resume)", &result);
}

// ============================================================================
// SetOracle
// ============================================================================

#[tokio::test]
async fn test_set_oracle() {
    let action = SetOracle {
        dex: test_dex::PERP_DEX.to_string(),
        oracle_pxs: vec![(test_dex::TEST_COIN.to_string(), "100.0".to_string())],
        mark_pxs: vec![vec![(test_dex::TEST_COIN.to_string(), "100.0".to_string())]],
        external_perp_pxs: vec![],
        nonce: None,
    };

    log_action("SetOracle", &action);

    let result = send_action(action).await;
    log_response("SetOracle", &result);
}

// ============================================================================
// SetFeeRecipient
// ============================================================================

#[tokio::test]
async fn test_set_fee_recipient() {
    let action = SetFeeRecipient::new(test_dex::PERP_DEX, Address::repeat_byte(0x33));
    log_action("SetFeeRecipient", &action);

    let result = send_action(action).await;
    log_response("SetFeeRecipient", &result);
}

// ============================================================================
// SetFundingMultipliers
// ============================================================================

#[tokio::test]
async fn test_set_funding_multipliers() {
    let action = SetFundingMultipliers::new(test_dex::PERP_DEX, vec![(test_dex::TEST_COIN, 1.0)]);
    log_action("SetFundingMultipliers", &action);

    let result = send_action(action).await;
    log_response("SetFundingMultipliers", &result);
}

// ============================================================================
// SetGrowthModes
// ============================================================================

#[tokio::test]
async fn test_set_growth_modes() {
    let action = SetGrowthModes::new(test_dex::PERP_DEX, vec![(test_dex::TEST_COIN, true)]);
    log_action("SetGrowthModes", &action);

    let result = send_action(action).await;
    log_response("SetGrowthModes", &result);
}

// ============================================================================
// SetMarginTableIds
// ============================================================================

#[tokio::test]
async fn test_set_margin_table_ids() {
    let action = SetMarginTableIds::new(test_dex::PERP_DEX, vec![(test_dex::TEST_COIN, 10)]);
    log_action("SetMarginTableIds", &action);

    let result = send_action(action).await;
    log_response("SetMarginTableIds", &result);
}

// ============================================================================
// InsertMarginTable
// ============================================================================

#[tokio::test]
async fn test_insert_margin_table() {
    // Tiers are (lower_bound, max_leverage)
    let action = InsertMarginTable::new(
        test_dex::PERP_DEX,
        "Test margin table",
        vec![
            (0, 50),       // 0 lower bound, 50x max leverage
            (100_000, 25), // 100k lower bound, 25x max leverage
        ],
    );
    log_action("InsertMarginTable", &action);

    let result = send_action(action).await;
    log_response("InsertMarginTable", &result);
}

// ============================================================================
// SetSubDeployers
// ============================================================================

#[tokio::test]
async fn test_set_sub_deployers_enable_permissions() {
    // Use the convenience enable_permissions method
    let action = SetSubDeployers::new(test_dex::PERP_DEX)
        .enable_permissions(test_destination(), vec![SubDeployerVariant::SetOracle]);
    log_action("SetSubDeployers (enable_permissions)", &action);

    assert!(action.sub_deployers.iter().all(|s| s.allowed));

    let result = send_action(action).await;
    log_response("SetSubDeployers (enable_permissions)", &result);
}

#[tokio::test]
async fn test_set_sub_deployers_disable_permissions() {
    // Test revoking permissions
    let action = SetSubDeployers::new(test_dex::PERP_DEX)
        .disable_permissions(test_destination(), vec![SubDeployerVariant::SetOracle]);

    log_action("SetSubDeployers (disable_permissions)", &action);

    assert_eq!(action.sub_deployers.len(), 1);
    assert!(!action.sub_deployers[0].allowed);

    let result = send_action(action).await;
    log_response("SetSubDeployers (disable_permissions)", &result);
}
