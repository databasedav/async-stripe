#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsPaynow {
    /// Reference number associated with this PayNow payment.
    pub reference: Option<String>,
}
