//! The query "engine": creates requests and interprets responses.

pub mod fields;
pub mod order_by;
pub mod query;
pub mod request;

pub use fields::*;
pub use order_by::*;
pub use query::*;
pub use request::*;

use indexmap::IndexMap;
use metadata::ndc;
use ndc_sdk::{connector, models};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub value: Vec<BTreeMap<String, Value>>,
}

pub async fn execute_query(
    configuration: &ndc::Configuration,
    request: models::QueryRequest,
) -> Result<models::QueryResponse, connector::QueryError> {
    let request_url = super::query::Request::from_user_request(configuration, &request)
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?
        .to_url()
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?;

    let body: Response = reqwest::get(request_url)
        .await
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?
        .json()
        .await
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?;

    let mut results = Vec::new();

    for result_row in body.value {
        let mut row_field_values = IndexMap::new();

        for (field_name, field_type) in &request.query.fields.clone().unwrap() {
            match field_type {
                models::Field::Column { column } => match result_row.get(column.as_str()) {
                    Some(value) => {
                        row_field_values
                            .insert(field_name.clone(), models::RowFieldValue(value.clone()));
                    }
                    None => (),
                },

                models::Field::Relationship {
                    query: _,
                    relationship,
                    arguments: _,
                } => match result_row.get(relationship.as_str()) {
                    Some(object) => {
                        row_field_values
                            .insert(field_name.clone(), models::RowFieldValue(object.clone()));
                    }
                    None => (),
                },
            }
        }

        results.push(row_field_values);
    }

    Ok(models::QueryResponse(Vec::from([models::RowSet {
        rows: Some(results),
        aggregates: None,
    }])))
}
