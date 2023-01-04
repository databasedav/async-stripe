#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct MandateOptionsOffSessionDetailsBlik {
    /// Amount of each recurring payment.
    pub amount: Option<i64>,

    /// Currency of each recurring payment.
    pub currency: Option<crate::currency::Currency>,

    /// Frequency interval of each recurring payment.
    pub interval: Option<MandateOptionsOffSessionDetailsBlikInterval>,

    /// Frequency indicator of each recurring payment.
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateOptionsOffSessionDetailsBlikInterval {
    Day,
    Month,
    Week,
    Year,
}

impl MandateOptionsOffSessionDetailsBlikInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for MandateOptionsOffSessionDetailsBlikInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsOffSessionDetailsBlikInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for MandateOptionsOffSessionDetailsBlikInterval {
    fn default() -> Self {
        Self::Day
    }
}
