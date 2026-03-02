use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Schedule a cancel-all action at a future time.
///
/// Time must be at least 5 seconds in the future.
/// Maximum 10 scheduled cancels per day.
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCancel {
    /// UTC milliseconds timestamp for cancellation (optional - if None, cancels immediately)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl ScheduleCancel {
    /// Schedule a cancel at a specific time (UTC milliseconds).
    pub fn at(time: u64) -> Self {
        Self {
            time: Some(time),
            nonce: None,
        }
    }

    /// Cancel all orders immediately.
    pub fn now() -> Self {
        Self {
            time: None,
            nonce: None,
        }
    }
}
