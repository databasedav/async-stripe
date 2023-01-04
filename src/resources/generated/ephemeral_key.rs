#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct EphemeralKey {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Time at which the key will expire.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The key's secret.
    ///
    /// You can use this value to make authorized requests to the Stripe API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostEphemeralKeysParams {
    /// The ID of the Customer you'd like to modify using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The ID of the Issuing Card you'd like to access using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,
}

pub fn post_ephemeral_keys(
    client: &crate::Client,
    params: PostEphemeralKeysParams,
) -> crate::Response<crate::generated::EphemeralKey> {
    client.post_form("/ephemeral_keys", params)
}

pub fn delete_ephemeral_keys_key(
    client: &crate::Client,
    key: String,
) -> crate::Response<crate::generated::EphemeralKey> {
    client.delete(&format!("/ephemeral_keys/{key}", key = key))
}
