pub mod collections;
pub mod endpoint;
pub mod functions;
pub mod object_types;
pub mod procedures;
pub mod schema;
pub mod types;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use collections::*;
pub use endpoint::*;
pub use functions::*;
pub use object_types::*;
pub use procedures::*;
pub use schema::*;
pub use types::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Configuration {
    pub api_endpoint: Endpoint,
    pub schema: Schema,
}

#[derive(Deserialize, JsonSchema, Serialize, Clone, Debug, Default)]
pub struct RawConfiguration {
    // For a basic configuration, this is all we need: we can populate the `Schema` from the OData
    // endpoint using a `/$metadata` introspection query.
    pub api_endpoint: String,

    #[serde(default)]
    pub schema: Schema,
}
