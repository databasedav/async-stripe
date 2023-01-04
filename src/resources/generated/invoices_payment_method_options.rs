#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoicesPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    pub acss_debit: Option<crate::generated::InvoicePaymentMethodOptionsAcssDebit>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    pub bancontact: Option<crate::generated::InvoicePaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    pub card: Option<crate::generated::InvoicePaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    pub customer_balance: Option<crate::generated::InvoicePaymentMethodOptionsCustomerBalance>,

    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    pub konbini: Option<crate::generated::InvoicePaymentMethodOptionsKonbini>,

    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    pub us_bank_account: Option<crate::generated::InvoicePaymentMethodOptionsUsBankAccount>,
}
