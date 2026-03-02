use alloy::primitives::Address;
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::actions::serialization::ser_lowercase;

/// Transfer spot tokens between Hyperliquid accounts.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(
    action_type = "spotSend",
    types = "SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)"
)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct SpotTransfer {
    /// Destination address for the transfer
    #[serde(serialize_with = "ser_lowercase")]
    pub destination: Address,
    /// Token identifier (e.g., "PURR", "HYPE")
    pub token: String,
    /// Amount to transfer
    pub amount: Decimal,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl SpotTransfer {
    pub fn builder() -> SpotTransferBuilder {
        SpotTransferBuilder::default()
    }

    pub fn new(destination: Address, token: impl Into<String>, amount: Decimal) -> Self {
        Self {
            destination,
            token: token.into(),
            amount,
            nonce: None,
        }
    }
}
