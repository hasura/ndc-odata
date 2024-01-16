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

            object_types.insert(
                object_type.as_string(),
                from_entity_type(schema, &entity_type),
            );
        }

        for complex_type in &schema.complex_types {
            // ibid.
            let object_type = odata::types::Type::Qualified {
                schema: schema.namespace.clone(),
                name: complex_type.name.clone(),
            };

            object_types.insert(
                object_type.as_string(),
                from_complex_type(schema, &complex_type),
            );
        }

        object_types
    }
}

/// Create an object type based on a complex type underneath.
fn from_complex_type(schema: &odata::Schema, structure: &odata::ComplexType) -> ObjectType {
    let mut fields = BTreeMap::new();

    for property in &structure.properties {
        fields.insert(
            property.name.clone(),
            super::Type::from_type_data(&property.r#type),
        );
    }

    fields.append(&mut generate_navigation_properties(
        schema,
        &structure.navigation_properties,
    ));
    ObjectType { fields }
}

/// Create an object type based on an entity type underneath.
fn from_entity_type(schema: &odata::Schema, structure: &odata::EntityType) -> ObjectType {
    let mut fields = BTreeMap::new();

    for property in &structure.properties {
        fields.insert(
            property.name.clone(),
            super::Type::from_type_data(&property.r#type),
        );
    }

    fields.append(&mut generate_navigation_properties(
        schema,
        &structure.navigation_properties,
    ));
    ObjectType { fields }
}

fn generate_navigation_properties(
    schema: &odata::Schema,
    navigation_properties: &Vec<odata::NavigationProperty>,
) -> BTreeMap<String, super::Type> {
    let mut fields = BTreeMap::new();

    for navigation_property in navigation_properties {
        // See `crates/metadata/src/odata/types` for an extended rant about this. TL;DR, stripping the schema is not a
        // good idea, but deadlines are tight and I don't really have time to write a more complex schema parser.
        let target_type = navigation_property.r#type.inner.schemaless_name();

        // Find the entity type of the target of this particular navigation property. We also currently assume that
        // metadata is valid - eventually, we'll verify all the invariants on which we rely in `validate_configuration`.
        let target = schema.entity_type(&target_type).clone().unwrap();

        // The "primary key" of the target collection.
        let key = target.key(schema);

        // Find the type of the "primary key" of the target collection.
        let entity_type = &target
            .properties
            .iter()
            .find(|p| p.name == key)
            .unwrap()
            .r#type;

        fields.insert(
            navigation_property.name.clone(),
            super::Type::from_type_data(&entity_type),
        );
    }

    fields
}
