use indoc::formatdoc;

use crate::component_generator::ComponentGenerator;
use crate::ident::RustIdent;
use crate::templates::derives::{write_derives_line, Derive};
use crate::types::{RustStruct, StructField};
use crate::util::{write_doc_comment, write_serde_rename};

impl ComponentGenerator {
    pub fn gen_struct_field(&mut self, field: &StructField, ident: &RustIdent) -> String {
        let mut lines = vec![];
        if let Some(doc_comment) = &field.doc_comment {
            // For simplicity, strip newline from doc comment since we join with newlines later
            lines.push(write_doc_comment(doc_comment, 1).trim_end().to_string());
        }
        if let Some(renamer) = &field.rename_as {
            lines.push(write_serde_rename(renamer));
        }
        if let Some(skip_serializing) = field.skip_serializing {
            lines.push(format!(r#"#[serde(skip_serializing_if = "{skip_serializing}")]"#));
        }
        if let Some(default) = field.deser_default {
            lines.push(default.to_serde_attr());
        }

        let printable =
            self.make_type_printable(&field.rust_type, RustIdent::joined(ident, &field.field_name));
        lines.push(format!("pub {}: {printable},", field.field_name));
        lines.join("\n")
    }

    pub fn write_struct(
        &mut self,
        struct_: &RustStruct,
        struct_name: &RustIdent,
        additional_derives: &[Derive],
    ) {
        let doc_comment = if let Some(doc) = &struct_.doc_comment {
            write_doc_comment(doc, 0)
        } else {
            String::new()
        };
        let derives = write_derives_line(additional_derives);
        let fields = struct_
            .fields
            .iter()
            .map(|field| self.gen_struct_field(field, struct_name))
            .collect::<Vec<_>>()
            .join("\n\n");
        let disp = formatdoc!(
            r"
        {doc_comment}{derives}
        pub struct {struct_name} {{
        {fields}
        }}
    "
        );
        self.write_string(disp);
    }
}
