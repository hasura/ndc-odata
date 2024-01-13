//! Functions relating to NDC collections.

use crate::odata::schema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// In the NDC world, we represent collections in a very similar way. The chief difference is that,
/// at the moment, we don't have a notion of nested types in the NDC spec, which is a bit of a
/// problem given OData's "expandable relationship" model. We get around this by supporting
/// relationships to collections using their keys, and thus we invent a foreign key relationship.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Collection {
    pub name: String,
    pub key: String,
    pub collection_type: String,
    pub relationships: BTreeMap<String, String>, // navigation property => collection
}

impl Collection {
    /// OData's notion of entity sets maps pretty neatly onto the NDC notion of collections, so for
    /// now, we just transform one into the other.
    pub fn extract_from(schema: &schema::Schema) -> Vec<Collection> {
        let mut collections = BTreeMap::new();

        for entity_set in &schema.entity_container.entity_sets {
            let entity_type = match entity_set
                .entity_type
                .split('.')
                .collect::<Vec<_>>()
                .as_slice()
            {
                [parent, name] if parent == &schema.namespace.as_str() => name,
                _ => "entity sets can only have (qualified) types from the same schema.",
            };

            // For now, we have that the metadata is valid, and we panic if it doesn't exist.
            // Eventually, it would be nice to do a validation pass over the OData metadata /before/ we
            // build the connector metadata, just for the sake of error messages.
            let key = schema
                .entity_type(&entity_type)
                .expect("Collection's entity type doesn't exist.")
                .key(&schema);

            let collection_type = entity_set.entity_type.clone();
            let mut relationships = BTreeMap::new();

            for relationship in &entity_set.navigation_property_bindings {
                relationships.insert(relationship.path.clone(), relationship.target.clone());
            }

            collections.insert(
                entity_set.name.clone(),
                Collection {
                    name: entity_set.name.clone(),
                    collection_type,
                    relationships,
                    key,
                },
            );
        }

        collections.into_values().collect()
    }
}
