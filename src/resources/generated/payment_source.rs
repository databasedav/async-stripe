/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PaymentSource {
    Account(crate::generated::Account),
    BankAccount(crate::generated::BankAccount),
    Card(crate::generated::Card),
    Source(crate::generated::Source),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerSourcesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerSourcesIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerSourcesParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Please refer to full [documentation](https://stripe.com/docs/api) instead.
    pub source: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

pub fn get_customers_customer_sources(
    client: &crate::Client,
    customer: String,
    params: GetCustomersCustomerSourcesParams,
) -> crate::Response<crate::params::List<crate::generated::PaymentSource>> {
    client.get_query(&format!("/customers/{customer}/sources", customer = customer), params)
}

pub fn get_customers_customer_sources_id(
    client: &crate::Client,
    customer: String,
    id: String,
    params: GetCustomersCustomerSourcesIdParams,
) -> crate::Response<crate::generated::PaymentSource> {
    client.get_query(
        &format!("/customers/{customer}/sources/{id}", customer = customer, id = id),
        params,
    )
}

pub fn post_customers_customer_sources(
    client: &crate::Client,
    customer: String,
    params: PostCustomersCustomerSourcesParams,
) -> crate::Response<crate::generated::PaymentSource> {
    client.post_form(&format!("/customers/{customer}/sources", customer = customer), params)
}
