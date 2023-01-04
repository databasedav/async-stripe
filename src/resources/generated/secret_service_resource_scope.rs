#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SecretServiceResourceScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: SecretServiceResourceScopeType,

    /// The user ID, if type is set to "user".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
}

impl SecretServiceResourceScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for SecretServiceResourceScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SecretServiceResourceScopeType {
    fn default() -> Self {
        Self::Account
    }
}
