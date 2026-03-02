use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Register a spot market pair.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "spotDeploy", payload_key = "registerSpot")]
#[serde(rename_all = "camelCase")]
pub struct RegisterSpot {
    /// Token pair [base_token, quote_token]
    pub tokens: [u32; 2],
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl RegisterSpot {
    pub fn new(base_token: u32, quote_token: u32) -> Self {
        Self {
            tokens: [base_token, quote_token],
            nonce: None,
        }
    }
}
