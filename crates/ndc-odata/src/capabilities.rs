//! The capabilities that the OData connector affords.

use ndc_sdk::models;

/// Get the capabilities of the connector.
pub fn get_capabilities() -> models::CapabilitiesResponse {
    models::CapabilitiesResponse {
        versions: "^0.1.0".to_string(),
        capabilities: models::Capabilities {
            query: models::QueryCapabilities {
                aggregates: None,
                variables: None,
            },

            explain: Some(models::LeafCapability {}),

            relationships: Some(models::RelationshipCapabilities {
                order_by_aggregate: None,
                relation_comparisons: None,
            }),
        },
    }
}
