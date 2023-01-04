use std::fmt::Write as _;

use indoc::{formatdoc, writedoc};

use crate::component_generator::ComponentGenerator;
use crate::ident::RustIdent;
use crate::templates::derives::{write_derives_line, Derive};
use crate::types::{FieldedEnumVariant, RustFieldedEnum};
use crate::util::{write_doc_comment, write_serde_rename};

impl ComponentGenerator {
    pub fn write_fielded_enum(
        &mut self,
        enum_: &RustFieldedEnum,
        enum_name: &RustIdent,
        additional_derives: &[Derive],
    ) {
        // Build the body of the enum definition
        let mut enum_body = String::new();
        for FieldedEnumVariant { variant, rust_type, rename_as, .. } in &enum_.variants {
            if let Some(rename) = rename_as {
                let _ = writeln!(enum_body, "{}", write_serde_rename(rename));
            }
            let printable =
                self.make_type_printable(rust_type, RustIdent::joined(enum_name, variant));
            let _ = writeln!(enum_body, "{variant}({printable}),");
        }
        let doc_comment = write_doc_comment(enum_.doc_comment.as_deref().unwrap_or_default(), 0);
        let trimmed_doc = doc_comment.trim();
        let derives = write_derives_line(additional_derives);
        let mut out = formatdoc!(
            r#"
            
            {trimmed_doc}
            {derives}
            #[serde(untagged, rename_all = "snake_case")]
            pub enum {enum_name} {{
            {enum_body}
            }}
        "#
        );

        if let Some(variant) = &enum_.default_variant {
            let _ = writedoc!(
                out,
                r"
                impl std::default::Default for {enum_name} {{
                    fn default() -> Self {{
                        Self::{variant}(Default::default())
                    }}
                }}
                "
            );
        };
        self.write_string(out);
    }
}
