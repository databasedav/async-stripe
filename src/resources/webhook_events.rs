use chrono::Utc;
#[cfg(feature = "webhook-events")]
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
#[cfg(feature = "webhook-events")]
use sha2::Sha256;

use crate::error::WebhookError;
use crate::generated::webhook_endpoint::PostWebhookEndpointsParamsEnabledEvents;
use crate::generated::{
    Account, AccountCapabilities, Application, ApplicationFee, Balance, BankAccount,
    BillingPortalConfiguration, Card, Charge, CheckoutSession, Coupon, Customer, Discount, Dispute,
    FeeRefund, File, Invoice, Invoiceitem, IssuingAuthorization, IssuingCard, IssuingCardholder,
    IssuingDispute, IssuingTransaction, Mandate, NotificationEventRequest, Order, PaymentIntent,
    PaymentLink, PaymentMethod, Payout, Person, Plan, Price, Product, PromotionCode, Quote, Refund,
    Review, SetupIntent, Sku, Subscription, SubscriptionSchedule, TaxId, TaxRate,
    TestHelpersTestClock, Topup, Transfer,
};
use crate::ids::EventId;
use crate::{AccountId, Timestamp};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookEvent {
    /// Unique identifier for the object.
    pub id: EventId,

    /// Description of the event (e.g., `invoice.created` or `charge.refunded`).
    #[serde(rename = "type")]
    // TODO: use deduplicated EventType instead
    pub event_type: PostWebhookEndpointsParamsEnabledEvents,

    /// The connected account that originated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountId>,

    /// The Stripe API version used to render `data`.
    ///
    /// *Note: This property is populated only for events on or after October 31, 2014*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    pub data: EventData,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Number of webhooks that have yet to be successfully delivered (i.e., to return a 20x response) to the URLs you've specified.
    pub pending_webhooks: i64,

    /// Information on the API request that instigated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<NotificationEventRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventData {
    pub object: EventObject,
    // previous_attributes: ...
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum EventObject {
    Account(Account),
    #[serde(rename = "capability")]
    AccountCapabilities(AccountCapabilities),
    Application(Application),
    ApplicationFee(ApplicationFee),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(FeeRefund),
    Balance(Balance),
    BankAccount(BankAccount),
    #[serde(rename = "billing_portal.configuration")]
    BillingPortalConfiguration(BillingPortalConfiguration),
    Card(Card),
    Charge(Charge),
    #[serde(rename = "checkout.session")]
    CheckoutSession(CheckoutSession),
    Coupon(Coupon),
    Customer(Customer),
    Discount(Discount),
    Dispute(Dispute),
    File(File),
    Invoice(Invoice),
    #[serde(rename = "invoiceitem")]
    InvoiceItem(Invoiceitem),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(IssuingAuthorization),
    #[serde(rename = "issuing.card")]
    IssuingCard(IssuingCard),
    #[serde(rename = "issuing.cardholder")]
    IssuingCardholder(IssuingCardholder),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(IssuingTransaction),
    Mandate(Mandate),
    Order(Order),
    //The order return has currently disappeared from the
    //openAPI.  I believe this may be an error, but for now
    //we will disable it here
    //OrderReturn(OrderReturn),
    PaymentIntent(PaymentIntent),
    PaymentLink(PaymentLink),
    PaymentMethod(PaymentMethod),
    Payout(Payout),
    Person(Person),
    Plan(Plan),
    Price(Price),
    Product(Product),
    PromotionCode(PromotionCode),
    Quote(Quote),
    Refund(Refund),
    Review(Review),
    SetupIntent(SetupIntent),
    Sku(Sku),
    Subscription(Subscription),
    SubscriptionSchedule(SubscriptionSchedule),
    TaxId(TaxId),
    TaxRate(TaxRate),
    #[serde(rename = "test_helpers.test_clock")]
    TestHelpersTestClock(TestHelpersTestClock),
    Topup(Topup),
    Transfer(Transfer),
}

#[cfg(feature = "webhook-events")]
pub struct Webhook {
    current_timestamp: i64,
}

#[cfg(feature = "webhook-events")]
impl Webhook {
    /// # Errors
    ///
    /// This function will return a WebhookError if:
    ///  - the provided signature is invalid
    ///  - the provided secret is invalid
    ///  - the signature timestamp is older than 5 minutes
    pub fn construct_event(
        payload: &str,
        sig: &str,
        secret: &str,
    ) -> Result<WebhookEvent, WebhookError> {
        Self { current_timestamp: Utc::now().timestamp() }.do_construct_event(payload, sig, secret)
    }

    fn do_construct_event(
        self,
        payload: &str,
        sig: &str,
        secret: &str,
    ) -> Result<WebhookEvent, WebhookError> {
        // Get Stripe signature from header
        let signature = Signature::parse(sig)?;
        let signed_payload = format!("{}.{}", signature.t, payload);

        // Compute HMAC with the SHA256 hash function, using endpoing secret as key
        // and signed_payload string as the message.
        let mut mac =
            Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| WebhookError::BadKey)?;
        mac.update(signed_payload.as_bytes());

        let sig = hex::decode(signature.v1).map_err(|_| WebhookError::BadSignature)?;
        mac.verify_slice(sig.as_slice()).map_err(|_| WebhookError::BadSignature)?;

        // Get current timestamp to compare to signature timestamp
        if (self.current_timestamp - signature.t).abs() > 300 {
            return Err(WebhookError::BadTimestamp(signature.t));
        }

        Ok(serde_json::from_str(payload)?)
    }
}

