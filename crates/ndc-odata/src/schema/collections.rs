use metadata::ndc;
use ndc_sdk::models;
use std::collections::BTreeMap;

/// Translate our internal understanding of models into the `ndc-spec` format.
pub fn translate(collections: &Vec<ndc::Collection>) -> Vec<models::CollectionInfo> {
    collections
        .iter()
        .map(|collection| {
            let mut foreign_keys = BTreeMap::new();

            for (target, foreign_collection) in &collection.relationships {
                foreign_keys.insert(
                    target.clone(),
                    models::ForeignKeyConstraint {
                        column_mapping: BTreeMap::from([(
                            target.clone(),
                            collections
                                .iter()
                                .find(|c| &c.name == foreign_collection)
                                .unwrap()
                                .key
                                .clone(),
                        )]),
                        foreign_collection: foreign_collection.clone(),
                    },
                );
            }

            models::CollectionInfo {
                name: collection.name.clone(),
                collection_type: collection.collection_type.clone(),
                description: None,
                arguments: BTreeMap::new(),
                foreign_keys,
                uniqueness_constraints: uniqueness_constraints(&collection),
            }
        })
        .collect()
}

/// For now, the only uniqueness constraints we can glean are those given by the `Key` property in
/// the OData metadata.
fn uniqueness_constraints(
    collection: &ndc::Collection,
) -> BTreeMap<String, models::UniquenessConstraint> {
    BTreeMap::from([(
        format!("{}By{}", collection.name, collection.key),
        models::UniquenessConstraint {
            unique_columns: vec![collection.key.clone()],
        },
    )])
}
