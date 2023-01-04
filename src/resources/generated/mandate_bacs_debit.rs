#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    ///
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,

    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,

    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl MandateBacsDebitNetworkStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Refused => "refused",
            Self::Revoked => "revoked",
        }
    }
}

impl AsRef<str> for MandateBacsDebitNetworkStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for MandateBacsDebitNetworkStatus {
    fn default() -> Self {
        Self::Accepted
    }
}
