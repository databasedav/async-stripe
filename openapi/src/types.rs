use std::fmt::Display;

use crate::ident::RustIdent;
use crate::rust_type::RustType;
use crate::schema_path::ComponentPath;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustObject {
    Struct(RustStruct),
    Enum(RustEnum),
    FieldedEnum(RustFieldedEnum),
}

impl RustObject {
    pub fn schema_deps(&self) -> Vec<&ComponentPath> {
        let mut deps = vec![];
        self.add_schema_deps(&mut deps);
        deps
    }

    fn add_schema_deps<'a>(&'a self, deps: &mut Vec<&'a ComponentPath>) {
        match self {
            RustObject::Struct(struct_) => {
                for field in &struct_.fields {
                    if let Some(component_path) = field.rust_type.as_component_path() {
                        deps.push(component_path);
                    }
                }
            }
            RustObject::FieldedEnum(enum_) => {
                for variant in &enum_.variants {
                    if let Some(component_path) = variant.rust_type.as_component_path() {
                        deps.push(component_path);
                    }
                }
            }
            RustObject::Enum(_) => {}
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RustFieldedEnum {
    pub doc_comment: Option<String>,
    pub variants: Vec<FieldedEnumVariant>,
}

impl RustFieldedEnum {
    pub fn add_variant(&mut self, variant: FieldedEnumVariant) {
        self.variants.push(variant);
    }
}

/// Specification of a variant for an enum of the form `Ident(Type)`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldedEnumVariant {
    /// Identifier for this variant
    pub variant: RustIdent,
    /// The type of the single field
    pub rust_type: RustType,
}

impl FieldedEnumVariant {
    pub fn new(variant: RustIdent, rust_type: RustType) -> Self {
        Self { variant, rust_type }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RustEnum {
    pub doc_comment: Option<String>,
    pub variants: Vec<EnumVariant>,
    pub default_variant: Option<RustIdent>,
}

impl RustEnum {
    pub fn add_variant(&mut self, variant: EnumVariant) {
        self.variants.push(variant);
    }

    pub fn with_default_variant(mut self, variant: RustIdent) -> Self {
        self.default_variant = Some(variant);
        self
    }
}

/// Specification of a single field-less variant for an enum
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EnumVariant {
    /// Raw string association of the enum, used for renaming and `as_str` implementations
    pub wire_name: String,
    /// Identifier for this variant
    pub variant_name: RustIdent,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RustStruct {
    pub fields: Vec<StructField>,
    pub doc_comment: Option<String>,
}

impl RustStruct {
    pub fn add_field(&mut self, field: StructField) {
        self.fields.push(field);
    }
}

pub struct RustObjectBuilder {
    pub doc_comment: Option<String>,
}

impl RustObjectBuilder {
    pub fn new() -> Self {
        Self { doc_comment: None }
    }

    pub fn doc<T: Display>(mut self, doc_comment: T) -> Self {
        self.doc_comment = Some(doc_comment.to_string());
        self
    }

    pub fn maybe_doc<T: Display>(mut self, doc_comment: Option<T>) -> Self {
        if let Some(doc) = &doc_comment {
            self.doc_comment = Some(doc.to_string());
        }
        self
    }

    pub fn start_struct(&self) -> RustStruct {
        RustStruct { doc_comment: self.doc_comment.clone(), fields: vec![] }
    }

    pub fn start_enum(&self) -> RustEnum {
        RustEnum { doc_comment: self.doc_comment.clone(), variants: vec![], default_variant: None }
    }

    pub fn start_fielded_enum(&self) -> RustFieldedEnum {
        RustFieldedEnum { doc_comment: self.doc_comment.clone(), variants: vec![] }
    }
}

/// Specification for a field in a struct
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StructField {
    /// Used to document this enum if provided
    pub doc_comment: Option<String>,
    /// If provided, used to apply `serde(rename)`
    pub rename_as: Option<String>,
    /// If provided, used to apply `serde(default)`.
    pub deser_default: Option<DeserDefault>,
    /// If provided, used to apply `serde(skip_serializing_if)`.
    pub skip_serializing: Option<&'static str>,
    /// Name of this field
    pub field_name: String,
    /// Type for this field
    pub rust_type: RustType,
}

impl StructField {
    pub fn new<T: Display>(field_name: T, rust_type: RustType) -> Self {
        Self {
            field_name: field_name.to_string(),
            rust_type,
            doc_comment: None,
            rename_as: None,
            deser_default: None,
            skip_serializing: None,
        }
    }

    pub fn rename_as<T: Display>(mut self, rename: T) -> Self {
        self.rename_as = Some(rename.to_string());
        self
    }

    pub fn doc<T: Display>(mut self, doc_comment: T) -> Self {
        self.doc_comment = Some(doc_comment.to_string());
        self
    }

    pub fn skip_serializing(mut self, skip_serializing: &'static str) -> Self {
        self.skip_serializing = Some(skip_serializing);
        self
    }

    pub fn deser_default(mut self, default: DeserDefault) -> Self {
        self.deser_default = Some(default);
        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub enum DeserDefault {
    #[default]
    Default,
    Specific(&'static str),
}

impl DeserDefault {
    pub fn to_serde_attr(self) -> String {
        match self {
            DeserDefault::Default => "#[serde(default)]".to_string(),
            DeserDefault::Specific(val) => format!(r#"#[serde(default = "{val}")]"#),
        }
    }
}
