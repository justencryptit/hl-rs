use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Order type for limit orders with time-in-force.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderType {
    pub tif: Tif,
}

/// Time-in-force options.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tif {
    /// Good-til-cancelled
    Gtc,
    /// Immediate-or-cancel
    Ioc,
    /// All-or-nothing (fill completely or cancel)
    Alo,
    /// Front-running protection
    FrontendMarket,
}

/// Trigger order type for stop-loss/take-profit.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderType {
    pub trigger_px: String,
    pub is_market: bool,
    pub tpsl: TpSl,
}

/// Take-profit or stop-loss indicator.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TpSl {
    Tp,
    Sl,
}

/// Order type (limit or trigger).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OrderType {
    Limit(LimitOrderType),
    Trigger(TriggerOrderType),
}

/// Wire format for a single order.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderWire {
    /// Asset index
    pub a: u32,
    /// Is buy order
    pub b: bool,
    /// Limit price as string
    pub p: String,
    /// Size as string
    pub s: String,
    /// Reduce only
    pub r: bool,
    /// Order type
    pub t: OrderType,
    /// Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
}

/// Builder info for order attribution.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuilderInfo {
    /// Builder address (lowercase)
    pub b: String,
    /// Builder fee in basis points
    pub f: u32,
}

/// Order grouping options.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Grouping {
    #[default]
    Na,
    /// Vault orders
    NormalTpsl,
    /// Position TP/SL
    PositionTpsl,
}

/// Batch create orders action.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "order", payload_key = "orders")]
#[serde(rename_all = "camelCase")]
pub struct BatchOrder {
    /// Order wires
    pub orders: Vec<OrderWire>,
    /// Grouping type
    pub grouping: Grouping,
    /// Builder info (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl BatchOrder {
    pub fn new(orders: Vec<OrderWire>) -> Self {
        Self {
            orders,
            grouping: Grouping::Na,
            builder: None,
            nonce: None,
        }
    }

    pub fn with_grouping(mut self, grouping: Grouping) -> Self {
        self.grouping = grouping;
        self
    }

    pub fn with_builder(mut self, builder: BuilderInfo) -> Self {
        self.builder = Some(builder);
        self
    }
}
