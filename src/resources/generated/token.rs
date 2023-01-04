/// Tokenization is the process Stripe uses to collect sensitive card or bank
/// account details, or personally identifiable information (PII), directly from
/// your customers in a secure manner.
///
/// A token representing this information is returned to your server to use.
/// You should use our [recommended payments integrations](https://stripe.com/docs/payments) to perform this process client-side.
/// This ensures that no sensitive card data touches your server, and allows your integration to operate in a PCI-compliant way.  If you cannot use client-side tokenization, you can also create tokens using the API with either your publishable or secret API key.
/// Keep in mind that if your integration uses this method, you are responsible for any PCI compliance that may be required, and you must keep your secret API key safe.
/// Unlike with client-side tokenization, your customer's information is not sent directly to Stripe, so we cannot determine how it is handled or stored.  Tokens cannot be stored or used more than once.
/// To store card or bank account information for later use, you can create [Customer](https://stripe.com/docs/api#customers) objects or [Custom accounts](https://stripe.com/docs/api#external_accounts).
/// Note that [Radar](https://stripe.com/docs/radar), our integrated solution for automatic fraud protection, performs best with integrations that use client-side tokenization.  Related guide: [Accept a payment](https://stripe.com/docs/payments/accept-a-payment-charges#web-create-token).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<crate::generated::BankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::Card>,

    /// IP address of the client that generated the token.
    pub client_ip: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: String,

    /// Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTokensTokenParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParams {
    /// Information for the account this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<PostTokensParamsAccount>,

    /// The bank account this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PostTokensParamsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostTokensParamsCard>,

    /// The customer (owned by the application's account) for which to create a token.
    ///
    /// This can be used only with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// For more details, see [Cloning Saved Payment Methods](https://stripe.com/docs/connect/cloning-saved-payment-methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// The updated CVC value this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_update: Option<PostTokensParamsCvcUpdate>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Information for the person this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PostTokensParamsPerson>,

    /// The PII this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<PostTokensParamsPii>,
}

/// Information for the account this token will represent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccount {
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<PostTokensParamsAccountBusinessType>,

    /// Information about the company or business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<PostTokensParamsAccountCompany>,

    /// Information about the person represented by the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PostTokensParamsAccountIndividual>,

    /// Whether the user described by the data in the token has been shown [the Stripe Connected Account Agreement](https://stripe.com/docs/connect/account-tokens#stripe-connected-account-agreement).
    ///
    /// When creating an account token to create a new Connect account, this value must be `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

/// The bank account this token will represent.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsBankAccount {
    /// The name of the person or business that owns the bank account.This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The type of entity that holds the account.
    ///
    /// It can be `company` or `individual`.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<PostTokensParamsBankAccountAccountHolderType>,

    /// The account number for the bank account, in string form.
    ///
    /// Must be a checking account.
    pub account_number: String,

    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PostTokensParamsBankAccountAccountType>,

    /// The country in which the bank account is located.
    pub country: String,

    /// The currency the bank account is in.
    ///
    /// This must be a country/currency pairing that [Stripe supports.](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The routing number, sort code, or other country-appropriateinstitution number for the bank account.
    ///
    /// For US bank accounts, this is required and should bethe ACH routing number, not the wire routing number.
    /// If you are providing an IBAN for`account_number`, this field is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostTokensParamsCard {
    CreditCardSpecs(PostTokensParamsCardCreditCardSpecs),
    String(String),
}

/// The updated CVC value this token will represent.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsCvcUpdate {
    /// The CVC value, in string form.
    pub cvc: String,
}

/// Information for the person this token will represent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPerson {
    /// The person's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostTokensParamsPersonAddress>,

    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<PostTokensParamsPersonAddressKana>,

    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<PostTokensParamsPersonAddressKanji>,

    /// The person's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<PostTokensParamsPersonDob>,

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<PostTokensParamsPersonDocuments>,

    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,

    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// The person's ID number, as appropriate for their country.
    ///
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,

    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The country where the person is a national.
    ///
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,

    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<String>,

    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<PostTokensParamsPersonRegisteredAddress>,

    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PostTokensParamsPersonRelationship>,

    /// The last four digits of the person's Social Security number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PostTokensParamsPersonVerification>,
}

/// The PII this token will represent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPii {
    /// The `id_number` for the PII, in string form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTokensParamsAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl PostTokensParamsAccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::GovernmentEntity => "government_entity",
            Self::Individual => "individual",
            Self::NonProfit => "non_profit",
        }
    }
}

impl AsRef<str> for PostTokensParamsAccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTokensParamsAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTokensParamsAccountBusinessType {
    fn default() -> Self {
        Self::Company
    }
}
/// Information about the company or business.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompany {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostTokensParamsAccountCompanyAddress>,

    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<PostTokensParamsAccountCompanyAddressKana>,

    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<PostTokensParamsAccountCompanyAddressKanji>,

    /// Whether the company's directors have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,

    /// Whether the company's executives have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,

    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,

    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,

    /// Whether the company's owners have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,

    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<PostTokensParamsAccountCompanyOwnershipDeclaration>,

    /// Whether the user described by the data in the token has been shown the Ownership Declaration and indicated that it is correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration_shown_and_signed: Option<bool>,

    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    ///
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,

    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<PostTokensParamsAccountCompanyStructure>,

    /// The business ID number of the company, as appropriate for the company’s country.
    ///
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,

    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,

    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PostTokensParamsAccountCompanyVerification>,
}

