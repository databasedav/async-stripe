#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoiceInstallmentsCard {
    /// Whether Installments are enabled for this Invoice.
    pub enabled: Option<bool>,
}
