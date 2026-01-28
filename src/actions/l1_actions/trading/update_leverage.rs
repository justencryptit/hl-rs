use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Update leverage for an asset.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    /// Asset index
    pub asset: u32,
    /// True for cross margin, false for isolated
    pub is_cross: bool,
    /// Leverage multiplier
    pub leverage: u32,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl UpdateLeverage {
    pub fn cross(asset: u32, leverage: u32) -> Self {
        Self {
            asset,
            is_cross: true,
            leverage,
            nonce: None,
        }
    }

    pub fn isolated(asset: u32, leverage: u32) -> Self {
        Self {
            asset,
            is_cross: false,
            leverage,
            nonce: None,
        }
    }
}
