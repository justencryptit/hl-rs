use alloy::primitives::Address;
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::actions::serialization::ser_lowercase;

/// Withdraw USDC from Hyperliquid to an external address via the bridge.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(
    action_type = "withdraw3",
    types = "Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)"
)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct Withdraw {
    /// Destination address for the withdrawal
    #[serde(serialize_with = "ser_lowercase")]
    pub destination: Address,
    /// Amount to withdraw
    pub amount: Decimal,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl Withdraw {
    pub fn builder() -> WithdrawBuilder {
        WithdrawBuilder::default()
    }

    pub fn new(destination: Address, amount: Decimal) -> Self {
        Self {
            destination,
            amount,
            nonce: None,
        }
    }
}
