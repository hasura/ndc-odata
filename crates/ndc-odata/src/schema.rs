//! The `ndc-spec` schema for this OData endpoint.

pub mod collections;
pub mod functions;
pub mod object_types;
pub mod procedures;
pub mod types;

use metadata::ndc;
use ndc_sdk::models;

/// Translate the internal `ndc-odata` configuration into an `ndc-spec` schema.
pub fn get_schema(configuration: &ndc::Configuration) -> models::SchemaResponse {
    models::SchemaResponse {
        collections: collections::translate(&configuration.schema.collections),
        object_types: object_types::translate(&configuration.schema.object_types),
        scalar_types: types::scalar_types(&configuration.schema.scalar_types),
        functions: functions::translate(&configuration.schema.functions),
        procedures: procedures::translate(&configuration.schema.procedures),
    }
}
