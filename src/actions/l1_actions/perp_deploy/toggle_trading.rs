use hl_rs_derive::L1Action;
use serde::{Deserialize, Serialize};

/// Halt or resume trading for a specific coin on a perp DEX.
///
/// This action allows DEX deployers to temporarily halt trading for a coin
/// (e.g., during maintenance or emergencies) and resume it later.
///
/// # Examples
///
/// Halt trading for a coin:
///
/// ```
/// use hl_rs::actions::ToggleTrading;
///
/// // Halt trading for BTC on the "mydex" DEX
/// let action = ToggleTrading::halt("mydex", "BTC");
/// assert!(action.is_halted);
/// assert_eq!(action.coin, "mydex:BTC");
/// ```
///
/// Resume trading for a coin:
///
/// ```
/// use hl_rs::actions::ToggleTrading;
///
/// // Resume trading for ETH on the "mydex" DEX
/// let action = ToggleTrading::resume("mydex", "ETH");
/// assert!(!action.is_halted);
/// assert_eq!(action.coin, "mydex:ETH");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, L1Action)]
#[action(action_type = "perpDeploy", payload_key = "haltTrading")]
#[serde(rename_all = "camelCase")]
pub struct ToggleTrading {
    /// The coin identifier in the format "dex:SYMBOL" (e.g., "mydex:BTC").
    pub coin: String,
    /// Whether trading is halted (`true`) or active (`false`).
    pub is_halted: bool,
    #[serde(skip_serializing)]
    pub nonce: Option<u64>,
}

impl ToggleTrading {
    /// Halt trading for a coin on the specified DEX.
    ///
    /// This is the preferred method name as it clearly indicates the intent.
    ///
    /// # Arguments
    ///
    /// * `dex` - The DEX name (will be lowercased)
    /// * `coin` - The coin symbol (will be uppercased)
    ///
    /// # Example
    ///
    /// ```
    /// use hl_rs::actions::ToggleTrading;
    ///
    /// let action = ToggleTrading::halt("mydex", "BTC");
    /// assert!(action.is_halted);
    /// ```
    pub fn halt(dex: impl Into<String>, coin: impl Into<String>) -> Self {
        Self {
            coin: format!(
                "{}:{}",
                dex.into().to_lowercase(),
                coin.into().to_uppercase()
            ),
            is_halted: true,
            nonce: None,
        }
    }

    /// Resume trading for a coin on the specified DEX.
    ///
    /// This is the preferred method name as it clearly indicates the intent.
    ///
    /// # Arguments
    ///
    /// * `dex` - The DEX name (will be lowercased)
    /// * `coin` - The coin symbol (will be uppercased)
    ///
    /// # Example
    ///
    /// ```
    /// use hl_rs::actions::ToggleTrading;
    ///
    /// let action = ToggleTrading::resume("mydex", "ETH");
    /// assert!(!action.is_halted);
    /// ```
    pub fn resume(dex: impl Into<String>, coin: impl Into<String>) -> Self {
        Self {
            coin: format!(
                "{}:{}",
                dex.into().to_lowercase(),
                coin.into().to_uppercase()
            ),
            is_halted: false,
            nonce: None,
        }
    }
}
