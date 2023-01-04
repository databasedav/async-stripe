#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SigmaScheduledQueryRunError {
    /// Information about the run failure.
    pub message: String,
}
