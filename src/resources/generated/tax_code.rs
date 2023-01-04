/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,

    /// Unique identifier for the object.
    pub id: String,

    /// A short name for the tax code.
    pub name: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTaxCodesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTaxCodesIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_tax_codes(
    client: &crate::Client,
    params: GetTaxCodesParams,
) -> crate::Response<crate::params::List<crate::generated::TaxCode>> {
    client.get_query("/tax_codes", params)
}

pub fn get_tax_codes_id(
    client: &crate::Client,
    id: String,
    params: GetTaxCodesIdParams,
) -> crate::Response<crate::generated::TaxCode> {
    client.get_query(&format!("/tax_codes/{id}", id = id), params)
}
