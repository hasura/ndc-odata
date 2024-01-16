//! A Serializer and Deserializer for OData's type syntax.

use pest::Parser;
use pest_derive::Parser;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A complex type is the analogous to an NDC object type: it is a product type comprised of named
/// fields that we'll represent using JSON objects. They may be built by extending other types.
/// @TODO: can complex types have navigation properties?
#[derive(Clone, Debug, Deserialize)]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,

    // @TODO: we should expand base types as soon as possible: right now, for
    // example, if the `key` of an entity type is a property on the base type,
    // we'll crash.
    #[serde(rename = "@BaseType")]
    pub base_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "Property")]
    pub properties: Vec<Property>,

    #[serde(default)]
    #[serde(rename = "NavigationProperty")]
    pub navigation_properties: Vec<NavigationProperty>,
}

/// An available navigation property. Entity sets may choose to bind this property to a different
/// entity set to indicate a relationship between the two. This is analogous to foreign keys.
#[derive(Clone, Debug, Deserialize)]
pub struct NavigationProperty {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: super::TypeData,
}

/// A field within a complex type: it is described by a field name and a value type.
#[derive(Clone, Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(flatten)]
    pub r#type: TypeData,
}

impl Property {
    /// Get the underlying type of a property: the `Named` string at the bottom of the `Type`
    /// stack.
    pub fn underlying_type(&self) -> String {
        self.r#type.underlying_type()
    }
}

/// As well as an underlying type, some number of other attributes relating to the type may be
/// included as attributes on the parent tag (`Property`, `Parameter`, and so on). Currently,
/// because we perform no mutations, the only one we really care about is nullability.
#[derive(Clone, Debug, Deserialize)]
pub struct TypeData {
    #[serde(rename = "@Type")]
    pub inner: Type,

    #[serde(default = "super::helpers::r#true")]
    #[serde(rename = "@Nullable")]
    #[serde(deserialize_with = "super::helpers::str_to_bool")]
    pub nullable: bool,
}

impl TypeData {
    /// Get the underlying type of a property: the `Named` string at the bottom of the `Type`
    /// stack.
    pub fn underlying_type(&self) -> String {
        self.inner.underlying_type()
    }
}

/// OData has a relatively simple type structure: there are named types and collections of types.
/// Even the primitive types in an OData API are provided as named types in the `Edm` schema, and
/// so they can be treated in the same way.
#[derive(Clone, Debug, JsonSchema, Parser)]
#[grammar = "../grammars/type_name.pest"]
pub enum Type {
    /// A collection of a specific type.
    Collection { elements: Box<Type> },

    /// A singular type defined in some schema.
    Qualified { schema: String, name: String },
}

impl Type {
    /// Pull out the scalar type name within a `Type`.
    pub fn underlying_type(&self) -> String {
        match &self {
            Type::Collection { elements } => elements.underlying_type(),
            Type::Qualified { schema, name } => format!("{schema}.{name}"),
        }
    }

    // @TODO: this is a real wart. The problem is that a type is implicitly
    // namespaced by the schema in which it is declared, _but_ every reference
    // to it will be /expicitly/ namespaced. Ideally, what we'd do is parse the
    // OData metadata and add the schema namespace to every type we parse as
    // we parse it. However, that would mean dealing with Serde's dark corners.
    //
    // Instead, for now, we assume that the APIs we're interested in will live
    // within a single schema, so we're always safe to strip the schema because
    // it should always be the right one anyway. This is not a good long-term
    // solution.
    pub fn schemaless_name(&self) -> String {
        match &self {
            Type::Collection { elements } => elements.schemaless_name(),
            Type::Qualified { schema: _, name } => name.clone(),
        }
    }

    /// A helper method to print an OData type in the OData format.
    pub fn as_string(&self) -> String {
        match self {
            Type::Collection { elements } => format!("Collection({})", elements.as_string()),
            Type::Qualified { schema, name } => format!("{schema}.{name}"),
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let type_string = String::deserialize(deserializer)?;

        read_type_pairs::<D>(
            Type::parse(Rule::type_name, &type_string).map_err(serde::de::Error::custom)?,
        )
    }
}

impl Serialize for Type {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.as_string())
    }
}

/// A helper for deserialization. According to the Pest grammar, we'll have one of two situations:
///
/// - We have a list of `component` rules, which means we're dealing with a regular type.
/// - We have a single `collection` rule, with `component` children, so we have a collection.
///
/// If the latter is the case, we'll find out in the first iteration of the loop, so we can exit
/// early.
fn read_type_pairs<'de, D: Deserializer<'de>>(
    pairs: pest::iterators::Pairs<Rule>,
) -> Result<Type, D::Error> {
    let mut components = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::collection => {
                let inner = read_type_pairs::<D>(pair.into_inner())?;
                return Ok(Type::Collection {
                    elements: Box::new(inner),
                });
            }

            Rule::component => components.push(pair.as_str()),

            // I'm not sure why `pest` generates these rules, given that they're explicitly
            // silenced in the grammar...
            Rule::qualified_name => panic!("Internal error: found raw qualified name"),
            Rule::type_name => panic!("Internal error: found raw type name"),
        }
    }

    // We guarantee non-emptiness in the grammar, so this should also never fail.
    let name = components.pop().unwrap().to_string();
    let schema = components.join(".");

    Ok(Type::Qualified { schema, name })
}
