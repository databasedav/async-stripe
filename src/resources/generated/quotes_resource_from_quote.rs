#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct QuotesResourceFromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,

    /// The quote that was cloned.
    pub quote: Vec<crate::generated::Quote>,
}
