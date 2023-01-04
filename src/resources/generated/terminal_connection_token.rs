/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet Management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    ///
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConnectionTokensParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The id of the location that this connection token is scoped to.
    ///
    /// If specified the connection token will only be usable with readers assigned to that location, otherwise the connection token will be usable with all readers.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

pub fn post_terminal_connection_tokens(
    client: &crate::Client,
    params: PostTerminalConnectionTokensParams,
) -> crate::Response<crate::generated::TerminalConnectionToken> {
    client.post_form("/terminal/connection_tokens", params)
}
