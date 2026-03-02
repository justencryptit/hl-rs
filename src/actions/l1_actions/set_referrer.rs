use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Set a referrer code for the user.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    /// The referrer code to set
    pub code: String,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl SetReferrer {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            nonce: None,
        }
    }
}
