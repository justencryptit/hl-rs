use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Enable DEX abstraction for an agent.
/// This action has no parameters - it's an empty action that just needs to be signed.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
pub struct AgentEnableDexAbstraction {
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl AgentEnableDexAbstraction {
    pub fn new() -> Self {
        Self { nonce: None }
    }
}

impl Default for AgentEnableDexAbstraction {
    fn default() -> Self {
        Self::new()
    }
}
