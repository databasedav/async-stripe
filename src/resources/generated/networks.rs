#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Networks {
    /// All available networks for the card.
    pub available: Vec<String>,

    /// The preferred network for the card.
    pub preferred: Option<String>,
}
