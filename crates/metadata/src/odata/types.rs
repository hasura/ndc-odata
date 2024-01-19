//! A Serializer and Deserializer for OData's type syntax.

use pest::Parser;
use pest_derive::Parser;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A complex type is the analogous to an NDC object type: it is a product type comprised of named
/// fields that we'll represent using JSON objects. They may be built by extending other types.
#[derive(Clone, Debug, Deserialize)]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "@BaseType")]
    pub base_type: Option<super::QualifiedType>,

    #[serde(default)]
    #[serde(rename = "Property")]
    pub properties: Vec<Property>,

    #[serde(default)]
    #[serde(rename = "NavigationProperty")]
    pub navigation_properties: Vec<NavigationProperty>,
}

impl ComplexType {
    /// Get all the fields from the type and the chain of base types.
    pub fn fields(&self, metadata: &super::EDMX) -> Vec<super::Property> {
        let mut collection = Vec::new();
        collection.append(&mut self.properties.clone());

        if let Some(target) = self
            .base_type
            .as_ref()
            .and_then(|x| metadata.entity_type(x))
        {
            collection.append(&mut target.fields(metadata))
        }

        if let Some(target) = self
            .base_type
            .as_ref()
            .and_then(|x| metadata.complex_type(x))
        {
            collection.append(&mut target.fields(metadata))
        }

        collection
    }

    /// Get all the navigation properties from the type and the chain of base types.
    pub fn navigation_properties(&self, metadata: &super::EDMX) -> Vec<super::NavigationProperty> {
        let mut collection = Vec::new();
        collection.append(&mut self.navigation_properties.clone());

        if let Some(target) = &self
            .base_type
            .as_ref()
            .and_then(|x| metadata.entity_type(x))
        {
            collection.append(&mut target.navigation_properties(metadata))
        }

        if let Some(target) = &self
            .base_type
            .as_ref()
            .and_then(|x| metadata.complex_type(x))
        {
            collection.append(&mut target.navigation_properties(metadata))
        }

        collection
    }
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
    /// Get the underlying type of a property: the `Named` string at the bottom of the `Property`
    /// stack. We do this because the `ndc-spec` requires us to declare all types upfront, and
    /// doesn't consider collections or nullable versions of types to be distinct types in their
    /// own right. Thus, we only need to detect the underlying type.
    pub fn underlying_type(&self) -> &QualifiedType {
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
    /// Get the underlying type of a property: the `Named` string at the bottom of the `TypeData`
    /// stack. We do this because the `ndc-spec` requires us to declare all types upfront, and
    /// doesn't consider collections or nullable versions of types to be distinct types in their
    /// own right. Thus, we only need to detect the underlying type.
    pub fn underlying_type(&self) -> &QualifiedType {
        self.inner.underlying_type()
    }
}

/// If we ignore nullability  - in OData, this is a feature of the user of the type, not the type
/// itself - a type is either a schema-qualified type, _or_ an heterogeneous collection.
#[derive(Clone, Debug, JsonSchema, Parser)]
#[grammar = "../grammars/type_name.pest"]
pub enum Type {
    /// A collection of a specific type.
    Collection { elements: Box<Type> },

    /// A singular type defined in some schema.
    Qualified { qualified_type: QualifiedType },
}

impl Type {
    /// Find the underlying type within any number of 'Collection' layers.
    pub fn underlying_type(&self) -> &QualifiedType {
        match &self {
            Type::Collection { elements } => elements.underlying_type(),
            Type::Qualified { qualified_type } => qualified_type,
        }
    }
}

impl std::fmt::Display for Type {
    /// A helper method to print an OData type in the OData format.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Collection { elements } => write!(formatter, "Collection({})", elements),
            Type::Qualified { qualified_type } => qualified_type.fmt(formatter),
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
        serializer.serialize_str(&self.to_string())
    }
}

/// Specifically, a scalar type (i.e. not a collection) that belongs to some schema.
#[derive(Clone, Debug, JsonSchema)]
pub struct QualifiedType {
    pub schema: String,
    pub name: String,
}

impl std::fmt::Display for QualifiedType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}", self.schema, self.name)
    }
}

impl<'de> Deserialize<'de> for QualifiedType {
    /// This is very lazy: we parse the type as a regular type and forbid the `Collection` variant.
    /// We should write this type its own deserializer.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let type_string = String::deserialize(deserializer)?;
        let pairs = Type::parse(Rule::type_name, &type_string).map_err(serde::de::Error::custom)?;

        match read_type_pairs::<D>(pairs)? {
            Type::Qualified { qualified_type } => Ok(qualified_type),
            Type::Collection { elements: _ } => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&type_string),
                &"a non-collection type",
            )),
        }
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

    let qualified_type = QualifiedType {
        // We guarantee non-emptiness in the grammar, so this should also never fail.
        name: components.pop().expect("Parsed an empty type").to_string(),
        schema: components.join("."),
    };

    Ok(Type::Qualified { qualified_type })
}
