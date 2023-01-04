use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Write as _;

use crate::ident::RustIdent;
use crate::rust_type::{PrintableType, RustType};
use crate::schema_path::ComponentPath;
use crate::stripe_object::RequestSpec;
use crate::templates::derives::Derive;
use crate::types::RustObject;
use crate::util::write_to_file;

#[derive(Debug, Clone)]
pub struct Component {
    pub path: ComponentPath,
    pub object: RustObject,
    pub id_type: Option<RustIdent>,
    pub requests: Vec<RequestSpec>,
}

pub struct ComponentGenerator {
    pub component: Component,
    pub objects: VecDeque<(RustIdent, RustObject)>,
    out: String,
}

impl ComponentGenerator {
    pub fn new(component: Component) -> Self {
        Self { component, objects: Default::default(), out: String::new() }
    }

    pub fn write_file_code(&mut self) -> anyhow::Result<()> {
        self.gen_code();
        self.write_file()
    }

    fn gen_object(&mut self, ident: RustIdent, obj: RustObject) {
        match obj {
            RustObject::Struct(struct_) => {
                let mut struct_derives = vec![Derive::Deserialize, Derive::Serialize];
                if struct_.fields.iter().all(|field| field.rust_type.is_option()) {
                    struct_derives.push(Derive::Default);
                }
                self.write_struct(&struct_, &ident, &struct_derives);
            }
            RustObject::Enum(enum_) => {
                self.write_string(enum_.gen_definition_and_methods(&ident));
            }
            RustObject::FieldedEnum(enum_) => {
                self.write_fielded_enum(&enum_, &ident, &[Derive::Serialize, Derive::Deserialize]);
            }
        }
    }

    fn add_object(&mut self, ident: RustIdent, obj: RustObject) {
        self.objects.push_back((ident, obj));
    }

    fn gen_code(&mut self) {
        self.add_object(RustIdent::create(&self.component.path), self.component.object.clone());

        // Just collecting the requests instead of printing so we can place them at the end
        // of the file
        let mut reqs = vec![];
        for req in self.component.requests.clone() {
            reqs.push(self.gen_request(req));
        }

        while let Some((rust_ident, obj)) = self.objects.pop_front() {
            self.gen_object(rust_ident, obj);
        }

        for req in reqs {
            self.write_string(req);
        }
    }

    fn write_file(&self) -> anyhow::Result<()> {
        let file_path = format!("{}.rs", self.component.path);
        write_to_file(self.out.as_ref(), &file_path)?;
        Ok(())
    }

    pub(crate) fn make_type_printable(
        &mut self,
        rust_type: &RustType,
        ident: RustIdent,
    ) -> PrintableType {
        if let Some(obj) = rust_type.as_rust_obj() {
            let printable = rust_type.to_printable_with_object_name(ident.clone());
            self.add_object(ident, obj.clone());
            printable
        } else {
            rust_type.to_printable().expect("Type should not have objects in it")
        }
    }

    pub(crate) fn write_string<T: Display>(&mut self, val: T) {
        let _ = writeln!(self.out, "{val}");
    }
}
