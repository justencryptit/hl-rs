use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, L1Action, Default)]
#[action(action_type = "noop")]
pub struct NoOp {
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl NoOp {
    pub fn invalidate_nonce(nonce: u64) -> Self {
        Self { nonce: Some(nonce) }
    }
}
