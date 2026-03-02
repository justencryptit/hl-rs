use crate::flatten_vec;
use hl_rs_derive::L1Action;

/// Set funding rate multipliers for assets in a perp DEX.
///
/// Multipliers must be in the range [0, 10].
/// The tuples are sorted by asset name during serialization.
#[derive(Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "setFundingMultipliers")]
pub struct SetFundingMultipliers {
    /// Vec of (asset, multiplier) tuples
    pub multipliers: Vec<(String, String)>,
    pub nonce: Option<u64>,
}

impl SetFundingMultipliers {
    /// Create a new SetFundingMultipliers action.
    ///
    /// # Arguments
    /// * `dex_name` - Name of the perp DEX
    /// * `multipliers` - Vec of (asset, multiplier) tuples. Multipliers must be in range [0, 10].
    pub fn new(dex_name: impl Into<String>, multipliers: Vec<(impl Into<String>, f64)>) -> Self {
        let dex_name = dex_name.into().to_lowercase();
        Self {
            multipliers: multipliers
                .into_iter()
                .map(|(asset, multiplier)| {
                    (
                        format!("{dex_name}:{}", asset.into().to_uppercase()),
                        multiplier.to_string(),
                    )
                })
                .collect(),
            nonce: None,
        }
    }
}

flatten_vec!(SetFundingMultipliers, multipliers);
