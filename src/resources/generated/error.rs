/// An error response from the Stripe API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Error {
    pub error: Box<crate::generated::ApiErrors>,
}
