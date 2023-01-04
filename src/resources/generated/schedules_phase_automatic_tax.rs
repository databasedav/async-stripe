#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SchedulesPhaseAutomaticTax {
    /// Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
}
