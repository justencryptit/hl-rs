use crate::flatten_vec;
use hl_rs_derive::L1Action;

/// Set growth modes for assets in a perp DEX.
///
/// Growth mode status can only be changed once every 30 days.
/// The tuples are sorted by asset name during serialization.
#[derive(Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "setGrowthModes")]
pub struct SetGrowthModes {
    /// Vec of (asset, is_growth_mode_enabled) tuples
    pub modes: Vec<(String, bool)>,
    pub nonce: Option<u64>,
}

impl SetGrowthModes {
    /// Create a new SetGrowthModes action.
    ///
    /// # Arguments
    /// * `dex_name` - Name of the perp DEX
    /// * `modes` - Vec of (asset, is_enabled) tuples
    pub fn new(dex_name: impl Into<String>, modes: Vec<(impl Into<String>, bool)>) -> Self {
        let dex_name = dex_name.into().to_lowercase();
        Self {
            modes: modes
                .into_iter()
                .map(|(asset, enabled)| {
                    (
                        format!("{dex_name}:{}", asset.into().to_uppercase()),
                        enabled,
                    )
                })
                .collect(),
            nonce: None,
        }
    }
}

flatten_vec!(SetGrowthModes, modes);
