#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SetupAttemptPaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::SetupAttemptPaymentMethodDetailsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::SetupAttemptPaymentMethodDetailsAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::SetupAttemptPaymentMethodDetailsBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::SetupAttemptPaymentMethodDetailsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::generated::SetupAttemptPaymentMethodDetailsBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::generated::SetupAttemptPaymentMethodDetailsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::SetupAttemptPaymentMethodDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::generated::SetupAttemptPaymentMethodDetailsCardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::SetupAttemptPaymentMethodDetailsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::SetupAttemptPaymentMethodDetailsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::SetupAttemptPaymentMethodDetailsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::SetupAttemptPaymentMethodDetailsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::SetupAttemptPaymentMethodDetailsSofort>,

    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
