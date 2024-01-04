pub mod request;

use indexmap::IndexMap;
use metadata::ndc;
use ndc_sdk::{connector, models};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use url_builder::URLBuilder;

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub value: Vec<BTreeMap<String, Value>>,
}

pub async fn execute_query(
    configuration: &ndc::Configuration,
    request: models::QueryRequest,
) -> Result<models::QueryResponse, connector::QueryError> {
    let mut builder = URLBuilder::new();

    builder
        .set_protocol(&configuration.api_endpoint.protocol)
        .set_host(&configuration.api_endpoint.authority)
        .add_route(&configuration.api_endpoint.path);

    request::request_to_url(&mut builder, &request);
    let built = builder.build();

    let body: Response = reqwest::get(built)
        .await
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?
        .json()
        .await
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?;

    Ok(models::QueryResponse(Vec::from([models::RowSet {
        rows: Some(body.value.into_iter().map(prepare_row).collect()),
        aggregates: None,
    }])))
}

fn prepare_row(result: BTreeMap<String, Value>) -> IndexMap<String, models::RowFieldValue> {
    result
        .into_iter()
        .map(|(key, value)| (key, models::RowFieldValue(value)))
        .collect()
}
