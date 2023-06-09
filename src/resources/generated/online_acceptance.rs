#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OnlineAcceptance {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,

    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}