/// Stores representations of [stock keeping units](http://en.wikipedia.org/wiki/Stock_keeping_unit).
/// SKUs describe specific product variations, taking into account any combination of: attributes,
/// currency, and cost.
///
/// For example, a product may be a T-shirt, whereas a specific SKU represents the `size: large`, `color: red` version of that shirt.  Can also be used to manage inventory.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Sku {
    /// Whether the SKU is available for purchase.
    pub active: bool,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    pub attributes: String,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Unique identifier for the object.
    pub id: String,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    pub image: Option<String>,

    pub inventory: crate::generated::SkuInventory,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The dimensions of this SKU for shipping purposes.
    pub package_dimensions: Option<crate::generated::PackageDimensions>,

    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,

    /// The ID of the product this SKU is associated with.
    ///
    /// The product must be currently active.
    pub product: Vec<crate::generated::Product>,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: crate::params::Timestamp,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedGetSkusId {
    Sku(crate::generated::Sku),
    DeletedSku(crate::generated::DeletedSku),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSkusIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSkusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSkusIdParams {
    /// Whether this SKU is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// When specified, `attributes` will partially update the existing attributes dictionary on the product, with the postcondition that a value must be present for each attribute key on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Description of the SKU's inventory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<PostSkusIdParamsInventory>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PostSkusIdParamsPackageDimensions>,

    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,

    /// The ID of the product that this SKU should belong to.
    ///
    /// The product must exist, have the same set of attribute names as the SKU's current product, and be of type `good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSkusParams {
    /// Whether the SKU is available for purchase.
    ///
    /// Default to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The identifier for the SKU.
    ///
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Description of the SKU's inventory.
    pub inventory: PostSkusParamsInventory,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PostSkusParamsPackageDimensions>,

    /// The cost of the item as a nonnegative integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,

    /// The ID of the product this SKU is associated with.
    ///
    /// Must be a product with type `good`.
    pub product: String,
}

/// Description of the SKU's inventory.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSkusIdParamsInventory {
    /// The count of inventory available.
    ///
    /// Required if `type` is `finite`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostSkusIdParamsInventoryType>,

    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<PostSkusIdParamsInventoryValue>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSkusIdParamsPackageDimensions {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,

    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,

    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,

    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}

/// Description of the SKU's inventory.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSkusParamsInventory {
    /// The count of inventory available.
    ///
    /// Required if `type` is `finite`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    pub type_: PostSkusParamsInventoryType,

    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<PostSkusParamsInventoryValue>,
}

/// The dimensions of this SKU for shipping purposes.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSkusParamsPackageDimensions {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,

    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,

    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,

    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSkusIdParamsInventoryType {
    Bucket,
    Finite,
    Infinite,
}

impl PostSkusIdParamsInventoryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bucket => "bucket",
            Self::Finite => "finite",
            Self::Infinite => "infinite",
        }
    }
}

impl AsRef<str> for PostSkusIdParamsInventoryType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSkusIdParamsInventoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSkusIdParamsInventoryType {
    fn default() -> Self {
        Self::Bucket
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSkusIdParamsInventoryValue {
    InStock,
    Limited,
    OutOfStock,
}

impl PostSkusIdParamsInventoryValue {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InStock => "in_stock",
            Self::Limited => "limited",
            Self::OutOfStock => "out_of_stock",
        }
    }
}

impl AsRef<str> for PostSkusIdParamsInventoryValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSkusIdParamsInventoryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSkusIdParamsInventoryValue {
    fn default() -> Self {
        Self::InStock
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSkusParamsInventoryType {
    Bucket,
    Finite,
    Infinite,
}

impl PostSkusParamsInventoryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bucket => "bucket",
            Self::Finite => "finite",
            Self::Infinite => "infinite",
        }
    }
}

impl AsRef<str> for PostSkusParamsInventoryType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSkusParamsInventoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSkusParamsInventoryType {
    fn default() -> Self {
        Self::Bucket
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSkusParamsInventoryValue {
    InStock,
    Limited,
    OutOfStock,
}

impl PostSkusParamsInventoryValue {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InStock => "in_stock",
            Self::Limited => "limited",
            Self::OutOfStock => "out_of_stock",
        }
    }
}

impl AsRef<str> for PostSkusParamsInventoryValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSkusParamsInventoryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSkusParamsInventoryValue {
    fn default() -> Self {
        Self::InStock
    }
}
pub fn get_skus_id(
    client: &crate::Client,
    id: String,
    params: GetSkusIdParams,
) -> crate::Response<ReturnedGetSkusId> {
    client.get_query(&format!("/skus/{id}", id = id), params)
}

pub fn get_skus(
    client: &crate::Client,
    params: GetSkusParams,
) -> crate::Response<crate::params::List<crate::generated::Sku>> {
    client.get_query("/skus", params)
}

pub fn post_skus_id(
    client: &crate::Client,
    id: String,
    params: PostSkusIdParams,
) -> crate::Response<crate::generated::Sku> {
    client.post_form(&format!("/skus/{id}", id = id), params)
}

pub fn post_skus(
    client: &crate::Client,
    params: PostSkusParams,
) -> crate::Response<crate::generated::Sku> {
    client.post_form("/skus", params)
}

pub fn delete_skus_id(
    client: &crate::Client,
    id: String,
) -> crate::Response<crate::generated::DeletedSku> {
    client.delete(&format!("/skus/{id}", id = id))
}
