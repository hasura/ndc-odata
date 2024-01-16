//! Entity sets are the OData analogue to NDC "collections". However, OData also has entity
//! containers: a sub-schema sort of namespacing that houses some entity sets, as well as a set of
//! functions and actions. Entity containers may also contain singletons, which are probably best
//! mapped to the NDC notion of functions: they return exactly one element of the given type. The
//! example used in the reference API that I found helpful is the singleton "Me", which returns the
//! row of type "People" corresponding to the current user.

use serde::Deserialize;

/// An entity set is the OData anologue to an NDC collection. It is named, has a row type described
/// by a named `EntityType`, and some "relationships". An `EntityType` contains some number of
/// `NavigationProperty` elements, and the entity set can choose to bind some of these to other
/// entity sets. We conceptualise the bound navigation properties as foreign key relationships
/// within the NDC.
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

/// A declaration that a indicates that a particular navigation property should be resolved to a
/// particular entity set. In RDBMS terms, we're defining a foreign key relationship from this
/// entity to the primary key of another (although OData doesn't expose the "key" to us). If the
/// navigation property type is `T` or `Collection(T)`, then the entity type of the entity set we
/// bind as a target must have type `T`.
#[derive(Clone, Debug, Deserialize)]
pub struct NavigationPropertyBinding {
    // @TODO: how do paths actually work? If I have some type `Person`, some subtype `Employee`,
    // and I bind an entity set `People` of type `Person` to have an `Employee` navigation
    // property, do I get a relationship only for any `Person` I can cast to an `Employee`?
    #[serde(rename = "@Path")]
    pub path: String,

    #[serde(rename = "@Target")]
    pub target: String,
}

/// An entity type describes the type of a singular entity in the API. This is analogous to the
/// type of a row in a table (an entity set). Entity types can be keyed (unlike compex types), and
/// may extend another entity type, which means they inherit all the fields and navigation
/// properties of the underlying type.
#[derive(Clone, Debug, Deserialize)]
pub struct EntityType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "Key")]
    pub key: Option<Key>,

    #[serde(rename = "@BaseType")]
    pub base_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "Property")]
    pub properties: Vec<super::Property>,

    #[serde(default)]
    #[serde(rename = "NavigationProperty")]
    pub navigation_properties: Vec<super::NavigationProperty>,
}

impl EntityType {
    pub fn key(&self, schema: &super::schema::Schema) -> String {
        if let Some(key) = &self.key {
            key.property_ref.name.clone()
        } else if let Some(base) = &self.base_type {
            schema.entity_type(base).unwrap().key(schema)
        } else {
            // @TODO: this is a bit of a shame; a better approach would be to parse the metadata
            // XML, then traverse the graph to verify some invariants (including this one), and
            // maybe during that process we can replace every `Option<Key>` with a `Key`, entirely
            // removing the need for this check to be here.
            panic!(
                "Entity type {} has neither a key nor a base type.",
                self.name
            )
        }
    }
}

/// The "key" of an entity type. This is the unique identifier of any given resource within the
/// entity set, and we can think of it as a primary key.
/// @TODO: can we have multiple keys? If so, are we saying that the combination of those keys must
///        be unique, or that the entity set has two unique indices?
#[derive(Clone, Debug, Deserialize)]
pub struct Key {
    #[serde(rename = "PropertyRef")]
    pub property_ref: PropertyRef,
}

/// The property on the entity type that we're using as our key.
#[derive(Clone, Debug, Deserialize)]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}

/// An entity container describes the available API given the defined entity types, functions, and
/// actions. Specifically, in the parlance of NDCs, this means that it describes the collections
/// (entity sets), functions (functions and singletons), and procedures available within this
/// particular schema.
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
    pub function_imports: Vec<super::FunctionImport>,

    #[serde(default)]
    #[serde(rename = "ActionImport")]
    pub action_imports: Vec<super::ActionImport>,
}

/// Singletons are conceptually equivalent to nullary functions within the NDC vocabulary: they are
/// defined at the entity container level, and return a singular row.
///
/// @TODO: are they necessarily nullary?
#[derive(Clone, Debug, Deserialize)]
pub struct Singleton {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: super::TypeData,

    #[serde(default)]
    #[serde(rename = "NavigationPropertyBinding")]
    pub navigation_property_bindings: Vec<NavigationPropertyBinding>,
}
