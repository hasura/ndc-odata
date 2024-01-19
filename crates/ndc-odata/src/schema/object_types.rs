use metadata::ndc;
use ndc_sdk::models;
use std::collections::BTreeMap;

pub fn translate((name, object_type): (&String, &ndc::ObjectType)) -> (String, models::ObjectType) {
    let mut fields = BTreeMap::new();

    for (field_name, field_type) in &object_type.fields {
        let r#type = super::types::translate_type(field_type);
        let description = None;

        fields.insert(
            field_name.clone(),
            models::ObjectField {
                r#type,
                description,
            },
        );
    }

    (
        name.clone(),
        models::ObjectType {
            description: None,
            fields,
        },
    )
}
