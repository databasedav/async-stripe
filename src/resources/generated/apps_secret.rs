/// Secret Store is an API that allows Stripe Apps developers to securely persist secrets for use by UI Extensions and app backends.
///
/// The primary resource in Secret Store is a `secret`.
///
/// Other apps can't view secrets created by an app.
/// Additionally, secrets are scoped to provide further permission control.  All Dashboard users and the app backend share `account` scoped secrets.
/// Use the `account` scope for secrets that don't change per-user, like a third-party API key.  A `user` scoped secret is accessible by the app backend and one specific Dashboard user.
/// Use the `user` scope for per-user secrets like per-user OAuth tokens, where different users might have different permissions.  Related guide: [Store data between page reloads](https://stripe.com/docs/stripe-apps/store-auth-data-custom-objects).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AppsSecret {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// If true, indicates that this secret has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<crate::resources::Scheduled>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A name for the secret that's unique within the scope.
    pub name: String,

    /// The plaintext secret value to be stored.
    pub payload: Option<String>,

    pub scope: crate::generated::SecretServiceResourceScope,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetAppsSecretsFindParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub name: String,

    pub scope: GetAppsSecretsFindParamsScope,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostAppsSecretsParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,

    /// A name for the secret that's unique within the scope.
    pub name: String,

    /// The plaintext secret value to be stored.
    pub payload: String,

    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: PostAppsSecretsParamsScope,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostAppsSecretsDeleteParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A name for the secret that's unique within the scope.
    pub name: String,

    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: PostAppsSecretsDeleteParamsScope,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetAppsSecretsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    pub scope: GetAppsSecretsParamsScope,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetAppsSecretsFindParamsScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: GetAppsSecretsFindParamsScopeType,

    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Specifies the scoping of the secret.
///
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostAppsSecretsParamsScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: PostAppsSecretsParamsScopeType,

    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Specifies the scoping of the secret.
///
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostAppsSecretsDeleteParamsScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: PostAppsSecretsDeleteParamsScopeType,

    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetAppsSecretsParamsScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: GetAppsSecretsParamsScopeType,

    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetAppsSecretsFindParamsScopeType {
    Account,
    User,
}

impl GetAppsSecretsFindParamsScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for GetAppsSecretsFindParamsScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetAppsSecretsFindParamsScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetAppsSecretsFindParamsScopeType {
    fn default() -> Self {
        Self::Account
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAppsSecretsParamsScopeType {
    Account,
    User,
}

impl PostAppsSecretsParamsScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for PostAppsSecretsParamsScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAppsSecretsParamsScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAppsSecretsParamsScopeType {
    fn default() -> Self {
        Self::Account
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAppsSecretsDeleteParamsScopeType {
    Account,
    User,
}

impl PostAppsSecretsDeleteParamsScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for PostAppsSecretsDeleteParamsScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAppsSecretsDeleteParamsScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAppsSecretsDeleteParamsScopeType {
    fn default() -> Self {
        Self::Account
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetAppsSecretsParamsScopeType {
    Account,
    User,
}

impl GetAppsSecretsParamsScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for GetAppsSecretsParamsScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetAppsSecretsParamsScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetAppsSecretsParamsScopeType {
    fn default() -> Self {
        Self::Account
    }
}
pub fn get_apps_secrets_find(
    client: &crate::Client,
    params: GetAppsSecretsFindParams,
) -> crate::Response<crate::generated::AppsSecret> {
    client.get_query("/apps/secrets/find", params)
}

pub fn post_apps_secrets(
    client: &crate::Client,
    params: PostAppsSecretsParams,
) -> crate::Response<crate::generated::AppsSecret> {
    client.post_form("/apps/secrets", params)
}

pub fn post_apps_secrets_delete(
    client: &crate::Client,
    params: PostAppsSecretsDeleteParams,
) -> crate::Response<crate::generated::AppsSecret> {
    client.post_form("/apps/secrets/delete", params)
}

pub fn get_apps_secrets(
    client: &crate::Client,
    params: GetAppsSecretsParams,
) -> crate::Response<crate::params::List<crate::generated::AppsSecret>> {
    client.get_query("/apps/secrets", params)
}
