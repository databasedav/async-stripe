#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    /// An universal link that redirect to WeChat Pay app.
    pub native_url: String,
}
