#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsKonbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<crate::generated::PaymentMethodDetailsKonbiniStore>,
}
