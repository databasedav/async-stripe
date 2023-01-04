/// Stripe needs to collect certain pieces of information about each account
/// created.
///
/// These requirements can differ depending on the account's country.
/// The Country Specs API makes these rules available to your integration.  You can also view the information from this API call as [an online guide](/docs/connect/required-verification-information).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CountrySpec {
    /// The default currency for this country.
    ///
    /// This applies to both payment methods and bank accounts.
    pub default_currency: crate::currency::Currency,

    /// Unique identifier for the object.
    ///
    /// Represented as the ISO country code for this country.
    pub id: String,

    /// Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: Vec<String>,

    /// Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,

    /// Payment methods available in the specified country.
    ///
    /// You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list.
    /// The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,

    /// Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,

    pub verification_fields: crate::generated::CountrySpecVerificationFields,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCountrySpecsParams {
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
pub struct GetCountrySpecsCountryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_country_specs(
    client: &crate::Client,
    params: GetCountrySpecsParams,
) -> crate::Response<crate::params::List<crate::generated::CountrySpec>> {
    client.get_query("/country_specs", params)
}

pub fn get_country_specs_country(
    client: &crate::Client,
    country: String,
    params: GetCountrySpecsCountryParams,
) -> crate::Response<crate::generated::CountrySpec> {
    client.get_query(&format!("/country_specs/{country}", country = country), params)
}
