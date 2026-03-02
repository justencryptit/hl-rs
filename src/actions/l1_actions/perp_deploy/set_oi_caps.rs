use crate::flatten_vec;
use hl_rs_derive::L1Action;

/// Set open interest caps for assets on a perp DEX.
///
/// Open interest caps limit the maximum total open interest (sum of all long
/// and short positions) for each asset. This helps manage risk exposure.
///
/// # Examples
///
/// Set caps for multiple assets at once:
///
/// ```
/// use hl_rs::actions::SetOpenInterestCaps;
///
/// let action = SetOpenInterestCaps::new("mydex", vec![
///     ("BTC", 10_000_000),  // $10M cap for BTC
///     ("ETH", 5_000_000),   // $5M cap for ETH
/// ]);
///
/// assert_eq!(action.caps.len(), 2);
/// assert_eq!(action.caps[0].0, "mydex:BTC");
/// ```
///
/// Build caps incrementally using the builder pattern:
///
/// ```
/// use hl_rs::actions::SetOpenInterestCaps;
///
/// let action = SetOpenInterestCaps::default()
///     .set_cap("mydex", "BTC", 10_000_000)
///     .set_cap("mydex", "ETH", 5_000_000);
///
/// assert_eq!(action.caps.len(), 2);
/// ```
///
/// Set a single cap:
///
/// ```
/// use hl_rs::actions::SetOpenInterestCaps;
///
/// let action = SetOpenInterestCaps::set_single("mydex", "SOL", 2_000_000);
/// assert_eq!(action.caps.len(), 1);
/// assert_eq!(action.caps[0], ("mydex:SOL".to_string(), 2_000_000));
/// ```
#[derive(Debug, Clone, L1Action, Default)]
#[action(action_type = "perpDeploy", payload_key = "setOpenInterestCaps")]
pub struct SetOpenInterestCaps {
    /// List of (coin, cap) tuples. Coin format is "dex:SYMBOL".
    pub caps: Vec<(String, u64)>,
    pub nonce: Option<u64>,
}

impl SetOpenInterestCaps {
    /// Create a new `SetOpenInterestCaps` action with multiple caps.
    ///
    /// # Arguments
    ///
    /// * `dex` - The DEX name (will be lowercased)
    /// * `caps` - Vector of (coin_symbol, cap_value) tuples
    ///
    /// # Example
    ///
    /// ```
    /// use hl_rs::actions::SetOpenInterestCaps;
    ///
    /// let action = SetOpenInterestCaps::new("mydex", vec![
    ///     ("BTC", 10_000_000),
    ///     ("ETH", 5_000_000),
    /// ]);
    /// ```
    pub fn new(dex: impl Into<String>, caps: Vec<(impl Into<String>, u64)>) -> Self {
        let dex = dex.into().to_lowercase();
        Self {
            caps: caps
                .into_iter()
                .map(|(coin, cap)| (format!("{}:{}", dex, coin.into().to_uppercase()), cap))
                .collect(),
            nonce: None,
        }
    }

    /// Create a `SetOpenInterestCaps` action for a single asset.
    ///
    /// # Arguments
    ///
    /// * `dex` - The DEX name (will be lowercased)
    /// * `coin` - The coin symbol (will be uppercased)
    /// * `cap` - The open interest cap value
    ///
    /// # Example
    ///
    /// ```
    /// use hl_rs::actions::SetOpenInterestCaps;
    ///
    /// let action = SetOpenInterestCaps::set_single("mydex", "BTC", 10_000_000);
    /// assert_eq!(action.caps[0].1, 10_000_000);
    /// ```
    pub fn set_single(dex: impl Into<String>, coin: impl Into<String>, cap: u64) -> Self {
        Self::default().set_cap(dex, coin, cap)
    }

    /// Add a cap for an asset (builder pattern).
    ///
    /// # Arguments
    ///
    /// * `dex` - The DEX name (will be lowercased)
    /// * `coin` - The coin symbol (will be uppercased)
    /// * `cap` - The open interest cap value
    ///
    /// # Example
    ///
    /// ```
    /// use hl_rs::actions::SetOpenInterestCaps;
    ///
    /// let action = SetOpenInterestCaps::default()
    ///     .set_cap("mydex", "BTC", 10_000_000)
    ///     .set_cap("mydex", "ETH", 5_000_000);
    /// ```
    pub fn set_cap(mut self, dex: impl Into<String>, coin: impl Into<String>, cap: u64) -> Self {
        self.caps.push((
            format!(
                "{}:{}",
                dex.into().to_lowercase(),
                coin.into().to_uppercase()
            ),
            cap,
        ));
        self
    }
}

flatten_vec!(SetOpenInterestCaps, caps);
