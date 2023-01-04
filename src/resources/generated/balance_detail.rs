#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceDetail {
    /// Funds that are available for use.
    pub available: Vec<crate::generated::BalanceAmount>,
}
