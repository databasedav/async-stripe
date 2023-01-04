use std::collections::BTreeSet;

const DEFAULT_DERIVES: [Derive; 2] = [Derive::Debug, Derive::Clone];

/// Attributes a struct or enum can derive
/// NB: ordering is purposefully alphabetic so we can derive an `Ord` which sorts
/// variants alphabetically
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Derive {
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    PartialEq,
    Serialize,
}

impl Derive {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Copy => "Copy",
            Self::Clone => "Clone",
            Self::Debug => "Debug",
            Self::Default => "Default",
            Self::Eq => "Eq",
            Self::PartialEq => "PartialEq",
            // So no need to worry about imports
            Self::Serialize => "serde::Serialize",
            Self::Deserialize => "serde::Deserialize",
        }
    }
}

pub fn write_derives_line(additional_derives: &[Derive]) -> String {
    // Use a `BTreeSet` so that we have consistent ordering when printing derives
    let mut to_derive = BTreeSet::from(DEFAULT_DERIVES);
    for derive in additional_derives {
        to_derive.insert(*derive);
    }
    let inner = to_derive.iter().map(Derive::as_str).collect::<Vec<_>>().join(",");
    format!("#[derive({inner})]")
}
