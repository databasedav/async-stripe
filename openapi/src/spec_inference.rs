use std::borrow::Borrow;
use std::ops::Deref;

use anyhow::Context;
use heck::SnakeCase;
use openapiv3::{AdditionalProperties, ReferenceOr, Schema, SchemaKind, Type};

use crate::ident::RustIdent;
use crate::mappings::maybe_force_field_type;
use crate::rust_type::{ExtType, RustType};
use crate::schema_path::ComponentPath;
use crate::spec::{
    as_data_array_item, as_object_enum_name, is_enum_with_just_empty_string, ExpansionResources,
};
use crate::types::{
    EnumVariant, FieldedEnumVariant, RustFieldedEnum, RustObject, RustObjectBuilder, StructField,
};
use crate::util::infer_integer_type;

pub fn infer_schema_type(field: &Schema, field_name: Option<&str>) -> RustType {
    if let Some(ext_type) = maybe_force_field_type(field_name) {
        return RustType::ext(ext_type);
    }
    let maybe_doc_comment = field.schema_data.description.as_deref();
    match &field.schema_kind {
        // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
        SchemaKind::Type(Type::Boolean {}) => RustType::bool(),
        SchemaKind::Type(Type::Number(_)) => RustType::float(),
        SchemaKind::Type(Type::Integer(format)) => infer_integer_type(field_name, &format.format),
        SchemaKind::Type(Type::String(typ)) => {
            if let Some(f_name) = field_name {
                if f_name == "currency" || f_name.ends_with("_currency") {
                    return RustType::ext(ExtType::Currency);
                }
            }
            let variants = typ.enumeration.iter().flatten().cloned().collect::<Vec<_>>();
            if !variants.is_empty() {
                let variants = build_enum_variants(&variants);
                let mut enum_obj = RustObjectBuilder::new().start_enum();
                let first_variant = variants.first().cloned();
                for variant in variants {
                    enum_obj.add_variant(variant);
                }
                if let Some(first_variant) = first_variant {
                    enum_obj = enum_obj.with_default_variant(first_variant.variant_name)
                }
                RustType::Object(RustObject::Enum(enum_obj))
            } else {
                RustType::string()
            }
        }
        SchemaKind::Type(Type::Array(typ)) => {
            let element = typ.items.clone().unwrap().unbox();
            let element_type = infer_schema_or_ref_type(&element, field_name);
            RustType::vec(element_type)
        }
        SchemaKind::Type(Type::Object(typ)) => {
            if as_object_enum_name(field).as_deref() == Some("list") {
                let element = as_data_array_item(typ).unwrap_or_else(|| {
                    panic!("Expected to find array item but found {:?}", field.schema_kind)
                });
                let element_type = infer_schema_or_ref_type(element, field_name);
                return RustType::list(element_type);
            }

            if let Some(AdditionalProperties::Schema(additional)) = &typ.additional_properties {
                return infer_schema_or_ref_type(additional, field_name);
            }
            // Generate the struct type
            let mut rust_struct =
                RustObjectBuilder::new().maybe_doc(maybe_doc_comment).start_struct();
            for (prop_field_name, field_spec) in &typ.properties {
                let is_required = typ.required.contains(prop_field_name);
                let rust_type = infer_schema_or_ref_type(field_spec, Some(prop_field_name));
                let field = build_struct_field(rust_type, prop_field_name, field_spec, is_required);
                rust_struct.add_field(field);
            }
            RustType::Object(RustObject::Struct(rust_struct))
        }
        SchemaKind::AnyOf { any_of: fields } | SchemaKind::OneOf { one_of: fields } => {
            let fields = fields
                .iter()
                .filter(|field| !is_enum_with_just_empty_string(field))
                .collect::<Vec<_>>();
            if fields.len() == 1 {
                infer_schema_or_ref_type(fields[0], field_name)
            } else if let Some(resources) = field.schema_data.extensions.get("x-expansionResources")
            {
                let expansion_resources =
                    serde_json::from_value::<ExpansionResources>(resources.clone())
                        .expect("Could not deserialize expansion resources");

                let ty_ = infer_schema_type(
                    &Schema {
                        schema_data: Default::default(),
                        schema_kind: expansion_resources.into_schema_kind(),
                    },
                    field_name,
                );
                // TODO: handle expand
                RustType::vec(ty_)
            } else if fields[0].as_item().and_then(|s| s.schema_data.title.as_deref())
                == Some("range_query_specs")
            {
                RustType::ext(ExtType::RangeQueryTs)
            } else {
                let object_builder = RustObjectBuilder::new().maybe_doc(maybe_doc_comment);
                let enum_ = build_fielded_enum(&fields, field_name, object_builder)
                    .expect("Could not build enum with fields");
                RustType::Object(RustObject::FieldedEnum(enum_))
            }
        }
        _ => {
            panic!("unhandled field type for");
        }
    }
}

