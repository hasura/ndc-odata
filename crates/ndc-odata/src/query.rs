//! The query "engine": creates requests and interprets responses.

pub mod request;

use indexmap::IndexMap;
use metadata::ndc;
use ndc_sdk::{connector, models};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Translate an `ndc-spec` request into an `OData` request that we can then convert into a URL.
/// This method is used by both the `/explain` and `/query` endpoints.
pub fn translate_request(
    configuration: &ndc::Configuration,
    request: models::QueryRequest,
) -> request::Request {
    let fields = request.query.fields.map(|fields| {
        fields
            .values()
            .filter_map(|field| match field {
                models::Field::Column { column } => Some(column.clone()),
                _ => None,
            })
            .collect()
    });

    let order_by = request.query.order_by.map(|order_by| {
        order_by
            .elements
            .iter()
            .filter_map(|element| match &element.target {
                models::OrderByTarget::Column { name, path: _ } => Some((
                    name.clone(),
                    match element.order_direction {
                        models::OrderDirection::Asc => request::Direction::Ascending,
                        models::OrderDirection::Desc => request::Direction::Descending,
                    },
                )),
                _ => None,
            })
            .collect()
    });

    request::Request {
        endpoint: configuration.api_endpoint.clone(),
        collection: request.collection,
        fields,
        order_by,
        limit: request.query.limit,
        offset: request.query.offset,
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub value: Vec<BTreeMap<String, Value>>,
}

pub async fn execute_query(
    configuration: &ndc::Configuration,
    request: models::QueryRequest,
) -> Result<models::QueryResponse, connector::QueryError> {
    let url = translate_request(configuration, request).to_url();

    let body: Response = reqwest::get(url)
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

fn prepare_row(row: BTreeMap<String, Value>) -> IndexMap<String, models::RowFieldValue> {
    let mut row_field_values = IndexMap::new();

    for (field_name, field_value) in row {
        row_field_values.insert(field_name.clone(), models::RowFieldValue(field_value));
    }

    row_field_values
}
