use alloy::{dyn_abi::DynSolType, primitives::Address};
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use serde::{Deserialize, Serialize};

use crate::{abi_value::AbiResult, Error, ToAbiValue};

/// Convert an account to a multi-signature user.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(types = "ConvertToMultiSigUser(string hyperliquidChain,string signers,uint64 nonce)")]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct ConvertToMultiSigUser {
    /// JSON-encoded signers configuration containing `authorizedUsers` and `threshold`
    pub signers: MultiSigSigners,
    #[builder(default)]
    pub nonce: Option<u64>,
}

/// Configuration for multi-sig signers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigSigners {
    /// List of authorized user addresses (must be sorted)
    pub authorized_users: Vec<Address>,
    /// Number of signatures required
    pub threshold: u32,
}

impl ConvertToMultiSigUser {
    pub fn builder() -> ConvertToMultiSigUserBuilder {
        ConvertToMultiSigUserBuilder::default()
    }

    /// Create from a list of authorized users and threshold
    pub fn new(mut authorized_users: Vec<Address>, threshold: u32) -> Self {
        // Sort addresses for deterministic serialization
        authorized_users.sort();

        let signers = MultiSigSigners {
            authorized_users,
            threshold,
        };

        Self {
            signers,
            nonce: None,
        }
    }
}

impl ToAbiValue for MultiSigSigners {
    fn to_abi_value(&self, abi_type: &DynSolType) -> AbiResult {
        let signers = serde_json::to_string(&self).map_err(|e| {
            Error::SerializationFailure(format!("Failed to serialize MultiSigSigners: {}", e))
        })?;
        signers.to_abi_value(abi_type)
    }
}
