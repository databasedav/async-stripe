/// `Exchange Rate` objects allow you to determine the rates that Stripe is
/// currently using to convert from one currency to another.
///
/// Since this number is variable throughout the day, there are various reasons why you might want to know the current rate (for example, to dynamically price an item for a user with a default payment in a foreign currency).  If you want a guarantee that the charge is made with a certain exchange rate you expect is current, you can pass in `exchange_rate` to charges endpoints. If the value is no longer up to date, the charge won't go through.
/// Please refer to our [Exchange Rates API](https://stripe.com/docs/exchange-rates) guide for more details.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ExchangeRate {
    /// Unique identifier for the object.
    ///
    /// Represented as the three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) in lowercase.
    pub id: String,

    /// Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: f64,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetExchangeRatesParams {
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
pub struct GetExchangeRatesRateIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_exchange_rates(
    client: &crate::Client,
    params: GetExchangeRatesParams,
) -> crate::Response<crate::params::List<crate::generated::ExchangeRate>> {
    client.get_query("/exchange_rates", params)
}

pub fn get_exchange_rates_rate_id(
    client: &crate::Client,
    rate_id: String,
    params: GetExchangeRatesRateIdParams,
) -> crate::Response<crate::generated::ExchangeRate> {
    client.get_query(&format!("/exchange_rates/{rate_id}", rate_id = rate_id), params)
}
