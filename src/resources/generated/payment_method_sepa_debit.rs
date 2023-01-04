#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Information about the object that generated this PaymentMethod.
    pub generated_from: Option<crate::generated::SepaDebitGeneratedFrom>,

    /// Last four characters of the IBAN.
    pub last4: Option<String>,
}
