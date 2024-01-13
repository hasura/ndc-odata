//! Enum types, as defined in the OData metadata.

use serde::Deserialize;

/// An enum type defines a variant, with each (nullary) constructor mapped to an integer value.
/// @TODO: how are these represented in requests/responses? Do we have to do any mapping?
#[derive(Clone, Debug, Deserialize)]
pub struct EnumType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "Member")]
    pub members: Vec<Member>,
}

/// A member of an enum type: a name, along with a corresponding integer value.
#[derive(Clone, Debug, Deserialize)]
pub struct Member {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Value")]
    pub value: u32,
}
