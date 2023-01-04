#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoiceSettingCustomerSetting {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<crate::generated::InvoiceSettingCustomField>>,

    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<Vec<crate::generated::PaymentMethod>>,

    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,

    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<crate::generated::InvoiceSettingRenderingOptions>,
}
