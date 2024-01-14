//! The `ndc-spec` schema for this OData endpoint.

pub mod collections;
pub mod functions;
pub mod object_types;
pub mod procedures;
pub mod scalar_types;
pub mod types;

use metadata::ndc;
use ndc_sdk::models;

/// Translate the internal `ndc-odata` configuration into an `ndc-spec` schema. The `ndc-odata`
/// internal metadata and `ndc-spec` metadata should be very closely related, so this mapping
/// should be relatively mechanical.
pub fn get_schema(configuration: &ndc::Configuration) -> models::SchemaResponse {
    models::SchemaResponse {
        collections: collections::translate(&configuration.schema.collections),

        object_types: configuration
            .schema
            .object_types
            .iter()
            .map(object_types::translate)
            .collect(),

        scalar_types: configuration
            .schema
            .scalar_types
            .iter()
            .map(scalar_types::translate)
            .collect(),

        functions: configuration
            .schema
            .functions
            .iter()
            .map(functions::translate)
            .collect(),

        procedures: configuration
            .schema
            .procedures
            .iter()
            .map(procedures::translate)
            .collect(),
    }
}
