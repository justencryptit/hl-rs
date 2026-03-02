use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Genesis configuration for a token.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "spotDeploy", payload_key = "genesis")]
#[serde(rename_all = "camelCase")]
pub struct Genesis {
    /// Token index
    pub token: u32,
    /// Maximum supply
    pub max_supply: String,
    /// Whether to disable hyperliquidity
    pub no_hyperliquidity: bool,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl Genesis {
    pub fn new(token: u32, max_supply: impl Into<String>, no_hyperliquidity: bool) -> Self {
        Self {
            token,
            max_supply: max_supply.into(),
            no_hyperliquidity,
            nonce: None,
        }
    }
}
