#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay_handle_redirect:
        Option<crate::generated::PaymentIntentNextActionAlipayHandleRedirect>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_display_details: Option<crate::generated::PaymentIntentNextActionBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_await_notification:
        Option<crate::generated::PaymentIntentNextActionCardAwaitNotification>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_bank_transfer_instructions:
        Option<crate::generated::PaymentIntentNextActionDisplayBankTransferInstructions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_display_details: Option<crate::generated::PaymentIntentNextActionKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_display_details: Option<crate::generated::PaymentIntentNextActionDisplayOxxoDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_display_qr_code:
        Option<crate::generated::PaymentIntentNextActionPaynowDisplayQrCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_display_qr_code: Option<crate::generated::PaymentIntentNextActionPixDisplayQrCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_display_qr_code:
        Option<crate::generated::PaymentIntentNextActionPromptpayDisplayQrCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<crate::generated::PaymentIntentNextActionRedirectToUrl>,

    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[serde(rename = "type")]
    pub type_: String,

    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<PaymentIntentNextActionUseStripeSdk>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits:
        Option<crate::generated::PaymentIntentNextActionVerifyWithMicrodeposits>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_display_qr_code:
        Option<crate::generated::PaymentIntentNextActionWechatPayDisplayQrCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_android_app:
        Option<crate::generated::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_ios_app:
        Option<crate::generated::PaymentIntentNextActionWechatPayRedirectToIosApp>,
}

/// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
///
/// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionUseStripeSdk {}
