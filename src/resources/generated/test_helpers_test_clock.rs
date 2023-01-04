/// A test clock enables deterministic control over objects in testmode.
///
/// With a test clock, you can create objects at a frozen time in the past or future, and advance to a specific future time to observe webhooks and state changes.
/// After the clock advances, you can either validate the current state of your scenario (and test your assumptions), change the current state of your scenario (and test more complex scenarios), or keep advancing forward in time.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TestHelpersTestClock {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Time at which this clock is scheduled to auto delete.
    pub deletes_after: crate::params::Timestamp,

    /// Time at which all objects belonging to this clock are frozen.
    pub frozen_time: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The custom name supplied at creation.
    pub name: Option<String>,

    /// The status of the Test Clock.
    pub status: TestHelpersTestClockStatus,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTestHelpersTestClocksTestClockParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTestClocksParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The initial frozen time for this test clock.
    pub frozen_time: crate::params::Timestamp,

    /// The name for this test clock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTestClocksTestClockAdvanceParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The time to advance the test clock.
    ///
    /// Must be after the test clock's current frozen time.
    /// Cannot be more than two intervals in the future from the shortest subscription in this test clock.
    /// If there are no subscriptions in this test clock, it cannot be more than two years in the future.
    pub frozen_time: crate::params::Timestamp,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTestHelpersTestClocksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TestHelpersTestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
}

impl TestHelpersTestClockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advancing => "advancing",
            Self::InternalFailure => "internal_failure",
            Self::Ready => "ready",
        }
    }
}

impl AsRef<str> for TestHelpersTestClockStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TestHelpersTestClockStatus {
    fn default() -> Self {
        Self::Advancing
    }
}
pub fn get_test_helpers_test_clocks_test_clock(
    client: &crate::Client,
    test_clock: String,
    params: GetTestHelpersTestClocksTestClockParams,
) -> crate::Response<crate::generated::TestHelpersTestClock> {
    client.get_query(
        &format!("/test_helpers/test_clocks/{test_clock}", test_clock = test_clock),
        params,
    )
}

pub fn post_test_helpers_test_clocks(
    client: &crate::Client,
    params: PostTestHelpersTestClocksParams,
) -> crate::Response<crate::generated::TestHelpersTestClock> {
    client.post_form("/test_helpers/test_clocks", params)
}

pub fn delete_test_helpers_test_clocks_test_clock(
    client: &crate::Client,
    test_clock: String,
) -> crate::Response<crate::generated::DeletedTestHelpersTestClock> {
    client.delete(&format!("/test_helpers/test_clocks/{test_clock}", test_clock = test_clock))
}

pub fn post_test_helpers_test_clocks_test_clock_advance(
    client: &crate::Client,
    test_clock: String,
    params: PostTestHelpersTestClocksTestClockAdvanceParams,
) -> crate::Response<crate::generated::TestHelpersTestClock> {
    client.post_form(
        &format!("/test_helpers/test_clocks/{test_clock}/advance", test_clock = test_clock),
        params,
    )
}

pub fn get_test_helpers_test_clocks(
    client: &crate::Client,
    params: GetTestHelpersTestClocksParams,
) -> crate::Response<crate::params::List<crate::generated::TestHelpersTestClock>> {
    client.get_query("/test_helpers/test_clocks", params)
}
