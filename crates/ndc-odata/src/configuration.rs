use ndc_sdk::connector;

pub async fn fetch_metadata(
    api_endpoint: &String,
) -> Result<metadata::ndc::Schema, connector::UpdateConfigurationError> {
    let metadata = format!("{}/$metadata", api_endpoint);

    let response = reqwest::get(metadata)
        .await
        .map_err(Box::from)
        .map_err(connector::UpdateConfigurationError::Other)?
        .text()
        .await
        .map_err(Box::from)
        .map_err(connector::UpdateConfigurationError::Other)?;

    let document = quick_xml::de::from_str(&response)
        .map_err(Box::from)
        .map_err(connector::UpdateConfigurationError::Other)?;

    Ok(metadata::prepare_odata_edmx(document))
}

// async fn validate_raw_configuration
