use alloy::primitives::Address;
use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

use crate::actions::serialization::ser_lowercase;

/// Transfer USD to/from a vault.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    /// Vault address
    #[serde(serialize_with = "ser_lowercase")]
    pub vault_address: Address,
    /// True to deposit to vault, false to withdraw
    pub is_deposit: bool,
    /// USD amount (raw integer, not decimal)
    pub usd: i64,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl VaultTransfer {
    /// Deposit USD to a vault
    pub fn deposit(vault_address: Address, usd: i64) -> Self {
        Self {
            vault_address,
            is_deposit: true,
            usd,
            nonce: None,
        }
    }

    /// Withdraw USD from a vault
    pub fn withdraw(vault_address: Address, usd: i64) -> Self {
        Self {
            vault_address,
            is_deposit: false,
            usd,
            nonce: None,
        }
    }
}
