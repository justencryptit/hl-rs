use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Insert a new margin table for a perp DEX.
///
/// Margin tables define leverage tiers with lower bounds and max leverage.
/// Max 3 tiers per table.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "insertMarginTable")]
#[serde(rename_all = "camelCase")]
pub struct InsertMarginTable {
    pub dex: String,
    pub margin_table: MarginTable,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

/// A margin table with description and tiers.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginTable {
    pub description: String,
    pub margin_tiers: Vec<MarginTier>,
}

/// A single margin tier defining leverage at a position size threshold.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginTier {
    /// Position size lower bound for this tier
    pub lower_bound: i64,
    /// Maximum leverage allowed at this tier
    pub max_leverage: i64,
}

impl InsertMarginTable {
    /// Create a new InsertMarginTable action.
    ///
    /// # Arguments
    /// * `dex` - Name of the perp DEX
    /// * `description` - Description of the margin table
    /// * `tiers` - Vec of (lower_bound, max_leverage) tuples. Max 3 tiers.
    pub fn new(
        dex: impl Into<String>,
        description: impl Into<String>,
        tiers: Vec<(i64, i64)>,
    ) -> Self {
        Self {
            dex: dex.into(),
            margin_table: MarginTable {
                description: description.into(),
                margin_tiers: tiers
                    .into_iter()
                    .map(|(lower_bound, max_leverage)| MarginTier {
                        lower_bound,
                        max_leverage,
                    })
                    .collect(),
            },
            nonce: None,
        }
    }
}
