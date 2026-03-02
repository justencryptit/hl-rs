use derive_builder::Builder;
use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Toggle big blocks mode for EVM user (L1 Action example)
#[derive(Debug, Clone, Serialize, Deserialize, Builder, L1Action)]
#[action(action_type = "evmUserModify")]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct ToggleBigBlocks {
    pub using_big_blocks: bool,
    #[serde(skip_serializing)]
    #[builder(default)]
    pub nonce: Option<u64>,
}

impl ToggleBigBlocks {
    pub fn builder() -> ToggleBigBlocksBuilder {
        ToggleBigBlocksBuilder::default()
    }

    pub fn enable() -> Self {
        Self {
            using_big_blocks: true,
            nonce: None,
        }
    }

    pub fn disable() -> Self {
        Self {
            using_big_blocks: false,
            nonce: None,
        }
    }
}
