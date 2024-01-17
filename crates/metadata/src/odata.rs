//! A representation of (some of) the OData Common Schema Definition Language (CSDL).

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

use serde::{Deserialize, Deserializer};

/// The top-level tag of an OData schema.
#[derive(Clone, Debug, Deserialize)]
pub struct EDMX {
    #[serde(rename = "@Version")]
    pub version: Version,

    #[serde(rename = "@xmlns:edmx")]
    pub edmx: Option<String>,

    #[serde(rename = "DataServices")]
    pub data_services: DataServices,
}

impl EDMX {
    pub fn schema(&self, name: &str) -> Option<&Schema> {
        self.data_services.schema(name)
    }

    pub fn complex_type(&self, qualified_type: &QualifiedType) -> Option<ComplexType> {
        self.schema(&qualified_type.schema)?
            .complex_type(&qualified_type.name)
    }

    pub fn entity_type(&self, qualified_type: &QualifiedType) -> Option<EntityType> {
        self.schema(&qualified_type.schema)?
            .entity_type(&qualified_type.name)
    }
}

/// The schema version. We require this to be 4.0 or 4.01, and otherwise, we can reject the
/// metadata with a versioning error, rather than erroring the first time we encounter something
/// contrary to the specification.
#[derive(Clone, Debug)]
pub struct Version(String);

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let version_string = String::deserialize(deserializer)?;

        if ["4.0", "4.01"].contains(&version_string.as_str()) {
            Ok(Version(version_string))
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(version_string.as_str()),
                &"$metadata version 4.0 or 4.01",
            ))
        }
    }
}

/// The tag containing every schema exposed by this API.
#[derive(Clone, Debug, Deserialize)]
pub struct DataServices {
    #[serde(default)]
    #[serde(rename = "Schema")]
    pub schema: Vec<Schema>,
}

impl DataServices {
    pub fn schema(&self, name: &str) -> Option<&Schema> {
        self.schema.iter().find(|target| target.namespace == name)
    }
}
