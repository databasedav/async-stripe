#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalFeatures {
    pub customer_update: crate::generated::PortalCustomerUpdate,

    pub invoice_history: crate::generated::PortalInvoiceList,

    pub payment_method_update: crate::generated::PortalPaymentMethodUpdate,

    pub subscription_cancel: crate::generated::PortalSubscriptionCancel,

    pub subscription_pause: crate::generated::PortalSubscriptionPause,

    pub subscription_update: crate::generated::PortalSubscriptionUpdate,
}
