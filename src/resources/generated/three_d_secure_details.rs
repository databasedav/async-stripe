#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ThreeDSecureDetails {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsAuthenticationFlow>,

    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsResult>,

    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsResultReason>,

    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsVersion>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
}

impl ThreeDSecureDetailsAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Challenge => "challenge",
            Self::Frictionless => "frictionless",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsAuthenticationFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ThreeDSecureDetailsAuthenticationFlow {
    fn default() -> Self {
        Self::Challenge
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}

impl ThreeDSecureDetailsResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AttemptAcknowledged => "attempt_acknowledged",
            Self::Authenticated => "authenticated",
            Self::Exempted => "exempted",
            Self::Failed => "failed",
            Self::NotSupported => "not_supported",
            Self::ProcessingError => "processing_error",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ThreeDSecureDetailsResult {
    fn default() -> Self {
        Self::AttemptAcknowledged
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

impl ThreeDSecureDetailsResultReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abandoned => "abandoned",
            Self::Bypassed => "bypassed",
            Self::Canceled => "canceled",
            Self::CardNotEnrolled => "card_not_enrolled",
            Self::NetworkNotSupported => "network_not_supported",
            Self::ProtocolError => "protocol_error",
            Self::Rejected => "rejected",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsResultReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ThreeDSecureDetailsResultReason {
    fn default() -> Self {
        Self::Abandoned
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
    V2_2_0,
}

impl ThreeDSecureDetailsVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V1_0_2 => "1.0.2",
            Self::V2_1_0 => "2.1.0",
            Self::V2_2_0 => "2.2.0",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ThreeDSecureDetailsVersion {
    fn default() -> Self {
        Self::V1_0_2
    }
}
