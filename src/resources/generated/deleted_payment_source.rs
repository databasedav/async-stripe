/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum DeletedPaymentSource {
    DeletedBankAccount(crate::generated::DeletedBankAccount),
    DeletedCard(crate::generated::DeletedCard),
}
