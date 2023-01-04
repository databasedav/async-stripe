use heck::SnakeCase;
use openapiv3::Schema;
use serde::{Deserialize, Serialize};

use crate::ident::RustIdent;
use crate::rust_type::{RustType, SimpleType};
use crate::schema_path::ComponentPath;

#[derive(Debug, Clone)]
pub struct StripeObject {
    pub id: Option<RustIdent>,
    pub operations: Vec<StripeOperation>,
    pub schema: Schema,
    pub path: ComponentPath,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StripeOperation {
    pub method_name: String,
    pub method_type: MethodType,
    pub method_on: String,
    #[serde(rename = "operation")]
    pub operation_type: OperationType,
    pub path: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MethodType {
    Retrieve,
    List,
    Create,
    Update,
    Delete,
    Custom,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub path_params: Vec<PathParam>,
    pub description: Option<String>,
    pub operation_id: String,
    pub operation: StripeOperation,
    pub params: RequestParams,
    pub returned: RustType,
}

#[derive(Debug, Clone)]
pub enum RequestParams {
    Delete,
    Post { body: RustType },
    Get { query: RustType },
}

impl RequestParams {
    pub fn as_type(&self) -> Option<&RustType> {
        match self {
            Self::Post { body } => Some(body),
            Self::Get { query } => Some(query),
            Self::Delete => None,
        }
    }
}

impl RequestSpec {
    pub fn func_name(&self) -> String {
        self.operation_id.to_snake_case()
    }
}

#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: String,
    pub rust_type: SimpleType,
}

#[derive(Debug, Clone)]
pub struct QueryParam {
    pub name: String,
    pub rust_type: RustType,
}
