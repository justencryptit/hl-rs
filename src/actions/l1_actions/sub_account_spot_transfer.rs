use alloy::primitives::Address;
use hl_rs_derive::L1Action;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::actions::serialization::ser_lowercase;

/// Transfer spot tokens between main account and sub-account.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountSpotTransfer {
    /// Sub-account user address
    #[serde(serialize_with = "ser_lowercase")]
    pub sub_account_user: Address,
    /// True to deposit to sub-account, false to withdraw
    pub is_deposit: bool,
    /// Token identifier
    pub token: String,
    /// Amount to transfer
    pub amount: Decimal,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl SubAccountSpotTransfer {
    /// Deposit tokens to a sub-account
    pub fn deposit(sub_account_user: Address, token: impl Into<String>, amount: Decimal) -> Self {
        Self {
            sub_account_user,
            is_deposit: true,
            token: token.into(),
            amount,
            nonce: None,
        }
    }

    /// Withdraw tokens from a sub-account
    pub fn withdraw(sub_account_user: Address, token: impl Into<String>, amount: Decimal) -> Self {
        Self {
            sub_account_user,
            is_deposit: false,
            token: token.into(),
            amount,
            nonce: None,
        }
    }
}
