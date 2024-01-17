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
    pub fn extract_from(
        metadata: &odata::EDMX,
        schema: &odata::Schema,
    ) -> BTreeMap<String, ndc::ObjectType> {
        let mut object_types = BTreeMap::new();

        for entity_type in &schema.entity_types {
            let object_type = odata::Type::Qualified {
                qualified_type: odata::QualifiedType {
                    schema: schema.namespace.clone(),
                    name: entity_type.name.clone(),
                },
            };

            object_types.insert(
                object_type.to_string(),
                from_entity_type(metadata, &entity_type),
            );
        }

        for complex_type in &schema.complex_types {
            let object_type = odata::Type::Qualified {
                qualified_type: odata::QualifiedType {
                    schema: schema.namespace.clone(),
                    name: complex_type.name.clone(),
                },
            };

            object_types.insert(
                object_type.to_string(),
                from_complex_type(metadata, &complex_type),
            );
        }

        object_types
    }
}

/// Create an object type based on a complex type underneath.
fn from_complex_type(metadata: &odata::EDMX, structure: &odata::ComplexType) -> ObjectType {
    let mut fields = BTreeMap::new();

    if let Some(base_type) = &structure.base_type {
        match metadata.complex_type(&base_type) {
            Some(complex_type) => {
                let mut inner = from_complex_type(metadata, &complex_type);
                fields.append(&mut inner.fields);
            }
            None => panic!(
                "{}'s base type {} doesn't exist",
                structure.name,
                base_type.to_string()
            ),
        }
    }

    for property in &structure.properties {
        let name = property.name.clone();
        let value = super::Type::from_type_data(&property.r#type);

        fields.insert(name, value);
    }

    let mut navigation = navigation_properties(metadata, &structure.navigation_properties);
    fields.append(&mut navigation);

    ObjectType { fields }
}

/// Create an object type based on an entity type underneath.
fn from_entity_type(metadata: &odata::EDMX, structure: &odata::EntityType) -> ObjectType {
    let mut fields = BTreeMap::new();

    for property in &structure.properties {
        let name = property.name.clone();
        let value = super::Type::from_type_data(&property.r#type);

        fields.insert(name, value);
    }

    let mut navigation =
        navigation_properties(metadata, &structure.navigation_properties(metadata));
    fields.append(&mut navigation);

    ObjectType { fields }
}

/// Convert OData navigation properties to `ndc-odata` navigation properties.
fn navigation_properties(
    metadata: &odata::EDMX,
    navigation_properties: &Vec<odata::NavigationProperty>,
) -> BTreeMap<String, super::Type> {
    let mut fields = BTreeMap::new();

    for navigation_property in navigation_properties {
        let target_type = navigation_property.r#type.inner.underlying_type();
        let target = metadata.entity_type(&target_type).clone().unwrap();

        let qualified_type = target.key_type(metadata);
        fields.insert(
            navigation_property.name.clone(),
            super::Type::Qualified {
                qualified_type: super::QualifiedType {
                    schema: qualified_type.schema.clone(),
                    name: qualified_type.name.clone(),
                }
            },
        );
    }

    fields
}
