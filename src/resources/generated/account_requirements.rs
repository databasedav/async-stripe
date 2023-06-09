#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AccountRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<crate::generated::AccountRequirementsAlternative>>,

    /// Date by which the fields in `currently_due` must be collected to keep the account enabled.
    ///
    /// These fields may disable the account sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<crate::params::Timestamp>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,

    /// If the account is disabled, this string describes why.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.
    pub disabled_reason: Option<String>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<crate::generated::AccountRequirementsError>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Option<Vec<String>>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the account.
    pub past_due: Option<Vec<String>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Option<Vec<String>>,
}