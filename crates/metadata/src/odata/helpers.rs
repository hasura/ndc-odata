//! Helpers for parsing the OData metadata.

use serde::{de, Deserialize, Deserializer};

/// A function that always returns true. We need this because we can't just use `default = "true"`
/// as a serde attribute: we have to pass a function.
pub fn r#true() -> bool {
    true
}

/// A function that parses booleans from XML attribute strings. Currently, we require that the
/// string be precisely "true" or "false", though we may have to relax this requirement as the
/// connector begins to make contact with the outside world.
pub fn str_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let bool_string = String::deserialize(deserializer)?.to_lowercase();
    bool_string
        .parse()
        .map_err(|_| de::Error::unknown_variant(&bool_string, &["true", "false"]))
}
