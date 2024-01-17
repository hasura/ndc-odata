//! Interpreters and generators for configuration in `ndc-odata`.

use metadata::ndc;
use ndc_sdk::connector;

/// Take the endpoint URL from the user config, introspect the schema, and replace the rest of the
/// metadata with the results of that introspection. Most of the work of this function is farmed
/// out to the `metadata` package.
pub async fn update_configuration(
    configuration: ndc::RawConfiguration,
) -> Result<ndc::RawConfiguration, connector::UpdateConfigurationError> {
    let metadata = format!("{}/$metadata", configuration.api_endpoint);

    // @TODO: can we do something about error unification?
    let response = reqwest::get(&metadata)
        .await
        .map_err(Box::from)
        .map_err(connector::UpdateConfigurationError::Other)?;

    let response_text = response
        .text()
        .await
        .map_err(Box::from)
        .map_err(connector::UpdateConfigurationError::Other)?;

    let document = quick_xml::de::from_str(&response_text)
        .map_err(Box::from)
        .map_err(connector::UpdateConfigurationError::Other)?;

    Ok(ndc::RawConfiguration {
        api_endpoint: configuration.api_endpoint.clone(),
        schema: metadata::prepare_odata_edmx(document),
    })
}

/// The only thing we actually care about is whether we can parse the URL. Everything else we
/// currently take on good faith.
/// @TODO: this would be a good place to check a whole bunch of invariants:
///        - Do all entity sets talk about valid entity types?
///        - Do all entity types have a key somewhere in their ancestry?
///        - etc.
pub async fn validate_raw_configuration(
    configuration: ndc::RawConfiguration,
) -> Result<ndc::Configuration, connector::ValidateError> {
    let parsed = ndc::Endpoint::parse(&configuration.api_endpoint).map_err(|message| {
        let path = Vec::from([connector::KeyOrIndex::Key("api_endpoint".to_string())]);
        let invalid_range = connector::InvalidRange { path, message };

        connector::ValidateError::ValidateError(Vec::from([invalid_range]))
    })?;

    Ok(metadata::ndc::Configuration {
        api_endpoint: parsed,
        schema: configuration.schema,
    })
}
