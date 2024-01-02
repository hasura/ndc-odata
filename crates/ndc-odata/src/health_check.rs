//! Connector health check functions.

use ndc_sdk::connector;

/// Check that the external OData service is alive and well. We do this by firing a request at its
/// root endpoint, and checking whether we get a 200 back. This is about as comprehensive as we can
/// be with only what we're provided by the OData spec.
pub async fn health_check(api_endpoint: &String) -> Result<(), connector::HealthError> {
    let status = reqwest::get(api_endpoint)
        .await
        .map_err(Box::from)
        .map_err(connector::HealthError::Other)?
        .status();

    if status.is_success() {
        Ok(())
    } else {
        let explanation = status.canonical_reason().unwrap_or(status.as_str());
        Err(connector::HealthError::Other(Box::from(explanation)))
    }
}
