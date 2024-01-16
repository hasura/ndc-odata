use itertools::Itertools;
use ndc_sdk::models;
use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
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
        self.fields
            .columns
            .values()
            .map(|super::Original(name)| name)
            .collect()
    }

    pub fn prepare_parameters(&self) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();

        let query_fields = &self.odata_fields().iter().join(", ");
        parameters.insert("$select".to_string(), query_fields.clone());

        if self.fields.relationships.len() > 0 {
            let mut expansions = Vec::new();

            // A bit of a nuisance - query components are separated in the top-level query with
            // `&`, but in every subquery with `;`, so we can't just use our beloved URL builder.
            for expansion in self.fields.relationships.values() {
                let mut components = Vec::new();

                for (subquery_key, subquery_value) in Self::prepare_parameters(&expansion.query) {
                    components.push(format!("{subquery_key}={subquery_value}"));
                }

                let joined = components.iter().join(";");
                expansions.push(format!("{}({})", expansion.relationship, joined));
            }

            parameters.insert("$expand".to_string(), expansions.join(","));
        }

        if let Some(super::order_by::OrderBy(elements)) = &self.order_by {
            parameters.insert(
                "$orderby".to_string(),
                elements
                    .iter()
                    .map(|order_by_element| {
                        let direction_ = match order_by_element.order_direction {
                            models::OrderDirection::Asc => "asc",
                            models::OrderDirection::Desc => "desc",
                        };

                        format!("{} {direction_}", order_by_element.target)
                    })
                    .join(", ")
                    .clone(),
            );
        }

        if let Some(limit) = self.limit {
            parameters.insert("$top".to_string(), limit.to_string());
        }

        if let Some(offset) = self.offset {
            parameters.insert("$skip".to_string(), offset.to_string());
        }

        parameters
    }
}
