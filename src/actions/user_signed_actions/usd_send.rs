use crate::actions::serialization::ser_lowercase;
use alloy::primitives::Address;
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// USDC transfer between Hyperliquid accounts (UserSignedAction example)
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(types = "UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)")]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct UsdSend {
    #[serde(serialize_with = "ser_lowercase")]
    pub destination: Address,
    pub amount: Decimal,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl UsdSend {
    pub fn builder() -> UsdSendBuilder {
        UsdSendBuilder::default()
    }

    pub fn new(destination: Address, amount: Decimal) -> Self {
        Self {
            destination,
            amount,
            nonce: None,
        }
    }
}
