use crate::flatten_vec;
use hl_rs_derive::L1Action;

/// Set margin table IDs for assets in a perp DEX.
///
/// References margin tables previously inserted via InsertMarginTable.
/// The tuples are sorted by asset name during serialization.
#[derive(Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "setMarginTableIds")]
pub struct SetMarginTableIds {
    /// Vec of (asset, margin_table_id) tuples
    pub ids: Vec<(String, i64)>,
    pub nonce: Option<u64>,
}

impl SetMarginTableIds {
    /// Create a new SetMarginTableIds action.
    ///
    /// # Arguments
    /// * `dex_name` - Name of the perp DEX
    /// * `ids` - Vec of (asset, margin_table_id) tuples
    pub fn new(dex_name: impl Into<String>, ids: Vec<(impl Into<String>, i64)>) -> Self {
        let dex_name = dex_name.into().to_lowercase();
        Self {
            ids: ids
                .into_iter()
                .map(|(asset, id)| (format!("{dex_name}:{}", asset.into().to_uppercase()), id))
                .collect(),
            nonce: None,
        }
    }
}

flatten_vec!(SetMarginTableIds, ids);
