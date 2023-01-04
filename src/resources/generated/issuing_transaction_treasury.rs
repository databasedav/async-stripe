#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingTransactionTreasury {
    /// The Treasury [ReceivedCredit](https://stripe.com/docs/api/treasury/received_credits) representing this Issuing transaction if it is a refund.
    pub received_credit: Option<String>,

    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) representing this Issuing transaction if it is a capture.
    pub received_debit: Option<String>,
}
