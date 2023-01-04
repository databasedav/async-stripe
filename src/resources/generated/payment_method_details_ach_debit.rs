#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsAchDebit {
    /// Type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodDetailsAchDebitAccountHolderType>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Routing transit number of the bank account.
    pub routing_number: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsAchDebitAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodDetailsAchDebitAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsAchDebitAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsAchDebitAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodDetailsAchDebitAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}
