/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum BalanceTransactionSource {
    ApplicationFee(crate::generated::ApplicationFee),
    Charge(crate::generated::Charge),
    ConnectCollectionTransfer(crate::generated::ConnectCollectionTransfer),
    Dispute(crate::generated::Dispute),
    FeeRefund(crate::generated::FeeRefund),
    IssuingAuthorization(crate::generated::IssuingAuthorization),
    IssuingDispute(crate::generated::IssuingDispute),
    IssuingTransaction(crate::generated::IssuingTransaction),
    Payout(crate::generated::Payout),
    PlatformTaxFee(crate::generated::PlatformTaxFee),
    Refund(crate::generated::Refund),
    ReserveTransaction(crate::generated::ReserveTransaction),
    TaxDeductedAtSource(crate::generated::TaxDeductedAtSource),
    Topup(crate::generated::Topup),
    Transfer(crate::generated::Transfer),
    TransferReversal(crate::generated::TransferReversal),
}
