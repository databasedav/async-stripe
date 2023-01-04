#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ShippingRateCurrencyOption {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: ShippingRateCurrencyOptionTaxBehavior,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateCurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl ShippingRateCurrencyOptionTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for ShippingRateCurrencyOptionTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateCurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ShippingRateCurrencyOptionTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