#[cfg(feature = "webhook-events")]
#[derive(Debug)]
struct Signature<'r> {
    t: i64,
    v1: &'r str,
}

#[cfg(feature = "webhook-events")]
impl<'r> Signature<'r> {
    fn parse(raw: &'r str) -> Result<Signature<'r>, WebhookError> {
        use std::collections::HashMap;
        let headers: HashMap<&str, &str> = raw
            .split(',')
            .map(|header| {
                let mut key_and_value = header.split('=');
                let key = key_and_value.next();
                let value = key_and_value.next();
                (key, value)
            })
            .filter_map(|(key, value)| match (key, value) {
                (Some(key), Some(value)) => Some((key, value)),
                _ => None,
            })
            .collect();
        let t = headers.get("t").ok_or(WebhookError::BadSignature)?;
        let v1 = headers.get("v1").ok_or(WebhookError::BadSignature)?;
        Ok(Signature { t: t.parse::<i64>().map_err(WebhookError::BadHeader)?, v1 })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "webhook-events")]
    #[test]
    fn test_signature_parse() {
        use super::Signature;

        let raw_signature =
            "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd";
        let signature = Signature::parse(raw_signature).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
        );

        let raw_signature_with_test_mode = "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd,v0=6ffbb59b2300aae63f272406069a9788598b792a944a07aba816edb039989a39";
        let signature = Signature::parse(raw_signature_with_test_mode).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
        );
    }

    #[cfg(feature = "webhook-events")]
    #[test]
    fn test_webhook_construct_event() {
        let payload = r#"{
  "id": "evt_123",
  "object": "event",
  "account": "acct_123",
  "api_version": "2017-05-25",
  "created": 1533204620,
  "data": {
    "object": {
      "id": "ii_123",
      "object": "invoiceitem",
      "amount": 1000,
      "currency": "usd",
      "customer": "cus_123",
      "date": 1533204620,
      "description": "Test Invoice Item",
      "discountable": false,
      "invoice": "in_123",
      "livemode": false,
      "metadata": {},
      "period": {
        "start": 1533204620,
        "end": 1533204620
      },
      "plan": null,
      "proration": false,
      "quantity": null,
      "subscription": null
    }
  },
  "livemode": false,
  "pending_webhooks": 1,
  "request": {
    "id": "req_123",
    "idempotency_key": "idempotency-key-123"
  },
  "type": "invoiceitem.created"
}
"#;
        let event_timestamp = 1533204620;
        let secret = "webhook_secret".to_string();
        let signature = format!("t={},v1=f0bdba6d4eacbd8ad8a3bbadd7248e633ec1477f7899c124c51b39405fa36613,v0=63f3a72374a733066c4be69ed7f8e5ac85c22c9f0a6a612ab9a025a9e4ee7eef", event_timestamp);

        let webhook = super::Webhook { current_timestamp: event_timestamp };

        let event = webhook
            .do_construct_event(payload, &signature, &secret)
            .expect("Failed to construct event");

        assert_eq!(event.event_type, super::EventType::InvoiceItemCreated);
        assert_eq!(event.id, "evt_123".parse::<crate::EventId>().unwrap());
        assert_eq!(event.account, "acct_123".parse().ok());
        assert_eq!(event.created, 1533204620);
    }
}
