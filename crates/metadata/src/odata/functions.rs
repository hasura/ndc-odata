//! Structures for describing actions in the OData specification.

use serde::Deserialize;

/// Actions are OData's answer to the NDC notion of commands.
/// @TODO: actions don't necessarily return values; do we need to use e.g. `true` for GraphQL?
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Function {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "Parameter")]
    pub parameters: Vec<Parameter>,

    #[serde(rename = "ReturnType")]
    pub return_type: super::TypeData,

    #[serde(rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,

    // TODO: is this important to us?
    #[serde(default)]
    #[serde(rename = "@IsBound")]
    #[serde(deserialize_with = "super::helpers::str_to_bool")]
    pub is_bound: bool,
}

/// A parameter for a function or action.
#[derive(Clone, Debug, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: super::TypeData,
}

/// A declaration that the given function is available within the parent entity container.
#[derive(Clone, Debug, Deserialize)]
pub struct FunctionImport {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Function")]
    pub function: String,

    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,
}
