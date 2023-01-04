#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<
        crate::generated::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer,
    >,

    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
