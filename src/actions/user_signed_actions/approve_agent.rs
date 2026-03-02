use alloy::primitives::Address;
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use serde::{Deserialize, Serialize};

/// Approve an agent to act on behalf of the user.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(
    types = "ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)"
)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct ApproveAgent {
    /// Agent's Ethereum address
    pub agent_address: Address,
    /// Optional agent identifier/name (can be empty or null)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl ApproveAgent {
    pub fn builder() -> ApproveAgentBuilder {
        ApproveAgentBuilder::default()
    }

    /// Approve an agent with a name
    pub fn new(agent_address: Address, agent_name: impl Into<String>) -> Self {
        Self {
            agent_address,
            agent_name: Some(agent_name.into()),
            nonce: None,
        }
    }

    /// Approve an agent without a name
    pub fn without_name(agent_address: Address) -> Self {
        Self {
            agent_address,
            agent_name: None,
            nonce: None,
        }
    }
}
