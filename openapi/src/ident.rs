use std::fmt::{Display, Formatter};
use std::ops::Deref;

use heck::CamelCase;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RustIdent(String);

impl RustIdent {
    pub fn from_valid(val: String) -> Self {
        Self(val)
    }

    pub fn create(val: &str) -> Self {
        if val.contains('.') {
            Self(val.replace('.', "_").to_camel_case())
        } else {
            Self(val.to_camel_case())
        }
    }

    pub fn joined<T: Display, U: Display>(piece1: T, piece2: U) -> Self {
        Self::create(&format!("{piece1}_{piece2}"))
    }
}

impl Deref for RustIdent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RustIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
