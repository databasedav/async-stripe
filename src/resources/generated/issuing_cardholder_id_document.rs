#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardholderIdDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<Vec<crate::generated::File>>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<Vec<crate::generated::File>>,
}
