/// Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.
///
/// Related guide: [Tax Rates](https://stripe.com/docs/billing/taxes/tax-rates).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TaxRate {
    /// Defaults to `true`.
    ///
    /// When set to `false`, this tax rate cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub active: bool,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    pub description: Option<String>,

    /// The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invoice page.
    pub display_name: String,

    /// Unique identifier for the object.
    pub id: String,

    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,

    /// The jurisdiction for the tax rate.
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub jurisdiction: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// This represents the tax rate percent out of 100.
    pub percentage: f64,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,

    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxRateTaxType>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTaxRatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTaxRatesTaxRateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTaxRatesParams {
    /// Flag determining whether the tax rate is active or inactive (archived).
    ///
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The display name of the tax rate, which will be shown to users.
    pub display_name: String,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,

    /// The jurisdiction for the tax rate.
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// This represents the tax rate percent out of 100.
    pub percentage: f64,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<PostTaxRatesParamsTaxType>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTaxRatesTaxRateParams {
    /// Flag determining whether the tax rate is active or inactive (archived).
    ///
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The display name of the tax rate, which will be shown to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The jurisdiction for the tax rate.
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<PostTaxRatesTaxRateParamsTaxType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxRateTaxType {
    Gst,
    Hst,
    Jct,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl TaxRateTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gst => "gst",
            Self::Hst => "hst",
            Self::Jct => "jct",
            Self::Pst => "pst",
            Self::Qst => "qst",
            Self::Rst => "rst",
            Self::SalesTax => "sales_tax",
            Self::Vat => "vat",
        }
    }
}

impl AsRef<str> for TaxRateTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TaxRateTaxType {
    fn default() -> Self {
        Self::Gst
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTaxRatesParamsTaxType {
    Gst,
    Hst,
    Jct,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl PostTaxRatesParamsTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gst => "gst",
            Self::Hst => "hst",
            Self::Jct => "jct",
            Self::Pst => "pst",
            Self::Qst => "qst",
            Self::Rst => "rst",
            Self::SalesTax => "sales_tax",
            Self::Vat => "vat",
        }
    }
}

impl AsRef<str> for PostTaxRatesParamsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTaxRatesParamsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTaxRatesParamsTaxType {
    fn default() -> Self {
        Self::Gst
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTaxRatesTaxRateParamsTaxType {
    Gst,
    Hst,
    Jct,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl PostTaxRatesTaxRateParamsTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gst => "gst",
            Self::Hst => "hst",
            Self::Jct => "jct",
            Self::Pst => "pst",
            Self::Qst => "qst",
            Self::Rst => "rst",
            Self::SalesTax => "sales_tax",
            Self::Vat => "vat",
        }
    }
}

impl AsRef<str> for PostTaxRatesTaxRateParamsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTaxRatesTaxRateParamsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTaxRatesTaxRateParamsTaxType {
    fn default() -> Self {
        Self::Gst
    }
}
pub fn get_tax_rates(
    client: &crate::Client,
    params: GetTaxRatesParams,
) -> crate::Response<crate::params::List<crate::generated::TaxRate>> {
    client.get_query("/tax_rates", params)
}

pub fn get_tax_rates_tax_rate(
    client: &crate::Client,
    tax_rate: String,
    params: GetTaxRatesTaxRateParams,
) -> crate::Response<crate::generated::TaxRate> {
    client.get_query(&format!("/tax_rates/{tax_rate}", tax_rate = tax_rate), params)
}

pub fn post_tax_rates(
    client: &crate::Client,
    params: PostTaxRatesParams,
) -> crate::Response<crate::generated::TaxRate> {
    client.post_form("/tax_rates", params)
}

pub fn post_tax_rates_tax_rate(
    client: &crate::Client,
    tax_rate: String,
    params: PostTaxRatesTaxRateParams,
) -> crate::Response<crate::generated::TaxRate> {
    client.post_form(&format!("/tax_rates/{tax_rate}", tax_rate = tax_rate), params)
}
