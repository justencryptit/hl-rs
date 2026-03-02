use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Set the deployer's trading fee share for a token.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "spotDeploy", payload_key = "setDeployerTradingFeeShare")]
#[serde(rename_all = "camelCase")]
pub struct SetDeployerFees {
    /// Token index
    pub token: u32,
    /// Fee share (as a decimal string)
    pub share: String,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl SetDeployerFees {
    pub fn new(token: u32, share: impl Into<String>) -> Self {
        Self {
            token,
            share: share.into(),
            nonce: None,
        }
    }
}
