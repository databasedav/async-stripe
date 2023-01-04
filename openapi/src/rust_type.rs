use std::fmt::{Display, Formatter};
use std::ops::Deref;

use anyhow::anyhow;

use crate::ident::RustIdent;
use crate::schema_path::ComponentPath;
use crate::types::{DeserDefault, RustObject};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PrintableType {
    WithinFile(RustIdent),
    Simple(SimpleType),
    Compound(CompoundTypeKind, Box<PrintableType>),
}

impl Display for PrintableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WithinFile(ident) => write!(f, "{ident}"),
            Self::Simple(typ) => write!(f, "{}", typ.print_with_import()),
            Self::Compound(container, typ) => {
                write!(f, "{}<{typ}>", container.print_with_import())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
    Object(RustObject),
    Simple(SimpleType),
    Compound(CompoundTypeKind, Box<RustType>),
}

impl RustType {
    pub fn bool() -> Self {
        Self::Simple(SimpleType::Bool)
    }

    pub fn float() -> Self {
        Self::Simple(SimpleType::Float)
    }

    pub fn string() -> Self {
        Self::Simple(SimpleType::String)
    }

    pub fn id(ident: RustIdent) -> Self {
        Self::Simple(SimpleType::Id(ident))
    }

    pub fn int(typ: IntType) -> Self {
        Self::Simple(SimpleType::Int(typ))
    }

    pub fn ext(ext: ExtType) -> Self {
        Self::Simple(SimpleType::Ext(ext))
    }

    pub fn option(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::Option, Box::new(typ))
    }

    pub fn boxed(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::Box, Box::new(typ))
    }

    pub fn list(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::List, Box::new(typ))
    }

    pub fn vec(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::Vec, Box::new(typ))
    }

    pub fn from_reference(reference: &str) -> Self {
        let path = ComponentPath::from_reference(reference);
        Self::Simple(SimpleType::Component(path))
    }

    pub fn should_box(&self) -> bool {
        match self {
            RustType::Simple(SimpleType::Component(path)) => path.to_string() == "api_errors",
            _ => false,
        }
    }

    pub fn as_nullable(&self) -> Self {
        match self {
            Self::Compound(CompoundTypeKind::List, _)
            | Self::Compound(CompoundTypeKind::Option, _) => self.clone(),
            _ => Self::option(self.clone()),
        }
    }

    pub fn as_skip_serializing(&self) -> Option<&'static str> {
        match self {
            Self::Compound(CompoundTypeKind::Option, _) => Some("Option::is_none"),
            _ => None,
        }
    }

    pub fn as_component_path(&self) -> Option<&ComponentPath> {
        match self {
            Self::Simple(SimpleType::Component(path)) => Some(path),
            Self::Compound(_, typ) => typ.as_component_path(),
            _ => None,
        }
    }

    pub fn as_rust_obj(&self) -> Option<&RustObject> {
        match self {
            Self::Object(obj) => Some(obj),
            Self::Simple(_) => None,
            Self::Compound(_, typ) => typ.as_rust_obj(),
        }
    }

    pub fn is_option(&self) -> bool {
        matches!(self, Self::Compound(CompoundTypeKind::Option, _))
    }

    pub fn as_deser_default(&self) -> Option<DeserDefault> {
        match self {
            Self::Simple(SimpleType::Bool)
            | Self::Compound(CompoundTypeKind::Vec, _)
            | Self::Compound(CompoundTypeKind::List, _) => Some(DeserDefault::Default),
            Self::Simple(SimpleType::Id(ident)) => {
                if ident.deref() == "InvoiceId" {
                    Some(DeserDefault::Specific("InvoiceId::none"))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn to_printable_with_object_name(&self, ident: RustIdent) -> PrintableType {
        match self {
            Self::Object(_) => PrintableType::WithinFile(ident),
            Self::Simple(typ) => PrintableType::Simple(typ.clone()),
            Self::Compound(kind, typ) => {
                PrintableType::Compound(*kind, Box::new(typ.to_printable_with_object_name(ident)))
            }
        }
    }

    pub fn to_printable(&self) -> anyhow::Result<PrintableType> {
        match self {
            Self::Object(_) => Err(anyhow!(
                "Cannot print type with an `Object` variant since the name is not known"
            )),
            Self::Simple(typ) => Ok(PrintableType::Simple(typ.clone())),
            Self::Compound(kind, typ) => {
                Ok(PrintableType::Compound(*kind, Box::new(typ.to_printable()?)))
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CompoundTypeKind {
    List,
    Vec,
    Expandable,
    Option,
    Box,
}

impl CompoundTypeKind {
    pub fn import_from(&self) -> Option<&'static str> {
        match self {
            Self::List | Self::Expandable => Some("params"),
            Self::Vec | Self::Option | Self::Box => None,
        }
    }

    pub fn print_with_import(&self) -> String {
        if let Some(import_from) = self.import_from() {
            format!("crate::{import_from}::{self}")
        } else {
            self.to_string()
        }
    }
}

impl Display for CompoundTypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Self::List => "List",
            Self::Vec => "Vec",
            Self::Expandable => "Expandable",
            Self::Option => "Option",
            Self::Box => "Box",
        };
        write!(f, "{disp}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimpleType {
    Bool,
    Float,
    String,
    Int(IntType),
    Ext(ExtType),
    Component(ComponentPath),
    Id(RustIdent),
}

impl SimpleType {
    pub fn import_from(&self) -> Option<&'static str> {
        match self {
            Self::Bool | Self::Float | Self::String | Self::Int(_) => None,
            Self::Ext(inner) => Some(inner.import_from()),
            Self::Component(_) => Some("generated"),
            Self::Id(_) => Some("ids"),
        }
    }

    pub fn print_with_import(&self) -> String {
        if let Some(import_from) = self.import_from() {
            format!("crate::{import_from}::{self}")
        } else {
            self.to_string()
        }
    }
}

impl Display for SimpleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Float => write!(f, "f64"),
            Self::String => write!(f, "String"),
            Self::Int(typ) => write!(f, "{typ}"),
            Self::Ext(ext) => write!(f, "{ext}"),
            Self::Component(path) => write!(f, "{}", RustIdent::create(path)),
            Self::Id(ident) => write!(f, "{ident}"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum IntType {
    U8,
    U32,
    U64,
    I64,
}

impl Display for IntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::U8 => "u8",
                Self::U32 => "u32",
                Self::U64 => "u64",
                Self::I64 => "i64",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ExtType {
    Currency,
    RangeQueryTs,
    Metadata,
    DelayDays,
    Scheduled,
    UpTo,
    Timestamp,
    PaymentIntentOffSession,
}

impl ExtType {
    pub fn import_from(&self) -> &'static str {
        match self {
            Self::Currency => "currency",
            Self::RangeQueryTs | Self::Metadata | Self::Timestamp => "params",
            Self::DelayDays | Self::Scheduled | Self::UpTo | Self::PaymentIntentOffSession => {
                "resources"
            }
        }
    }

    pub fn ident(&self) -> &'static str {
        match self {
            Self::Currency => "Currency",
            Self::RangeQueryTs => "RangeQueryTs",
            Self::Timestamp => "Timestamp",
            Self::Metadata => "Metadata",
            Self::DelayDays => "DelayDays",
            Self::Scheduled => "Scheduled",
            Self::PaymentIntentOffSession => "PaymentIntentOffSession",
            Self::UpTo => "UpTo",
        }
    }
}

impl Display for ExtType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident())
    }
}
