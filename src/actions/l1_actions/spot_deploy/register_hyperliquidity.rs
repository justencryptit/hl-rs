use hl_rs_derive::L1Action;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Register hyperliquidity for a spot market.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "spotDeploy", payload_key = "registerHyperliquidity")]
#[serde(rename_all = "camelCase")]
pub struct RegisterHyperliquidity {
    /// Spot market index
    pub spot: u32,
    /// Starting price
    pub start_px: Decimal,
    /// Order size
    pub order_sz: Decimal,
    /// Number of orders
    pub n_orders: u32,
    /// Number of seeded price levels (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_seeded_levels: Option<u32>,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl RegisterHyperliquidity {
    pub fn new(spot: u32, start_px: Decimal, order_sz: Decimal, n_orders: u32) -> Self {
        Self {
            spot,
            start_px,
            order_sz,
            n_orders,
            n_seeded_levels: None,
            nonce: None,
        }
    }

    pub fn with_seeded_levels(mut self, n_seeded_levels: u32) -> Self {
        self.n_seeded_levels = Some(n_seeded_levels);
        self
    }
}
