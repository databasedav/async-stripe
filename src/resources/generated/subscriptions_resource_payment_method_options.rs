#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionsResourcePaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<crate::generated::InvoicePaymentMethodOptionsAcssDebit>,

    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<crate::generated::InvoicePaymentMethodOptionsBancontact>,

    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<crate::generated::SubscriptionPaymentMethodOptionsCard>,

    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<crate::generated::InvoicePaymentMethodOptionsCustomerBalance>,

    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<crate::generated::InvoicePaymentMethodOptionsKonbini>,

    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<crate::generated::InvoicePaymentMethodOptionsUsBankAccount>,
}
