use metadata::ndc;
use ndc_sdk::models;
use std::collections::BTreeMap;

/// Translate our internal understanding of models into the `ndc-spec` format.
pub fn translate(collections: &Vec<ndc::Collection>) -> Vec<models::CollectionInfo> {
    let mut results = Vec::new();

    for collection in collections {
        let mut foreign_keys = BTreeMap::new();

        // Each collection has a key, so we can always generate a "foreign key" relationship to any
        // entity for which we have a navigation property.
        for (relationship_target, foreign_collection) in &collection.relationships {
            let target_key = collections
                .iter()
                .find(|collection| &collection.name == foreign_collection)
                .map(|collection| collection.key.clone())
                .unwrap();

            let column_mapping = BTreeMap::from([(relationship_target.clone(), target_key)]);

            foreign_keys.insert(
                relationship_target.clone(),
                models::ForeignKeyConstraint {
                    column_mapping,
                    foreign_collection: foreign_collection.clone(),
                },
            );
        }

        println!("{:?}", collection.collection_type.to_string());

        // Using the collection's own key, we can also generate a unique key constraint.
        let primary_key_constraint = format!("{}By{}", collection.name, collection.key.clone());
        let uniqueness_constraint = models::UniquenessConstraint {
            unique_columns: vec![collection.key.clone()],
        };

        results.push(models::CollectionInfo {
            name: collection.name.clone(),
            collection_type: collection.collection_type.to_string(),
            foreign_keys,
            description: None,
            arguments: BTreeMap::new(),
            uniqueness_constraints: BTreeMap::from([(
                primary_key_constraint,
                uniqueness_constraint,
            )]),
        });
    }

    results
}
