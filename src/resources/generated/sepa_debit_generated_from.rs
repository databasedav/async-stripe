#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SepaDebitGeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<Vec<crate::generated::Charge>>,

    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<Vec<crate::generated::SetupAttempt>>,
}
