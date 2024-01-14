//! The handler for the `/explain` endpoint.

use metadata::ndc;
use ndc_sdk::{connector, models};
use std::collections::BTreeMap;

/// Produce the request using the query pipeline, then return it to the user.
pub fn get_details(
    configuration: &ndc::Configuration,
    query: models::QueryRequest,
) -> Result<models::ExplainResponse, connector::ExplainError> {
    let request_url = super::query::build_url(configuration, &query).to_url();

    Ok(models::ExplainResponse {
        details: BTreeMap::from([("query".to_string(), request_url)]),
    })
}
