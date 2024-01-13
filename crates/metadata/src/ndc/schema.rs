//! The core schema type.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct Schema {
    #[serde(default)]
    pub collections: Vec<super::Collection>,
    #[serde(default)]
    pub scalar_types: BTreeSet<super::ScalarType>,
    #[serde(default)]
    pub object_types: BTreeMap<String, super::ObjectType>,
    #[serde(default)]
    pub functions: Vec<super::Function>,
    #[serde(default)]
    pub procedures: Vec<super::Procedure>,
}
