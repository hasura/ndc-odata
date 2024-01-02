pub mod request;

use http::Uri;
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

    // TODO: I don't feel good about the parsing here - we should probably do this when we validate
    // the configuration, given that we're going to have to do a lot of fumbling around with URLs
    // to make sure everything works.
    let uri = configuration.api_endpoint.parse::<Uri>().unwrap();

    builder
        .set_protocol(uri.scheme_str().unwrap())
        .set_host(uri.host().unwrap())
        .add_route(uri.path());

    request::request_to_url(&mut builder, &request);

    let built = builder.build();

    println!("{}", built);
    let body: Response = reqwest::get(built)
        .await
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?
        .json()
        .await
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?;

    let rows = body.value.into_iter().map(prepare_row).collect();

    Ok(models::QueryResponse(Vec::from([models::RowSet {
        aggregates: None,
        rows: Some(rows),
    }])))
}

fn prepare_row(result: BTreeMap<String, Value>) -> IndexMap<String, models::RowFieldValue> {
    result
        .into_iter()
        .map(|(key, value)| (key, models::RowFieldValue(value)))
        .collect()
}
