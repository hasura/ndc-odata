//! Functions relating to extracting object types.

use crate::ndc;
use crate::odata;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// In `ndc-odata`, we don't distinguish between entity types and complex types: the only
/// difference is the presence of a key, and we represent them as foreign key relationships.
/// However, because this distinction only exists within `ndc-odata`, we must track them
/// separately.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct ObjectType {
    pub fields: BTreeMap<String, super::Type>,
}

impl ObjectType {
    /// Collect entity types and complex types to declare as object types in the `ndc-spec` schema.
    pub fn extract_from(schema: &odata::Schema) -> BTreeMap<String, ndc::ObjectType> {
        let mut object_types = BTreeMap::new();

        for entity_type in &schema.entity_types {
            // As far as I know, you can't declare a type within one schema that "belongs" to
            // another schema, so we should always be safe to assume the parent schema here.
            let object_type = odata::types::Type::Qualified {
                schema: schema.namespace.clone(),
                name: entity_type.name.clone(),
            };

            object_types.insert(object_type.as_string(), from_entity_type(&entity_type));
        }

        for complex_type in &schema.complex_types {
            // ibid.
            let object_type = odata::types::Type::Qualified {
                schema: schema.namespace.clone(),
                name: complex_type.name.clone(),
            };

            object_types.insert(object_type.as_string(), from_complex_type(&complex_type));
        }

        object_types
    }
}

/// Create an object type based on a complex type underneath.
fn from_complex_type(structure: &odata::ComplexType) -> ObjectType {
    let mut fields = BTreeMap::new();

    for property in &structure.properties {
        fields.insert(
            property.name.clone(),
            super::Type::from_type_data(&property.r#type),
        );
    }

    // We define every navigation property as a "field" with the property's name in the `ndc-spec`,
    // and we're going to use this as our faux foreign key.
    for navigation_property in &structure.navigation_properties {
        fields.insert(
            navigation_property.name.clone(),
            super::Type::from_type_data(&navigation_property.r#type),
        );
    }

    ObjectType { fields }
}

/// Create an object type based on an entity type underneath.
fn from_entity_type(structure: &odata::EntityType) -> ObjectType {
    let mut fields = BTreeMap::new();

    for property in &structure.properties {
        fields.insert(
            property.name.clone(),
            super::Type::from_type_data(&property.r#type),
        );
    }

    // ibid.
    for navigation_property in &structure.navigation_properties {
        fields.insert(
            navigation_property.name.clone(),
            super::Type::from_type_data(&navigation_property.r#type),
        );
    }

    ObjectType { fields }
}
