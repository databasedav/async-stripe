/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FinancialConnectionsAccountOwnership {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// A paginated list of owners for this account.
    pub owners: crate::params::List<crate::generated::FinancialConnectionsAccountOwner>,
}
