#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CouponCurrencyOption {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: i64,
}
