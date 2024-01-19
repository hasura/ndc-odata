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

impl Schema {
    /// Look up a collection by name within this schema.
    pub fn collection(&self, name: &str) -> Option<&super::Collection> {
        self.collections
            .iter()
            .find(|collection| collection.name == name)
    }

    /// Look up a function by name within this schema.
    pub fn function(&self, name: &str) -> Option<&super::Function> {
        self.functions.iter().find(|function| function.name == name)
    }

    /// Look up a procedure by name within this schema.
    pub fn procedure(&self, name: &str) -> Option<&super::Procedure> {
        self.procedures
            .iter()
            .find(|procedure| procedure.name == name)
    }
}
