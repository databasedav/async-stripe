/// You can store multiple cards on a customer in order to charge the customer
/// later.
///
/// You can also store multiple debit cards on a recipient in order to transfer to those cards later.  Related guide: [Card Payments with Sources](https://stripe.com/docs/sources/cards).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Card {
    /// The account this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Vec<crate::generated::Account>>,

    /// City/District/Suburb/Town/Village.
    pub address_city: Option<String>,

    /// Billing address country, if provided when creating card.
    pub address_country: Option<String>,

    /// Address line 1 (Street address/PO Box/Company name).
    pub address_line1: Option<String>,

    /// If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,

    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub address_line2: Option<String>,

    /// State/County/Province/Region.
    pub address_state: Option<String>,

    /// ZIP or postal code.
    pub address_zip: Option<String>,

    /// If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_zip_check: Option<String>,

    /// A set of available payout methods for this card.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<CardAvailablePayoutMethods>>,

    /// Card brand.
    ///
    /// Can be `American Express`, `Diners Club`, `Discover`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    pub brand: String,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// Three-letter [ISO code for currency](https://stripe.com/docs/payouts).
    ///
    /// Only applicable on accounts (not customers or recipients).
    /// The card can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The customer that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    ///
    /// A result of unchecked indicates that CVC was provided but hasn't been checked yet.
    /// Checks are typically performed when attaching a card to a Customer object, or when creating a charge.
    /// For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    pub cvc_check: Option<String>,

    /// Whether this card is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,

    /// Unique identifier for the object.
    pub id: String,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: String,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// Cardholder name.
    pub name: Option<String>,

    /// For external accounts, possible values are `new` and `errored`.
    ///
    /// If a transfer fails, the status is set to `errored` and transfers are stopped until account details are updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// If the card number is tokenized, this is the method that was used.
    ///
    /// Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    pub tokenization_method: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedPostCustomersCustomerSourcesId {
    Card(crate::generated::Card),
    BankAccount(crate::generated::BankAccount),
    Source(crate::generated::Source),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerSourcesIdParams {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<PostCustomersCustomerSourcesIdParamsAccountHolderType>,

    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,

    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,

    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,

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

    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<PostCustomersCustomerSourcesIdParamsOwner>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedDeleteCustomersCustomerSourcesId {
    PaymentSource(crate::generated::PaymentSource),
    DeletedPaymentSource(crate::generated::DeletedPaymentSource),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostAccountsAccountExternalAccountsIdParams {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<PostAccountsAccountExternalAccountsIdParamsAccountHolderType>,

    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PostAccountsAccountExternalAccountsIdParamsAccountType>,

    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,

    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    /// When set to true, this becomes the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,

    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,

    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,

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

    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CardAvailablePayoutMethods {
    Instant,
    Standard,
}

impl CardAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Standard => "standard",
        }
    }
}

impl AsRef<str> for CardAvailablePayoutMethods {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CardAvailablePayoutMethods {
    fn default() -> Self {
        Self::Instant
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerSourcesIdParamsAccountHolderType {
    Company,
    Individual,
}

impl PostCustomersCustomerSourcesIdParamsAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerSourcesIdParamsAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersCustomerSourcesIdParamsAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerSourcesIdParamsAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerSourcesIdParamsOwner {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostCustomersCustomerSourcesIdParamsOwnerAddress>,

    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    Company,
    Individual,
}

impl PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAccountsAccountExternalAccountsIdParamsAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}

impl PostAccountsAccountExternalAccountsIdParamsAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Futsu => "futsu",
            Self::Savings => "savings",
            Self::Toza => "toza",
        }
    }
}

impl AsRef<str> for PostAccountsAccountExternalAccountsIdParamsAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAccountsAccountExternalAccountsIdParamsAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAccountsAccountExternalAccountsIdParamsAccountType {
    fn default() -> Self {
        Self::Checking
    }
}
/// Owner's address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerSourcesIdParamsOwnerAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

pub fn post_customers_customer_sources_id(
    client: &crate::Client,
    customer: String,
    id: String,
    params: PostCustomersCustomerSourcesIdParams,
) -> crate::Response<ReturnedPostCustomersCustomerSourcesId> {
    client.post_form(
        &format!("/customers/{customer}/sources/{id}", customer = customer, id = id),
        params,
    )
}

pub fn delete_customers_customer_sources_id(
    client: &crate::Client,
    customer: String,
    id: String,
) -> crate::Response<ReturnedDeleteCustomersCustomerSourcesId> {
    client.delete(&format!("/customers/{customer}/sources/{id}", customer = customer, id = id))
}

pub fn post_accounts_account_external_accounts_id(
    client: &crate::Client,
    account: String,
    id: String,
    params: PostAccountsAccountExternalAccountsIdParams,
) -> crate::Response<crate::generated::ExternalAccount> {
    client.post_form(
        &format!("/accounts/{account}/external_accounts/{id}", account = account, id = id),
        params,
    )
}

pub fn delete_accounts_account_external_accounts_id(
    client: &crate::Client,
    account: String,
    id: String,
) -> crate::Response<crate::generated::DeletedExternalAccount> {
    client.delete(&format!(
        "/accounts/{account}/external_accounts/{id}",
        account = account,
        id = id
    ))
}
