use hl_rs_derive::L1Action;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "setOracle")]
#[serde(rename_all = "camelCase")]
pub struct SetOracle {
    pub dex: String,
    pub oracle_pxs: Vec<(String, String)>,
    pub mark_pxs: Vec<Vec<(String, String)>>,
    pub external_perp_pxs: Vec<(String, String)>,
    pub nonce: Option<u64>,
}

impl Serialize for SetOracle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut oracle_pxs = self.oracle_pxs.clone();
        oracle_pxs.sort_by(|a, b| a.0.cmp(&b.0));

        let mut mark_pxs = self.mark_pxs.clone();
        for inner in &mut mark_pxs {
            inner.sort_by(|a, b| a.0.cmp(&b.0));
        }

        let mut external_perp_pxs = self.external_perp_pxs.clone();
        external_perp_pxs.sort_by(|a, b| a.0.cmp(&b.0));

        let mut state = serializer.serialize_struct("SetOracle", 5)?;
        state.serialize_field("dex", &self.dex)?;
        state.serialize_field("oraclePxs", &oracle_pxs)?;
        state.serialize_field("markPxs", &mark_pxs)?;
        state.serialize_field("externalPerpPxs", &external_perp_pxs)?;
        state.end()
    }
}