pub fn infer_schema_or_ref_type<T: Borrow<Schema>>(
    field: &ReferenceOr<T>,
    field_name: Option<&str>,
) -> RustType {
    let typ = match field {
        ReferenceOr::Reference { reference } => RustType::from_reference(reference),
        ReferenceOr::Item(schema) => infer_schema_type(schema.borrow(), field_name),
    };
    if typ.should_box() {
        RustType::boxed(typ)
    } else {
        typ
    }
}

fn build_enum_variants(options: &[String]) -> Vec<EnumVariant> {
    let mut enum_variants = vec![];
    for wire_name in options {
        if wire_name.trim().is_empty() {
            continue;
        }
        let variant_name = match wire_name.as_str() {
            "*" => RustIdent::create("all"),
            n => {
                if n.chars().next().unwrap().is_ascii_digit() {
                    RustIdent::from_valid(format!("V{}", n.to_string().replace(['-', '.'], "_")))
                } else {
                    // Hit for the case of timezones, e.g. Etc+7 and Etc-7
                    if wire_name.contains(['+', '-']) {
                        let cleaned = wire_name.replace('+', "Plus").replace('-', "Minus");
                        RustIdent::create(&cleaned)
                    } else {
                        RustIdent::create(wire_name)
                    }
                }
            }
        };
        enum_variants.push(EnumVariant { wire_name: wire_name.clone(), variant_name });
    }
    enum_variants
}

pub fn build_fielded_enum<T: Borrow<ReferenceOr<Schema>>>(
    fields: &Vec<T>,
    field_name: Option<&str>,
    object_builder: RustObjectBuilder,
) -> anyhow::Result<RustFieldedEnum> {
    let mut enum_ = object_builder.start_fielded_enum();
    for option in fields {
        match option.borrow() {
            ReferenceOr::Reference { reference } => {
                let option_schema_path = ComponentPath::from_reference(reference);
                enum_.add_variant(FieldedEnumVariant::new(
                    RustIdent::create(option_schema_path.deref()),
                    RustType::from_reference(reference),
                ));
            }
            ReferenceOr::Item(item) => {
                let rust_type = infer_schema_type(item, field_name);
                let variant_ident = infer_any_of_enum_name(item, &rust_type)
                    .with_context(|| format!("Could not an infer a variant name for {item:?}"))?;
                enum_.add_variant(FieldedEnumVariant::new(
                    RustIdent::create(&variant_ident),
                    rust_type,
                ));
            }
        }
    }
    Ok(enum_)
}

fn infer_any_of_enum_name(schema: &Schema, rust_type: &RustType) -> Option<String> {
    if let Some(title) = &schema.schema_data.title {
        return Some(title.clone());
    }
    if let Some(desc) = &schema.schema_data.description {
        if desc.contains("The ID of") {
            return Some("Id".into());
        }
    }
    if let RustType::Object(RustObject::Enum(enum_)) = rust_type {
        let len = enum_.variants.len();
        // Try to infer variant name based on enum fields if there aren't many.
        // TODO: better if we can directly inline enum fields with the rest of the enum.
        // This requires supporting enums with fields or not
        if len == 1 || len == 2 {
            return Some(
                enum_
                    .variants
                    .iter()
                    .map(|v| v.variant_name.to_string())
                    .collect::<Vec<_>>()
                    .join("Or"),
            );
        }
    }
    // For some types, infer the variant name by the type of the parameter itself. For now,
    // we just ignore types like Vec, List, etc. when doing this
    match rust_type {
        RustType::Simple(typ) => Some(format!("{typ}")),
        _ => None,
    }
}

pub fn build_struct_field<T: Borrow<Schema>>(
    base_rust_type: RustType,
    field_name: &str,
    field: &ReferenceOr<T>,
    required: bool,
) -> StructField {
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    let maybe_schema = field.as_item().map(|s| s.borrow());
    let is_nullable = maybe_schema.map(|s| s.schema_data.nullable).unwrap_or_default();
    let rust_type =
        if !required || is_nullable { base_rust_type.as_nullable() } else { base_rust_type };
    let mut struct_field = StructField::new(&field_rename, rust_type.clone());
    if let Some(doc) = field.as_item().and_then(|s| s.borrow().schema_data.description.as_deref()) {
        struct_field = struct_field.doc(doc);
    }
    if field_rename != field_name {
        struct_field = struct_field.rename_as(field_name);
    }
    if !required {
        if let Some(skip_ser) = rust_type.as_skip_serializing() {
            struct_field = struct_field.skip_serializing(skip_ser);
        }
        if let Some(default) = rust_type.as_deser_default() {
            struct_field = struct_field.deser_default(default);
        }
    }
    struct_field
}
