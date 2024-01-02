use http::Uri;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// There are no `Deserialize` / `Serialize` / `JsonSchema` instances for `Uri`, so we'll save the
/// components in a new structure and feed them into a Uri builder when we need them.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Endpoint {
    pub protocol: String,
    pub authority: String,
    pub path: String,
}

impl Endpoint {
    pub fn parse(input: &String) -> Result<Self, String> {
        let uri = input.parse::<Uri>().map_err(|x| x.to_string())?;

        let protocol: String = match uri.scheme_str() {
            Some(protocol) => protocol.to_string(),
            None => "API endpoint URL is missing a protocol".to_string(),
        };

        let authority: String = match uri.authority() {
            Some(authority) => authority.as_str().to_string(),
            None => "API endpoint URL is missing an authority".to_string(),
        };

        let path = match uri.path_and_query() {
            Some(path_and_query) => path_and_query.path().to_string(),
            None => String::new(),
        };

        Ok(Self {
            protocol,
            authority,
            path,
        })
    }

    pub fn to_string(&self) -> String {
        Uri::builder()
            .scheme(self.protocol.as_str())
            .authority(self.authority.as_str())
            .path_and_query(self.path.as_str())
            .build()
            .expect("Failed to re-parse a valid URL")
            .to_string()
    }
}
