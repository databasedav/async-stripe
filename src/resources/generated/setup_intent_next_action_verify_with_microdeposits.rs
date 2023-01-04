#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: crate::params::Timestamp,

    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,

    /// The type of the microdeposit sent to the customer.
    ///
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}

impl SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amounts => "amounts",
            Self::DescriptorCode => "descriptor_code",
        }
    }
}

impl AsRef<str> for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn default() -> Self {
        Self::Amounts
    }
}
