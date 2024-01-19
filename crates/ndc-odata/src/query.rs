//! The handler for the `/query` endpoint.

pub mod fields;
pub mod filters;
pub mod order_by;
#[allow(clippy::module_inception)]
pub mod query; // We can remove module inception when we fully move to use NDC requests.
pub mod request;
pub mod response;

pub use fields::*;
pub use filters::*;
pub use order_by::*;
pub use query::*;
pub use request::*;
pub use response::*;

use indexmap::IndexMap;
use metadata::ndc;
use ndc_sdk::{connector, models};

pub async fn execute_query(
    configuration: &ndc::Configuration,
    request: models::QueryRequest,
) -> Result<models::QueryResponse, connector::QueryError> {
    let request_structure = Request::from_user_request(configuration, &request)
        .map_err(Box::from)
        .map_err(connector::QueryError::Other)?;

    let request_url = request_structure
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

    let mut rows = Vec::new();

    for result_row in &body.value {
        let mut row = IndexMap::new();

        for (field, value) in Response::interpret(result_row, &request_structure.query) {
            row.insert(field.clone(), models::RowFieldValue(value));
        }

        rows.push(row);
    }

    let row_set = models::RowSet {
        rows: Some(rows),
        aggregates: None,
    };

    Ok(models::QueryResponse(Vec::from([row_set])))
}