/// Information about the person represented by the account.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividual {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostTokensParamsAccountIndividualAddress>,

    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<PostTokensParamsAccountIndividualAddressKana>,

    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<PostTokensParamsAccountIndividualAddressKanji>,

    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<PostTokensParamsAccountIndividualDob>,

    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The Kana variation of the the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,

    /// The individual's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// The government-issued ID number of the individual, as appropriate for the representative’s country.
    ///
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,

    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<PostTokensParamsAccountIndividualPoliticalExposure>,

    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<PostTokensParamsAccountIndividualRegisteredAddress>,

    /// The last four digits of the individual's Social Security Number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PostTokensParamsAccountIndividualVerification>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTokensParamsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PostTokensParamsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostTokensParamsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTokensParamsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTokensParamsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTokensParamsBankAccountAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}

impl PostTokensParamsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Futsu => "futsu",
            Self::Savings => "savings",
            Self::Toza => "toza",
        }
    }
}

impl AsRef<str> for PostTokensParamsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTokensParamsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTokensParamsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsCardCreditCardSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,

    pub exp_month: String,

    pub exp_year: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub number: String,
}

/// The person's address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonAddress {
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

/// The Kana variation of the person's address (Japan only).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

/// The Kanji variation of the person's address (Japan only).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

/// Documents that may be submitted to satisfy various informational requests.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonDocuments {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<PostTokensParamsPersonDocumentsCompanyAuthorization>,

    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<PostTokensParamsPersonDocumentsPassport>,

    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<PostTokensParamsPersonDocumentsVisa>,
}

/// The person's registered address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonRegisteredAddress {
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

/// The relationship that this person has with the account's legal entity.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonRelationship {
    /// Whether the person is a director of the account's legal entity.
    ///
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,

    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,

    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,

    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,

    /// Whether the person is authorized as the primary representative of the account.
    ///
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,

    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// The person's verification status.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonVerification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PostTokensParamsPersonVerificationAdditionalDocument>,

    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PostTokensParamsPersonVerificationDocument>,
}

/// The company's primary address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompanyAddress {
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

/// The Kana variation of the company's primary address (Japan only).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompanyAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

/// The Kanji variation of the company's primary address (Japan only).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompanyAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

/// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompanyOwnershipDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTokensParamsAccountCompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
}

impl PostTokensParamsAccountCompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FreeZoneEstablishment => "free_zone_establishment",
            Self::FreeZoneLlc => "free_zone_llc",
            Self::GovernmentInstrumentality => "government_instrumentality",
            Self::GovernmentalUnit => "governmental_unit",
            Self::IncorporatedNonProfit => "incorporated_non_profit",
            Self::LimitedLiabilityPartnership => "limited_liability_partnership",
            Self::Llc => "llc",
            Self::MultiMemberLlc => "multi_member_llc",
            Self::PrivateCompany => "private_company",
            Self::PrivateCorporation => "private_corporation",
            Self::PrivatePartnership => "private_partnership",
            Self::PublicCompany => "public_company",
            Self::PublicCorporation => "public_corporation",
            Self::PublicPartnership => "public_partnership",
            Self::SingleMemberLlc => "single_member_llc",
            Self::SoleEstablishment => "sole_establishment",
            Self::SoleProprietorship => "sole_proprietorship",
            Self::TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            Self::UnincorporatedAssociation => "unincorporated_association",
            Self::UnincorporatedNonProfit => "unincorporated_non_profit",
        }
    }
}

impl AsRef<str> for PostTokensParamsAccountCompanyStructure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTokensParamsAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTokensParamsAccountCompanyStructure {
    fn default() -> Self {
        Self::FreeZoneEstablishment
    }
}
/// Information on the verification state of the company.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompanyVerification {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PostTokensParamsAccountCompanyVerificationDocument>,
}

/// The individual's primary address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualAddress {
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

/// The Kana variation of the the individual's primary address (Japan only).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

/// The Kanji variation of the the individual's primary address (Japan only).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTokensParamsAccountIndividualPoliticalExposure {
    Existing,
    None,
}

impl PostTokensParamsAccountIndividualPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Existing => "existing",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostTokensParamsAccountIndividualPoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTokensParamsAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTokensParamsAccountIndividualPoliticalExposure {
    fn default() -> Self {
        Self::Existing
    }
}
/// The individual's registered address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualRegisteredAddress {
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

/// The individual's verification document information.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualVerification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document:
        Option<PostTokensParamsAccountIndividualVerificationAdditionalDocument>,

    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PostTokensParamsAccountIndividualVerificationDocument>,
}

/// One or more documents that demonstrate proof that this person is authorized to represent the company.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonDocumentsCompanyAuthorization {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

/// One or more documents showing the person's passport page with photo and personal data.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonDocumentsPassport {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

/// One or more documents showing the person's visa required for living in the country where they are residing.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonDocumentsVisa {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonVerificationAdditionalDocument {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

/// An identifying document, either a passport or local ID card.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsPersonVerificationDocument {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

/// A document verifying the business.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountCompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualVerificationAdditionalDocument {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

/// An identifying document, either a passport or local ID card.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTokensParamsAccountIndividualVerificationDocument {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

pub fn get_tokens_token(
    client: &crate::Client,
    token: String,
    params: GetTokensTokenParams,
) -> crate::Response<crate::generated::Token> {
    client.get_query(&format!("/tokens/{token}", token = token), params)
}

pub fn post_tokens(
    client: &crate::Client,
    params: PostTokensParams,
) -> crate::Response<crate::generated::Token> {
    client.post_form("/tokens", params)
}
