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

impl std::fmt::Display for Endpoint {
    /// ```
    /// let endpoint = metadata::ndc::Endpoint {
    ///   protocol: "http".to_string(),
    ///   authority: "example.com".to_string(),
    ///   path: "/test".to_string()
    /// };
    ///
    /// assert_eq!(endpoint.to_string(), "http://example.com/test")
    /// ```
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = Uri::builder()
            .scheme(self.protocol.as_str())
            .authority(self.authority.as_str())
            .path_and_query(self.path.as_str());

        match result.build() {
            Ok(success) => write!(formatter, "{success}"),
            Err(_ignore) => Err(std::fmt::Error),
        }
    }
}

impl Endpoint {
    /// Parse a URL string into an `Endpoint` structure. We do this via the `http::Uri` builder,
    /// and then break it up again (because we can't `Serialise`/`Deserialise` the builder itself).
    pub fn parse(input: &str) -> Result<Self, String> {
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
}
