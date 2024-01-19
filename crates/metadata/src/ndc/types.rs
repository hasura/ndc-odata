//! Functions relating to extracting scalar types from OData metadata.

use crate::odata::{schema, types};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// The name of a scalar type.
/// @TODO: extend the structure to include operations.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScalarType(pub String);

impl ScalarType {
    /// Traverse the EDMX document looking for scalar types. In valid EDMX documents, all
    /// references to entity types or complex types within an entity container or function/action
    /// should refer to types declared in the root of the schema as either entity types or complex
    /// types, so we don't bother traversing into them.
    pub fn extract_from(schema: &schema::Schema) -> BTreeSet<ScalarType> {
        //  We don't want to show all the primitive types unless we need to, because it'll just
        //  pollute the GraphQL schema. So, we only declare scalar types that we find while
        //  traversing the API schema.
        let mut scalar_types = BTreeSet::new();

        for entity_type in &schema.entity_types {
            for property in &entity_type.properties {
                let underlying_type = property.underlying_type().to_string();

                if PRIMITIVE_TYPES.contains(&underlying_type.as_str()) {
                    scalar_types.insert(ScalarType(underlying_type));
                }
            }
        }

        for complex_type in &schema.complex_types {
            for property in &complex_type.properties {
                let underlying_type = property.underlying_type().to_string();

                if PRIMITIVE_TYPES.contains(&underlying_type.as_str()) {
                    scalar_types.insert(ScalarType(underlying_type));
                }
            }
        }

        scalar_types
    }
}

/// Types as described in the `ndc-spec`: we reify collections and nullability as separate
/// constructors of the type. We also require that all types be namespaced.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(tag = "type")]
pub enum Type {
    Collection { element_type: Box<Type> },
    Nullable { underlying_type: Box<Type> },
    Qualified { qualified_type: QualifiedType },
}

/// Qualified types that state both the schema and name of the type to which they refer.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct QualifiedType {
    pub schema: String,
    pub name: String,
}

impl std::fmt::Display for QualifiedType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}", self.schema, self.name)
    }
}

impl Type {
    /// OData has a slightly different language for types (for example, you can't have a nullable array
    /// of nullable elements: all array elements are non-null), so we have to do a sightly clunky
    /// mapping.
    pub fn from_type_data(input: &types::TypeData) -> Self {
        match input {
            types::TypeData {
                inner,
                nullable: false,
            } => {
                let underlying = Type::from_type_data(&types::TypeData {
                    inner: inner.clone(),
                    nullable: true,
                });

                Type::Nullable {
                    underlying_type: Box::new(underlying),
                }
            }

            types::TypeData {
                inner: types::Type::Collection { elements },
                ..
            } => {
                let underlying = types::TypeData {
                    inner: *elements.clone(),
                    nullable: false,
                };

                Type::Collection {
                    element_type: Box::new(Type::from_type_data(&underlying)),
                }
            }

            types::TypeData {
                inner: types::Type::Qualified { qualified_type },
                ..
            } => Type::Qualified {
                qualified_type: QualifiedType {
                    schema: qualified_type.schema.clone(),
                    name: qualified_type.name.clone(),
                },
            },
        }
    }
}

/// All the primitive /scalar/ types. Rather than trying to deduce scalar types by whether
/// or not they're defined elsewhere in the schema, we can make our lives a little easier by
/// taking advantage of the fact that OData already has defined primitive types, and just
/// filter out any types that don't show up in our schema.
///
/// @TODO: what do we do about geography and geometry?
const PRIMITIVE_TYPES: [&str; 16] = [
    "Edm.Binary",
    "Edm.Boolean",
    "Edm.Byte",
    "Edm.Date",
    "Edm.DateTimeOffset",
    "Edm.Decimal",
    "Edm.Double",
    "Edm.Duration",
    "Edm.Guid",
    "Edm.Int16",
    "Edm.Int32",
    "Edm.Int64",
    "Edm.SByte",
    "Edm.Single",
    "Edm.String",
    "Edm.TimeOfDay",
];
