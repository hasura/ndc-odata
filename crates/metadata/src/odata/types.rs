//! A Serializer and Deserializer for OData's type syntax.

use pest::Parser;
use pest_derive::Parser;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// OData has a relatively simple type structure: there are named types and collections of types.
/// Even the primitive types in an OData API are provided as named types in the `Edm` schema, and
/// so they can be treated in the same way.
#[derive(Clone, Debug, JsonSchema, Parser)]
#[grammar = "../grammars/type_name.pest"]
pub enum Type {
    /// A collection of a specific type.
    Collection { element_type: Box<Type> },

    /// A singular type defined in some schema.
    Qualified { schema: String, r#type: String },
}

impl Type {
    /// Pull out the scalar type name within a `Type`.
    pub fn underlying_type(&self) -> String {
        match &self {
            Type::Collection { element_type } => element_type.underlying_type(),
            Type::Qualified { schema, r#type } => format!("{schema}.{type}"),
        }
    }

    /// A helper method to print an OData type in the OData format.
    pub fn as_string(&self) -> String {
        match self {
            Type::Collection { element_type } => {
                format!("Collection({})", element_type.as_string())
            }

            Type::Qualified { schema, r#type } => format!("{schema}.{type}"),
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
                    element_type: Box::new(inner),
                });
            }

            Rule::component => components.push(pair.as_str()),
            Rule::qualified_name => panic!("Internal error: found raw qualified name"),
            Rule::type_name => panic!("Internal error: found raw type name"),
        }
    }

    let r#type = components.pop().expect("Internal error: bad type grammar");
    Ok(Type::Qualified {
        schema: components.join("."),
        r#type: r#type.to_string(),
    })
}
