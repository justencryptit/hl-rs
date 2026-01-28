use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transfer USDC between perp and spot accounts.
///
/// Note: This action does not support `expires_after`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(
    types = "UsdClassTransfer(string hyperliquidChain,string amount,bool toPerp,uint64 nonce)"
)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct UsdClassTransfer {
    /// Amount to transfer (can include subaccount suffix)
    pub amount: Decimal,
    /// True to transfer to perp, false to transfer to spot
    pub to_perp: bool,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl UsdClassTransfer {
    pub fn builder() -> UsdClassTransferBuilder {
        UsdClassTransferBuilder::default()
    }

    /// Transfer USDC from spot to perp account
    pub fn to_perp(amount: Decimal) -> Self {
        Self {
            amount,
            to_perp: true,
            nonce: None,
        }
    }

    /// Transfer USDC from perp to spot account
    pub fn to_spot(amount: Decimal) -> Self {
        Self {
            amount,
            to_perp: false,
            nonce: None,
        }
    }
}
