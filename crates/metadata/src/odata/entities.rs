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
    pub entity_type: super::QualifiedType,

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
    #[serde(rename = "@Path")]
    pub path: String,

    #[serde(rename = "@Target")]
    pub target: String,
}

/// An entity type describes the type of a singular entity in the API. This is analogous to the
/// type of a row in a table (an entity set). Entity types can be keyed (unlike compex types), and
/// may extend another entity type, which means they inherit all the fields and navigation
/// properties of the underlying type.
///
/// @TODO: it would be nice, internally, to reuse `ComplexType` and `flatten` the structure with
/// `serde`, but `serde` didn't like this. We'll come back to it.
#[derive(Clone, Debug, Deserialize)]
pub struct EntityType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "Key")]
    pub key: Option<Key>,

    #[serde(rename = "@BaseType")]
    pub base_type: Option<super::QualifiedType>,

    #[serde(default)]
    #[serde(rename = "Property")]
    pub properties: Vec<super::Property>,

    #[serde(default)]
    #[serde(rename = "NavigationProperty")]
    pub navigation_properties: Vec<super::NavigationProperty>,
}

impl EntityType {
    /// Get all the fields from the type and the chain of base types.
    pub fn fields(&self, metadata: &super::EDMX) -> Vec<super::Property> {
        let mut collection = Vec::new();
        collection.append(&mut self.properties.clone());

        if let Some(target) = self.base_type.as_ref().and_then(|x| metadata.entity_type(&x)) {
            collection.append(&mut target.fields(&metadata))
        }

        if let Some(target) = self.base_type.as_ref().and_then(|x| metadata.complex_type(&x)) {
            collection.append(&mut target.fields(&metadata))
        }

        collection
    }

    /// Get all the navigation properties from the type and the chain of base types.
    pub fn navigation_properties(&self, metadata: &super::EDMX) -> Vec<super::NavigationProperty> {
        let mut collection = Vec::new();
        collection.append(&mut self.navigation_properties.clone());

        if let Some(target) = self.base_type.as_ref().and_then(|x| metadata.entity_type(&x)) {
            collection.append(&mut target.navigation_properties(&metadata))
        }

        if let Some(target) = self.base_type.as_ref().and_then(|x| metadata.complex_type(&x)) {
            collection.append(&mut target.navigation_properties(&metadata))
        }

        collection
    }

    /// Get the name of the key for this entity type, potentially checking through the base type
    /// ancestry to find it.
    pub fn key_name(&self, metadata: &super::EDMX) -> String {
        match &self.key {
            Some(key) => key.property_ref.name.clone(),
            None => match &self.base_type {
                Some(base_type) => match metadata.entity_type(&base_type) {
                    Some(entity_type) => entity_type.key_name(metadata).to_string(),
                    None => panic!("Can't find base type for {}", self.name),
                },
                None => panic!("Key type {} has neither a key nor a base type", self.name),
            },
        }
    }

    /// Get the type of the key in this entity type. If the entity type has a key, we look up the
    /// type of that key in all the fields of the current entity and its ancestors. If it doesn't,
    /// we have to look up the base type ancestors to find a key.
    pub fn key_type(&self, metadata: &super::EDMX) -> super::QualifiedType {
        match &self.key {
            Some(key) => self
                .fields(metadata)
                .iter()
                .find(|property| property.name == key.property_ref.name)
                .unwrap()
                .underlying_type()
                .clone(),
            None => match &self.base_type {
                Some(base_type) => match metadata.entity_type(&base_type) {
                    Some(entity) => entity.key_type(metadata),
                    None => panic!("Base type {} doesn't exist", base_type.to_string()),
                },

                None => panic!("Key type {} has neither a key nor a base type", self.name),
            },
        }
    }
}

/// The "key" of an entity type. This is the unique identifier of any given resource within the
/// entity set, and we can think of it as a primary key.
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
