use alloy::primitives::Address;
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Approve a builder fee rate for a specific builder address.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(
    types = "ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,address builder,uint64 nonce)"
)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct ApproveBuilderFee {
    /// Maximum fee rate as a decimal string (e.g., "0.001" for 0.1%)
    pub max_fee_rate: Decimal,
    /// Builder address to approve
    pub builder: Address,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl ApproveBuilderFee {
    pub fn builder() -> ApproveBuilderFeeBuilder {
        ApproveBuilderFeeBuilder::default()
    }

    /// Create a new approval with the given fee rate and builder address
    pub fn new(max_fee_rate: Decimal, builder: Address) -> Self {
        Self {
            max_fee_rate,
            builder,
            nonce: None,
        }
    }
}
