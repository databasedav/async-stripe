#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AlternateStatementDescriptors {
    /// The Kana variation of the descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kana: Option<String>,

    /// The Kanji variation of the descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<String>,
}
