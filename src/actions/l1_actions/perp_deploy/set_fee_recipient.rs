use crate::actions::serialization::ser_lowercase;
use alloy::primitives::Address;
use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "setFeeRecipient")]
#[serde(rename_all = "camelCase")]
pub struct SetFeeRecipient {
    pub dex: String,
    #[serde(serialize_with = "ser_lowercase")]
    pub fee_recipient: Address,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl SetFeeRecipient {
    pub fn new(dex: impl Into<String>, fee_recipient: Address) -> Self {
        Self {
            dex: dex.into(),
            fee_recipient,
            nonce: None,
        }
    }
}
