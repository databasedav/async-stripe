use std::fmt::Write as _;

use heck::SnakeCase;
use indoc::{formatdoc, writedoc};

use crate::ident::RustIdent;
use crate::templates::derives::{write_derives_line, Derive};
use crate::types::{EnumVariant, RustEnum};
use crate::util::{write_doc_comment, write_serde_rename};

// What we derive for a simple enum, no flexibility for now
const EXTRA_ENUM_DERIVES: [Derive; 5] =
    [Derive::Copy, Derive::Eq, Derive::PartialEq, Derive::Deserialize, Derive::Serialize];

impl RustEnum {
    pub fn gen_definition_and_methods(&self, enum_name: &RustIdent) -> String {
        // Build the body of the enum definition
        let mut enum_def_body = String::new();
        for EnumVariant { wire_name, variant_name } in &self.variants {
            if &variant_name.to_snake_case() != wire_name {
                let _ = writeln!(enum_def_body, "{}", write_serde_rename(wire_name));
            }
            let _ = writeln!(enum_def_body, "{variant_name},");
        }

        // Build the body of the `as_str` implementation
        let as_str_body = self
            .variants
            .iter()
            .map(|EnumVariant { wire_name, variant_name }| {
                format!(r#"Self::{variant_name} => "{wire_name}","#)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let doc_comment = if let Some(doc) = &self.doc_comment {
            write_doc_comment(doc, 0)
        } else {
            String::new()
        };
        let trimmed_doc_comment = doc_comment.trim_end();
        let derives = write_derives_line(&EXTRA_ENUM_DERIVES);
        let mut out = formatdoc!(
            r#"
            
            {trimmed_doc_comment}
            {derives}
            #[serde(rename_all = "snake_case")]
            pub enum {enum_name} {{
            {enum_def_body}
            }}

            impl {enum_name} {{
                pub fn as_str(self) -> &'static str {{
                    match self {{
            {as_str_body}
                    }}
                }}
            }}

            impl AsRef<str> for {enum_name} {{
                fn as_ref(&self) -> &str {{
                    self.as_str()
                }}
            }}

            impl std::fmt::Display for {enum_name} {{
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                    self.as_str().fmt(f)
                }}
            }}
        "#
        );

        if let Some(default_variant) = &self.default_variant {
            let _ = writedoc!(
                out,
                r"
                
            impl Default for {enum_name} {{
                fn default() -> Self {{
                    Self::{default_variant}
                }}
            }}"
            );
        }
        out
    }
}
