pub mod ndc;
pub mod odata;

use std::collections::{BTreeMap, BTreeSet};

/// Translate an EDMX document into the ndc-odata metadata type.
pub fn prepare_odata_edmx(metadata: odata::EDMX) -> ndc::Schema {
    let mut collections: Vec<ndc::Collection> = Vec::new(); // TODO: don't use a Vec for appends.
    let mut scalar_types: BTreeSet<String> = BTreeSet::new();
    let mut object_types: BTreeMap<String, ndc::ObjectType> = BTreeMap::new();

    for schema in &metadata.data_services.schema {
        let mut scalar_type_additions = find_scalar_types(&schema);
        scalar_types.append(&mut scalar_type_additions);

        let mut object_type_additions = find_object_types(&schema);
        object_types.append(&mut object_type_additions);

        let mut collection_additions = find_collections(&schema);
        collections.append(&mut collection_additions);
    }

    ndc::Schema {
        collections,
        object_types,
        scalar_types,
    }
}

/// OData's notion of entity sets maps pretty neatly onto the NDC notion of collections, so for
/// now, we just transform one into the other.
pub fn find_collections(schema: &odata::Schema) -> Vec<ndc::Collection> {
    schema
        .entity_container
        .entity_sets
        .iter()
        .map(|entity_set| ndc::Collection {
            name: entity_set.name.clone(),
            collection_type: entity_set.name.clone(),
        })
        .collect()
}

/// Traverse the EDMX document looking for scalar types. If collections are found, the singular
/// types should be extracted.
pub fn find_scalar_types(schema: &odata::Schema) -> BTreeSet<String> {
    let mut scalar_types = BTreeSet::new();

    schema
        .entity_types
        .iter()
        .flat_map(|entity_type| entity_type.properties.clone())
        .map(|property| property.r#type.inner.underlying_type())
        .for_each(|inner| {
            scalar_types.insert(inner);
        });

    schema
        .complex_types
        .iter()
        .flat_map(|complex_type| complex_type.properties.clone())
        .map(|property| property.r#type.inner.underlying_type())
        .for_each(|inner| {
            scalar_types.insert(inner);
        });

    schema
        .enum_types
        .iter()
        .map(|enum_type| enum_type.name.to_string())
        .for_each(|name| {
            scalar_types.insert(name);
        });

    schema
        .entity_container
        .singletons
        .iter()
        .map(|singleton| singleton.r#type.inner.underlying_type())
        .for_each(|inner| {
            scalar_types.insert(inner);
        });

    scalar_types
}

/// Traverse the EDMX document looking for object types. The concept of object types in the NDC
/// spec maps quite neatly to the OData concepts of entity types and complex types.
pub fn find_object_types(schema: &odata::Schema) -> BTreeMap<String, ndc::ObjectType> {
    let mut object_types = BTreeMap::new();

    let property_to_field = |property: odata::Property| {
        let name = property.name.clone();
        let field = type_description_to_type(&property.r#type);

        (name, field)
    };

    schema
        .entity_types
        .clone()
        .into_iter()
        .for_each(|entity_type| {
            let object_type = odata::types::Type::Qualified {
                schema: schema.namespace.clone(),
                r#type: entity_type.name,
            };

            let fields = entity_type
                .properties
                .into_iter()
                .map(property_to_field)
                .collect();

            object_types.insert(object_type.as_string(), ndc::ObjectType { fields });
        });

    schema
        .complex_types
        .clone()
        .into_iter()
        .for_each(|complex_type| {
            let object_type = odata::types::Type::Qualified {
                schema: schema.namespace.clone(),
                r#type: complex_type.name,
            };

            let fields = complex_type
                .properties
                .into_iter()
                .map(property_to_field)
                .collect();

            object_types.insert(object_type.as_string(), ndc::ObjectType { fields });
        });

    for entity_set in &schema.entity_container.entity_sets {
        match object_types.get(&entity_set.entity_type) {
            Some(object_type) => object_types.insert(entity_set.name.clone(), object_type.clone()),
            None => None, // panic!("Singular type {} should exist...", &entity_set.entity_type),
        };
    }

    object_types
}

// --- Helpers

/// OData has a slightly different language for types (for example, you can't have a nullable array
/// of nullable elements: all array elements are non-null), so we have to do a sightly clunky
/// mapping.
fn type_description_to_type(input: &odata::TypeDescription) -> ndc::Type {
    match input {
        odata::TypeDescription {
            nullable: true,
            inner,
        } => ndc::Type::Nullable {
            underlying_type: Box::new(type_description_to_type(&odata::TypeDescription {
                inner: inner.clone(),
                nullable: false,
            })),
        },

        odata::TypeDescription {
            nullable: _,
            inner: odata::types::Type::Collection { element_type },
        } => ndc::Type::Collection {
            element_type: Box::new(type_description_to_type(&odata::TypeDescription {
                inner: *element_type.clone(),
                nullable: false,
            })),
        },

        odata::TypeDescription {
            nullable: _,
            inner:
                odata::types::Type::Qualified {
                    schema: _,
                    r#type: _,
                },
        } => ndc::Type::Named {
            name: input.inner.underlying_type(),
        },
    }
}
