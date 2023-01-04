use openapiv3::{Operation, Parameter, ParameterSchemaOrContent, ReferenceOr};

use crate::rust_type::RustType;
use crate::spec::{get_ok_response_schema, get_request_form_parameters, Spec};
use crate::spec_inference::{build_struct_field, infer_schema_or_ref_type};
use crate::stripe_object::{
    OperationType, PathParam, RequestParams, RequestSpec, StripeObject, StripeOperation,
};
use crate::types::{RustObject, RustObjectBuilder};

/// Should we skip a currently unsupported request?
fn should_skip_request(op: &StripeOperation) -> bool {
    // TODO: what is the relevance of the `method_on` field? A small number of requests
    // use "collection" instead of "service", but the OpenAPI schema does not differentiate
    // so we just end up with duplicate requests if we don't skip like this
    op.method_on != "service"
        // Skip PDF download (binary format response not supported by client yet)
        || op.method_name == "pdf"
        // Skip file upload (form/multipart not supported by client yet)
        || (op.method_name == "create" && op.path == "/v1/files")
}

impl StripeObject {
    pub fn build_requests(&self, spec: &Spec) -> Vec<RequestSpec> {
        let mut requests = vec![];
        for op in &self.operations {
            if should_skip_request(op) {
                log::warn!("Skipping request at path {} with name {}", op.path, op.method_name);
                continue;
            }
            let path_item = spec.get_request_unwrapped(&op.path);
            let ReferenceOr::Item(item) = path_item else { panic!("Expected item") };
            let operation = match op.operation_type {
                OperationType::Get => &item.get,
                OperationType::Post => &item.post,
                OperationType::Delete => &item.delete,
            }
            .as_ref()
            .expect("Operation not found");
            requests.push(build_request(op, operation));
        }
        requests
    }
}

fn build_request(op_spec: &StripeOperation, operation: &Operation) -> RequestSpec {
    let op_id = operation.operation_id.as_ref().expect("No operation id");
    let Some(return_schema) = get_ok_response_schema(operation) else {
        panic!("Expected schema for 200 response on operation {:?}", op_spec)
    };
    let return_type = infer_schema_or_ref_type(return_schema, None);
    let op_desc = operation.description.clone();
    let mut path_params = vec![];
    let mut query_struct_fields = vec![];

    for param in &operation.parameters {
        let ReferenceOr::Item(param) = param else { panic!("Expected item") };
        match param {
            Parameter::Query { parameter_data, .. } => {
                let ParameterSchemaOrContent::Schema(schema) = &parameter_data.format else { panic!("Expected schema") };
                let typ = infer_schema_or_ref_type(schema, Some(&parameter_data.name));
                let struct_field =
                    build_struct_field(typ, &parameter_data.name, schema, parameter_data.required);
                query_struct_fields.push(struct_field);
            }
            Parameter::Path { parameter_data, .. } => {
                let ParameterSchemaOrContent::Schema(schema) = &parameter_data.format else { panic!("Expected schema") };
                let param_typ = infer_schema_or_ref_type(schema, Some(&parameter_data.name));

                let RustType::Simple(typ) = param_typ else { panic!("Unexpected path type inferred: {:?}", param_typ)};

                path_params.push(PathParam { name: parameter_data.name.clone(), rust_type: typ });
            }
            _ => panic!("Unexpected parameter type"),
        }
    }
    let form_params = get_request_form_parameters(operation);
    let params = match op_spec.operation_type {
        OperationType::Get => {
            assert!(form_params.is_none(), "Body parameters not supported for GET");
            assert!(!query_struct_fields.is_empty(), "Found no query params for GET");
            let mut rust_struct = RustObjectBuilder::new().start_struct();
            for field in query_struct_fields {
                rust_struct.add_field(field);
            }
            RequestParams::Get { query: RustType::Object(RustObject::Struct(rust_struct)) }
        }
        OperationType::Post => {
            assert!(query_struct_fields.is_empty(), "Did not expected query parameters for POST");
            let Some(form_params) = form_params else {
                panic!("POST request should have params: not found for {:?}", op_spec);
            };
            RequestParams::Post { body: infer_schema_or_ref_type(form_params, None) }
        }
        OperationType::Delete => {
            assert!(
                query_struct_fields.is_empty(),
                "Query parameters not supported for DELETE at {:?}",
                op_spec,
            );
            if form_params.is_some() {
                log::warn!("Ignoring body parameters for DELETE at path {}", op_spec.path);
            }
            RequestParams::Delete
        }
    };
    RequestSpec {
        path_params,
        description: op_desc,
        params,
        returned: return_type,
        operation: op_spec.clone(),
        operation_id: op_id.clone(),
    }
}
