//! The handler for the `/explain` endpoint.

use metadata::ndc;
use ndc_sdk::{connector, models};
use std::collections::BTreeMap;

/// Produce the request using the query pipeline, then return it to the user.
pub fn get_details(
    configuration: &ndc::Configuration,
    request: models::QueryRequest,
) -> Result<models::ExplainResponse, connector::ExplainError> {
    let request_url = super::query::Request::from_user_request(configuration, &request)
        .map_err(Box::from)
        .map_err(connector::ExplainError::Other)?
        .to_url()
        .map_err(Box::from)
        .map_err(connector::ExplainError::Other)?;

    // @TODO: we could add a few more details in here using OData context.
    let mut details = BTreeMap::new();
    details.insert("query".to_string(), request_url);

    Ok(models::ExplainResponse { details })
}
