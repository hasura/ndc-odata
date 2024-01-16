use ndc_sdk::models;

pub struct Query {
    pub fields: super::Fields,
    pub order_by: Option<super::OrderBy>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Query {
    // In an OData request, we have to distinguish between fields in the current collection (which
    // we query using `$select`) and fields in a related collection (which we query via `$expand`).
    // This method effectively just separates the two types of field and also strips any parts of
    // the query that we don't support yet.
    pub fn from_user_query(query: &models::Query) -> Result<Self, String> {
        let fields = super::Fields::from_user_query(&query)
            .expect("Only queries with fields are currently supported");

        let order_by = super::OrderBy::from_user_query(&query);

        if let Some(_) = query.aggregates {
            return Err("Aggregation queries are not yet supported.".to_string());
        }

        if let Some(_) = query.predicate {
            return Err("Filtering is not yet supported.".to_string());
        }

        Ok(Query {
            fields,
            limit: query.limit,
            offset: query.offset,
            order_by,
        })
    }

    pub fn odata_fields(&self) -> Vec<&String> {
        self.fields.columns.values().map(|super::Original(name)| name).collect()
    }
}

