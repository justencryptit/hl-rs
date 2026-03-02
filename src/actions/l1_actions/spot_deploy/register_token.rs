use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Token specification for registration.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenSpec {
    /// Token name/symbol
    pub name: String,
    /// Size decimals
    pub sz_decimals: u32,
    /// Wei decimals
    pub wei_decimals: u32,
}

/// Register a new token.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "spotDeploy", payload_key = "registerToken2")]
#[serde(rename_all = "camelCase")]
pub struct RegisterToken {
    /// Token specification
    pub spec: TokenSpec,
    /// Maximum gas for the operation
    pub max_gas: u64,
    /// Full display name for the token
    pub full_name: String,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl RegisterToken {
    pub fn new(
        name: impl Into<String>,
        sz_decimals: u32,
        wei_decimals: u32,
        max_gas: u64,
        full_name: impl Into<String>,
    ) -> Self {
        Self {
            spec: TokenSpec {
                name: name.into(),
                sz_decimals,
                wei_decimals,
            },
            max_gas,
            full_name: full_name.into(),
            nonce: None,
        }
    }
}
