/// The Pause Collection settings determine how we will pause collection for this subscription and for how long the subscription
/// should be paused.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionsResourcePauseCollection {
    /// The payment collection behavior for this subscription while paused.
    ///
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: SubscriptionsResourcePauseCollectionBehavior,

    /// The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl SubscriptionsResourcePauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepAsDraft => "keep_as_draft",
            Self::MarkUncollectible => "mark_uncollectible",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for SubscriptionsResourcePauseCollectionBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsResourcePauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SubscriptionsResourcePauseCollectionBehavior {
    fn default() -> Self {
        Self::KeepAsDraft
    }
}
