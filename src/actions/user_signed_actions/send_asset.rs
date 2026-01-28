use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    primitives::{keccak256, Address},
};
use derive_builder::Builder;
use hl_rs_derive::UserSignedAction;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{abi_value::AbiResult, actions::serialization::ser_lowercase, Error, ToAbiValue};

/// Send assets between DEXes on Hyperliquid.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, UserSignedAction)]
#[action(
    types = "SendAsset(string hyperliquidChain,string destination,string sourceDex,string destinationDex,string token,string amount,string fromSubAccount,uint64 nonce)"
)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct SendAsset {
    /// Destination address
    #[serde(serialize_with = "ser_lowercase")]
    pub destination: Address,
    /// Source DEX identifier
    pub source_dex: DexId,
    /// Destination DEX identifier
    pub destination_dex: DexId,
    /// Token to send
    pub token: String,
    /// Amount to send
    pub amount: Decimal,
    /// Source subaccount (can be empty string for main account)
    pub from_sub_account: String,
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl SendAsset {
    pub fn builder() -> SendAssetBuilder {
        SendAssetBuilder::default()
    }

    pub fn new(
        destination: Address,
        source_dex: DexId,
        destination_dex: DexId,
        token: impl Into<String>,
        amount: Decimal,
    ) -> Self {
        Self {
            destination,
            source_dex,
            destination_dex,
            token: token.into(),
            amount,
            from_sub_account: String::new(),
            nonce: None,
        }
    }

    /// Set the source subaccount
    pub fn from_sub_account(mut self, sub_account: impl Into<String>) -> Self {
        self.from_sub_account = sub_account.into();
        self
    }
}

/// DEX identifier for asset transfers.
///
/// # String representation
/// - `Spot` -> `"spot"`
/// - `Perp` -> `""` (empty string)
/// - `Hip3(name)` -> lowercase name
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DexId {
    Spot,
    Perp,
    Hip3(String),
}

impl DexId {
    /// Returns the string representation used for serialization and hashing.
    pub fn as_str(&self) -> &str {
        match self {
            DexId::Spot => "spot",
            DexId::Perp => "",
            DexId::Hip3(dex) => dex.as_str(),
        }
    }
}

impl Serialize for DexId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DexId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(match s.to_lowercase().as_str() {
            "spot" => DexId::Spot,
            "" => DexId::Perp,
            other => DexId::Hip3(other.to_owned()),
        })
    }
}

impl ToAbiValue for DexId {
    fn to_abi_value(&self, abi_type: &DynSolType) -> AbiResult {
        if abi_type != &DynSolType::String {
            return Err(Error::AbiEncode {
                rust_type: "DexId",
                abi_type: format!("{:?}", abi_type),
            });
        }

        Ok(DynSolValue::FixedBytes(
            keccak256(self.as_str().as_bytes()),
            32,
        ))
    }
}
