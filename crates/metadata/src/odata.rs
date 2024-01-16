//! A representation of (some of) the OData Common Schema Definition Language (CSDL). This is by no
//! means complete, but it will be expanded as more of the schema becomes necessary.

pub mod actions;
pub mod entities;
pub mod enums;
pub mod functions;
pub mod helpers;
pub mod schema;
pub mod types;

pub use actions::*;
pub use entities::*;
pub use enums::*;
pub use functions::*;
pub use helpers::*;
pub use schema::*;
pub use types::*;

use serde::Deserialize;

/// The top-level tag of an OData schema.
/// @TODO: we should make some assertion on the version string so we can immediately reject
///        versions that we support and return a substantially nicer error message.
#[derive(Clone, Debug, Deserialize)]
pub struct EDMX {
    #[serde(rename = "@Version")]
    pub version: String,

    #[serde(rename = "@xmlns:edmx")]
    pub edmx: Option<String>,

    #[serde(rename = "DataServices")]
    pub data_services: DataServices,
}

/// The tag containing every schema exposed by this API.
#[derive(Clone, Debug, Deserialize)]
pub struct DataServices {
    #[serde(default)]
    #[serde(rename = "Schema")]
    pub schema: Vec<Schema>,
}
