use metadata::ndc;
use ndc_sdk::models;
use std::collections::{BTreeMap, BTreeSet};

pub fn get_schema(configuration: &ndc::Configuration) -> models::SchemaResponse {
    models::SchemaResponse {
        collections: translate_collections(&configuration.schema.collections),
        object_types: translate_object_types(&configuration.schema.object_types),
        scalar_types: translate_scalar_types(&configuration.schema.scalar_types),

        // TODO: In OData, these are functions and actions - we already parse these in the OData
        // response, so these shoudn't be too tricky to add in.
        functions: Vec::new(),
        procedures: Vec::new(),
    }
}

/// Translate an `ndc-odata` collection into an `ndc-spec` collection.
pub fn translate_collections(collections: &Vec<ndc::Collection>) -> Vec<models::CollectionInfo> {
    let transform = |ndc::Collection {
                         name,
                         collection_type,
                     }: &ndc::Collection| {
        models::CollectionInfo {
            name: name.clone(),
            collection_type: collection_type.clone(),
            description: None,
            arguments: BTreeMap::new(),
            foreign_keys: BTreeMap::new(),
            uniqueness_constraints: BTreeMap::new(),
        }
    };

    collections.iter().map(transform).collect()
}

/// Translate an `ndc-odata` object type into an `ndc-spec` collection.
/// TODO: This seems messy; maybe I should learn more about iterators?
pub fn translate_object_types(
    object_types: &BTreeMap<String, ndc::ObjectType>,
) -> BTreeMap<String, models::ObjectType> {
    let mut translated = BTreeMap::new();

    for (name, object_type) in object_types {
        let mut fields = BTreeMap::new();

        for (key, r#type) in &object_type.fields {
            fields.insert(
                key.clone(),
                models::ObjectField {
                    description: None,
                    r#type: translate_type(r#type),
                },
            );
        }

        translated.insert(
            name.clone(),
            models::ObjectType {
                description: None,
                fields,
            },
        );
    }

    translated
}

/// Translate an `ndc-odata` scalar type (currently just a `String`) into an `ndc-spec` scalar
/// type.
pub fn translate_scalar_types(
    scalar_types: &BTreeSet<String>,
) -> BTreeMap<String, models::ScalarType> {
    let mut translated = BTreeMap::new();

    for name in scalar_types {
        translated.insert(
            name.clone(),
            models::ScalarType {
                aggregate_functions: BTreeMap::new(),
                comparison_operators: BTreeMap::new(),
            },
        );
    }

    translated
}

// Helpers

/// Translate an `ndc-odata` type into an `ndc-spec` type. They're basically identical.
fn translate_type(r#type: &ndc::Type) -> models::Type {
    match r#type {
        ndc::Type::Named { name } => models::Type::Named {
            name: name.to_string(),
        },

        ndc::Type::Nullable { underlying_type } => models::Type::Nullable {
            underlying_type: Box::new(translate_type(&*underlying_type)),
        },

        ndc::Type::Collection { element_type } => models::Type::Array {
            element_type: Box::new(translate_type(&*element_type)),
        },
    }
}
