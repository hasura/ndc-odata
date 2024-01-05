pub mod url;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Configuration {
    pub api_endpoint: url::Endpoint,
    pub schema: Schema,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct RawConfiguration {
    pub api_endpoint: String,

    #[serde(default)]
    pub schema: Schema,
}

impl Default for RawConfiguration {
    fn default() -> Self {
        RawConfiguration {
            api_endpoint: Default::default(),
            schema: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Schema {
    #[serde(default)]
    pub collections: Vec<Collection>,
    #[serde(default)]
    pub scalar_types: BTreeSet<String>,
    #[serde(default)]
    pub object_types: BTreeMap<String, ObjectType>,
    #[serde(default)]
    pub functions: Vec<Function>,
    #[serde(default)]
    pub procedures: Vec<Procedure>,
}

impl Default for Schema {
    fn default() -> Self {
        Schema {
            collections: Default::default(),
            object_types: Default::default(),
            scalar_types: Default::default(),

            functions: Default::default(),
            procedures: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Collection {
    pub name: String,
    pub key: Option<String>,
    pub collection_type: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct ObjectType {
    #[serde(flatten)]
    pub fields: BTreeMap<String, Type>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: BTreeMap<String, Type>,
    pub result_type: Type,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Procedure {
    pub name: String,
    pub arguments: BTreeMap<String, Type>,
    pub result_type: Type,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(tag = "type")]
pub enum Type {
    Collection { element_type: Box<Type> },
    Nullable { underlying_type: Box<Type> },
    Named { name: String },
}
