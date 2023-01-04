#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoiceSettingRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
}
