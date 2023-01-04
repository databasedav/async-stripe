#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    ///
    /// Otherwise, this field is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controller: Option<bool>,

    /// The controller type.
    ///
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[serde(rename = "type")]
    pub type_: AccountUnificationAccountControllerType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}

impl AccountUnificationAccountControllerType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Application => "application",
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for AccountUnificationAccountControllerType {
    fn default() -> Self {
        Self::Account
    }
}
