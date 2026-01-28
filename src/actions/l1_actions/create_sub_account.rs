use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Create a new sub-account.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccount {
    /// Name for the sub-account
    pub name: String,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl CreateSubAccount {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nonce: None,
        }
    }
}
