#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,

    /// Unique identifier for the object.
    pub id: String,

    /// The predicate to evaluate the payment against.
    pub predicate: String,
}
