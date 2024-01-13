//! Structures for describing actions in the OData specification.

use serde::Deserialize;

/// Actions are OData's answer to the NDC notion of commands.
/// @TODO: actions don't necessarily return values; do we need to use e.g. `true` for GraphQL?
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    #[serde(rename = "@Name")]
    pub name: String,

    // TODO: is this important to us?
    #[serde(default)]
    #[serde(rename = "@IsBound")]
    #[serde(deserialize_with = "super::helpers::str_to_bool")]
    pub is_bound: bool,

    #[serde(default)]
    #[serde(rename = "Parameter")]
    pub parameters: Vec<super::Parameter>,

    #[serde(rename = "ReturnType")]
    pub return_type: Option<super::TypeData>,

    #[serde(rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,
}

/// A declaration that the given action is available within the parent entity container.
#[derive(Clone, Debug, Deserialize)]
pub struct ActionImport {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Action")]
    pub action: String,

    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,
}
