/// Account Links are the means by which a Connect platform grants a connected account permission to access
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://stripe.com/docs/connect/connect-onboarding).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The timestamp at which this account link will expire.
    pub expires_at: crate::resources::Scheduled,

    /// The URL for the account link.
    pub url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostAccountLinksParams {
    /// The identifier of the account to create an account link for.
    pub account: String,

    /// Which information the platform needs to collect from the user.
    ///
    /// One of `currently_due` or `eventually_due`.
    /// Default is `currently_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect: Option<PostAccountLinksParamsCollect>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The URL the user will be redirected to if the account link is expired, has been previously-visited, or is otherwise invalid.
    ///
    /// The URL you specify should attempt to generate a new account link with the same parameters used to create the original account link, then redirect the user to the new account link's URL so they can continue with Connect Onboarding.
    /// If a new account link cannot be generated or the redirect fails you should display a useful error to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,

    /// The URL that the user will be redirected to upon leaving or completing the linked flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The type of account link the user is requesting.
    ///
    /// Possible values are `account_onboarding` or `account_update`.
    #[serde(rename = "type")]
    pub type_: PostAccountLinksParamsType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAccountLinksParamsCollect {
    CurrentlyDue,
    EventuallyDue,
}

impl PostAccountLinksParamsCollect {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentlyDue => "currently_due",
            Self::EventuallyDue => "eventually_due",
        }
    }
}

impl AsRef<str> for PostAccountLinksParamsCollect {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAccountLinksParamsCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAccountLinksParamsCollect {
    fn default() -> Self {
        Self::CurrentlyDue
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAccountLinksParamsType {
    AccountOnboarding,
    AccountUpdate,
    CustomAccountUpdate,
    CustomAccountVerification,
}

impl PostAccountLinksParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountOnboarding => "account_onboarding",
            Self::AccountUpdate => "account_update",
            Self::CustomAccountUpdate => "custom_account_update",
            Self::CustomAccountVerification => "custom_account_verification",
        }
    }
}

impl AsRef<str> for PostAccountLinksParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAccountLinksParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAccountLinksParamsType {
    fn default() -> Self {
        Self::AccountOnboarding
    }
}
pub fn post_account_links(
    client: &crate::Client,
    params: PostAccountLinksParams,
) -> crate::Response<crate::generated::AccountLink> {
    client.post_form("/account_links", params)
}
