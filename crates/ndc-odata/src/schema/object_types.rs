use metadata::ndc;
use ndc_sdk::models;
use std::collections::BTreeMap;

pub fn translate(
    object_types: &BTreeMap<String, ndc::ObjectType>,
) -> BTreeMap<String, models::ObjectType> {
    let mut results = BTreeMap::new();

    for (object_type_name, object_type) in object_types {
        let mut fields = BTreeMap::new();

        for (field_name, field_type) in &object_type.fields {
            fields.insert(
                field_name.clone(),
                models::ObjectField {
                    r#type: super::types::translate_type(&field_type),
                    description: None,
                },
            );
        }

        for (field_name, field_type) in &object_type.fields {
            fields.insert(
                field_name.clone(),
                models::ObjectField {
                    r#type: super::types::translate_type(&field_type),
                    description: None,
                },
            );
        }

        for (relationship_name, relationship_type) in &object_type.relationships {
            fields.insert(
                relationship_name.clone(),
                models::ObjectField {
                    r#type: super::types::translate_type(&relationship_type),
                    description: None,
                },
            );
        }

        results.insert(
            object_type_name.clone(),
            models::ObjectType {
                description: None,
                fields,
            },
        );
    }

    results
}
