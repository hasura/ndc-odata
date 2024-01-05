//! A representation of (some of) the OData Common Schema Definition Language (CSDL). This is by no
//! means complete, but it will be expanded as more of the schema becomes necessary.

pub mod types;

use crate::odata::types::Type;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
pub struct EDMX {
    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@edmx")]
    pub edmx: Option<String>,

    #[serde(rename = "DataServices")]
    pub data_services: DataServices,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DataServices {
    #[serde(default)]
    #[serde(rename = "Schema")]
    pub schema: Vec<Schema>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    #[serde(rename = "@Namespace")]
    pub namespace: String,

    #[serde(default)]
    #[serde(rename = "EntityType")]
    pub entity_types: Vec<EntityType>,

    #[serde(default)]
    #[serde(rename = "ComplexType")]
    pub complex_types: Vec<ComplexType>,

    #[serde(default)]
    #[serde(rename = "EnumType")]
    pub enum_types: Vec<EnumType>,

    #[serde(default)]
    #[serde(rename = "Function")]
    pub functions: Vec<Function>,

    #[serde(default)]
    #[serde(rename = "Action")]
    pub actions: Vec<Action>,

    #[serde(rename = "EntityContainer")]
    pub entity_container: EntityContainer,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EntityType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "Key")]
    pub key: Option<Key>,

    #[serde(default)]
    #[serde(rename = "Property")]
    pub properties: Vec<Property>,

    #[serde(default)]
    #[serde(rename = "NavigationProperty")]
    pub navigation_properties: Vec<NavigationProperty>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Key {
    #[serde(rename = "PropertyRef")]
    pub property_ref: PropertyRef,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: TypeDescription,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NavigationProperty {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: TypeDescription,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@BaseType")]
    pub base_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "Property")]
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnumType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "Member")]
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Member {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Value")]
    pub value: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Function {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "Parameter")]
    pub parameters: Vec<Parameter>,

    #[serde(rename = "ReturnType")]
    pub return_type: ReturnType,

    #[serde(rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,

    #[serde(default)]
    #[serde(rename = "@IsBound")]
    #[serde(deserialize_with = "str_to_bool")]
    pub is_bound: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: TypeDescription,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReturnType {
    #[serde(flatten)]
    pub r#type: TypeDescription,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "@IsBound")]
    #[serde(deserialize_with = "str_to_bool")]
    pub is_bound: bool,

    #[serde(default)]
    #[serde(rename = "Parameter")]
    pub parameters: Vec<Parameter>,

    #[serde(rename = "ReturnType")]
    pub return_type: Option<ReturnType>,

    #[serde(rename = "EntitySetPath")]
    pub entity_set_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EntityContainer {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "EntitySet")]
    pub entity_sets: Vec<EntitySet>,

    #[serde(default)]
    #[serde(rename = "Singleton")]
    pub singletons: Vec<Singleton>,

    #[serde(default)]
    #[serde(rename = "FunctionImport")]
    pub function_imports: Vec<FunctionImport>,

    #[serde(default)]
    #[serde(rename = "ActionImport")]
    pub action_imports: Vec<ActionImport>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EntitySet {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@EntityType")]
    pub entity_type: String,

    #[serde(default)]
    #[serde(rename = "NavigationPropertyBinding")]
    pub navigation_property_bindings: Vec<NavigationPropertyBinding>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NavigationPropertyBinding {
    #[serde(rename = "@Path")]
    pub path: String,

    #[serde(rename = "@Target")]
    pub target: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Singleton {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: TypeDescription,

    #[serde(default)]
    #[serde(rename = "NavigationPropertyBinding")]
    pub navigation_property_bindings: Vec<NavigationPropertyBinding>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FunctionImport {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Function")]
    pub function: String,

    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActionImport {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Action")]
    pub action: String,

    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDescription {
    #[serde(rename = "@Type")]
    pub inner: Type,

    #[serde(default = "r#true")]
    #[serde(rename = "@Nullable")]
    #[serde(deserialize_with = "str_to_bool")]
    pub nullable: bool,
}

fn r#true() -> bool {
    true
}

fn str_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let bool_string = String::deserialize(deserializer)?;
    Ok(bool_string.to_lowercase().trim() == "true") // I have a feeling there'll be more...
}
