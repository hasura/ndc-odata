//! The handler for the `/health` endpoint.

use ndc_sdk::connector;

/// Check that the external OData service is alive and well. We do this by firing a request at its
/// root endpoint, and checking whether we get a 200 back. This is about as comprehensive as we can
/// be with only what we're provided by the OData spec.
pub async fn health_check(
    api_endpoint: &metadata::ndc::Endpoint,
) -> Result<(), connector::HealthError> {
    let status = reqwest::get(api_endpoint.to_string())
        .await
        .map_err(Box::from)
        .map_err(connector::HealthError::Other)?
        .status();

    status.is_success().then_some(()).ok_or({
        let explanation = status.canonical_reason().unwrap_or(status.as_str());
        connector::HealthError::Other(Box::from(explanation))
    })
}
