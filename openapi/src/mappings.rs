use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::rust_type::ExtType;

pub fn id_renames() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("fee_refund", "application_fee_refund"),
        ("invoiceitem", "invoice_item"),
        ("line_item", "invoice_line_item"),
        ("source_transaction", "charge"),
        ("item", "checkout_session_item"),
    ])
}

fn field_name_to_ext_type_mappings() -> HashMap<&'static str, ExtType> {
    HashMap::from([
        ("metadata", ExtType::Metadata),
        ("delay_days", ExtType::DelayDays),
        ("expires_at", ExtType::Scheduled),
        ("off_session", ExtType::PaymentIntentOffSession),
        ("up_to", ExtType::UpTo),
    ])
}

lazy_static! {
    pub static ref ID_RENAMES: HashMap<&'static str, &'static str> = id_renames();
    pub static ref FIELD_TYPE_INFERENCE: HashMap<&'static str, ExtType> =
        field_name_to_ext_type_mappings();
}

pub fn maybe_force_field_type(field_name: Option<&str>) -> Option<ExtType> {
    if let Some(name) = field_name {
        FIELD_TYPE_INFERENCE.get(name).copied()
    } else {
        None
    }
}
