use std::collections::HashMap;
use std::fmt::Write as _;

use openapiv3::{ReferenceOr, Schema, SchemaKind, Type};
use petgraph::prelude::Bfs;

use crate::component_generator::{Component, ComponentGenerator};
use crate::ident::RustIdent;
use crate::mappings::ID_RENAMES;
use crate::schema_path::ComponentPath;
use crate::spec::{as_object_type, Spec};
use crate::spec_inference::{build_fielded_enum, build_struct_field, infer_schema_or_ref_type};
use crate::stripe_object::{StripeObject, StripeOperation};
use crate::types::{RustObject, RustObjectBuilder};
use crate::url_finder::UrlFinder;
use crate::util::write_to_file;

impl StripeObject {
    pub fn gen_object(&self, doc_url: Option<String>) -> RustObject {
        let mut doc_comment = if let Some(descr) = &self.schema.schema_data.description {
            descr.clone()
        } else if let Some(title) = &self.schema.schema_data.title {
            format!("The resource representing a Stripe {title}")
        } else {
            String::new()
        };
        if let Some(doc) = doc_url {
            let _ = writeln!(doc_comment, "\n\nFor more details see <{doc}>");
        }

        let rust_obj = match &self.schema.schema_kind {
            SchemaKind::AnyOf { any_of } => {
                let object_builder = RustObjectBuilder::new().doc(doc_comment);
                let enum_ =
                    build_fielded_enum(any_of, None, object_builder).expect("Could not build enum");
                RustObject::FieldedEnum(enum_)
            }
            SchemaKind::Type(Type::Object(obj)) => {
                // Generate the struct type
                let mut rust_struct = RustObjectBuilder::new().doc(doc_comment).start_struct();
                for (field_name, field_spec) in &obj.properties {
                    if field_name == "object" {
                        continue;
                    }
                    let is_required = obj.required.contains(field_name);
                    let rust_type = infer_schema_or_ref_type(field_spec, Some(field_name));
                    let field = build_struct_field(rust_type, field_name, field_spec, is_required);
                    rust_struct.add_field(field);
                }
                RustObject::Struct(rust_struct)
            }
            _ => panic!("Unexpected top level schema at path {}", self.path),
        };
        rust_obj
    }
}

pub struct CodeGen {
    spec: Spec,
    url_finder: UrlFinder,
    pub(crate) components: HashMap<ComponentPath, Component>,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder) -> Self {
        let mut codegen = Self { spec, url_finder, components: Default::default() };
        codegen.build_components();
        codegen
    }

    fn build_components(&mut self) {
        let mut objects = vec![];
        for (path, maybe_schema) in self.spec.component_schemas() {
            let ReferenceOr::Item(schema) = maybe_schema else { panic!("Did not expect top level ref") };
            let stripe_reqs: Vec<StripeOperation> =
                if let Some(stripe_ops) = schema.schema_data.extensions.get("x-stripeOperations") {
                    serde_json::from_value(stripe_ops.clone())
                        .expect("Could not deserialize stripe operations")
                } else {
                    vec![]
                };

            objects.push(StripeObject {
                id: extract_obj_id(path, schema),
                operations: stripe_reqs,
                schema: schema.clone(),
                path: ComponentPath::new(path),
            });
        }
        for obj in objects {
            log::info!("Generating component at path {}", obj.path);
            let doc_url = self.url_finder.url_for_object(&obj.path);
            let base_object = obj.gen_object(doc_url);
            let requests = obj.build_requests(&self.spec);
            self.components.insert(
                obj.path.clone(),
                Component { path: obj.path, object: base_object, id_type: obj.id, requests },
            );
        }
    }

    fn get_component_dependencies(&self, component: ComponentPath) -> Vec<ComponentPath> {
        let dep_graph = self.gen_dep_graph();
        let index = dep_graph.node_indices().find(|i| dep_graph[*i] == &component).unwrap();
        let mut bfs = Bfs::new(&dep_graph, index);
        let mut dependencies = vec![];
        while let Some(nx) = bfs.next(&dep_graph) {
            dependencies.push(dep_graph[nx].clone());
        }
        dependencies
    }

    pub fn write_files(&self, single_object: Option<&String>) -> anyhow::Result<()> {
        let objects_to_write = if let Some(single) = single_object {
            self.get_component_dependencies(ComponentPath::new(single))
        } else {
            self.components.keys().cloned().collect()
        };

        for obj in &objects_to_write {
            let component = &self.components[obj];
            let mut component_gen = ComponentGenerator::new(component.clone());
            component_gen.write_file_code()?;
        }
        let mod_rs = objects_to_write
            .into_iter()
            .map(|path| format!("pub mod {path};pub use {path}::{};", RustIdent::create(&path)))
            .collect::<Vec<_>>()
            .join("");
        write_to_file(mod_rs.as_ref(), "mod.rs")?;
        Ok(())
    }
}

fn extract_obj_id(schema_path: &str, schema: &Schema) -> Option<RustIdent> {
    let Some(obj_type) = as_object_type(schema) else { return None;};
    if !obj_type.properties.contains_key("id") {
        return None;
    }
    let id_type = ID_RENAMES.get(schema_path).unwrap_or(&schema_path);
    Some(RustIdent::create(&format!("{id_type}_id")))
}
