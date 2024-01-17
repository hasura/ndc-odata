//! The overall schema type for OData metadata.

use serde::Deserialize;

/// The schema of an OData API, as described in the `/$metadata` endpoint. From this, we derive the
/// entire schema for the NDC.
#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    #[serde(rename = "@Namespace")]
    pub namespace: String,

    #[serde(rename = "@xmlns")]
    pub xmlns: Option<String>,

    #[serde(default)]
    #[serde(rename = "EntityType")]
    pub entity_types: Vec<super::EntityType>,

    #[serde(default)]
    #[serde(rename = "ComplexType")]
    pub complex_types: Vec<super::ComplexType>,

    #[serde(default)]
    #[serde(rename = "EnumType")]
    pub enum_types: Vec<super::EnumType>,

    #[serde(default)]
    #[serde(rename = "Function")]
    pub functions: Vec<super::Function>,

    #[serde(default)]
    #[serde(rename = "Action")]
    pub actions: Vec<super::Action>,

    #[serde(rename = "EntityContainer")]
    pub entity_container: super::EntityContainer,
}

impl Schema {
    /// Look up a complex type by name within this schema.
    pub fn complex_type(&self, name: &str) -> Option<super::ComplexType> {
        self.complex_types
            .iter()
            .find(|complex_type| complex_type.name == name)
            .cloned()
    }

    /// Look up an entity type by name within this schema.
    pub fn entity_set(&self, name: &str) -> Option<super::EntitySet> {
        self.entity_container
            .entity_sets
            .iter()
            .find(|entity_set| entity_set.name == name)
            .cloned()
    }

    /// Look up an entity type by name within this schema.
    pub fn entity_type(&self, name: &str) -> Option<super::EntityType> {
        self.entity_types
            .iter()
            .find(|entity_type| entity_type.name == name)
            .cloned()
    }

    /// Look up an enum type by name within this schema.
    pub fn enum_type(&self, name: &str) -> Option<super::EnumType> {
        self.enum_types
            .iter()
            .find(|enum_type| enum_type.name == name)
            .cloned()
    }

    /// Look up a function by name within this schema.
    /// @TODO: we should really be looking this up in the entity container to check it is exposed
    ///        in the API, and _then_ get the content.
    pub fn function(&self, name: &str) -> Option<super::Function> {
        self.functions
            .iter()
            .find(|function| function.name == name)
            .cloned()
    }

    /// Look up an action by name within this schema.
    /// @TODO: we should really be looking this up in the entity container to check it is exposed
    ///        in the API, and _then_ get the content.
    pub fn action(&self, name: &str) -> Option<super::Action> {
        self.actions
            .iter()
            .find(|action| action.name == name)
            .cloned()
    }
}
