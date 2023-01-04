#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceTaxDetails {
    /// Describes the purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    pub tax_exempt: OrdersV2ResourceTaxDetailsTaxExempt,

    /// The purchaser's tax IDs to be used in calculation of tax for this Order.
    pub tax_ids: Vec<crate::generated::OrdersV2ResourceTaxDetailsResourceTaxId>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl OrdersV2ResourceTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for OrdersV2ResourceTaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceTaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrdersV2ResourceTaxDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}
