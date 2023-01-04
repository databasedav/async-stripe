use indoc::formatdoc;

use crate::component_generator::ComponentGenerator;
use crate::ident::RustIdent;

impl ComponentGenerator {
    pub fn write_object_trait(&mut self) {
        let assoc_type = if let Some(id_type) = &self.component.id_type {
            format!("crate::ids::{id_type}")
        } else {
            "()".into()
        };
        let id_body = if assoc_type != "()" { "self.id.clone()" } else { "" };
        let impl_for = RustIdent::create(&self.component.path);
        let out = formatdoc!(
            r#"
            impl crate::Object for {impl_for} {{
                type Id = {assoc_type};
                fn id(&self) -> Self::Id {{
                    {id_body}
                }}
                
                fn object(&self) -> &'static str {{
                    todo!()
                }}
            }}
            "#
        );
        self.write_string(out);
    }
}
