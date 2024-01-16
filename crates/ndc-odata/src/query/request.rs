use itertools::Itertools;
use metadata::ndc;
use ndc_sdk::models;
use url_builder::URLBuilder;

pub struct Request {
    api_endpoint: ndc::Endpoint,
    collection: String,
    query: super::Query,
}

impl Request {
    pub fn from_user_request(
        configuration: &ndc::Configuration,
        request: &models::QueryRequest
    ) -> Result<Self,String> {
        Ok(Request {
            api_endpoint: configuration.api_endpoint.clone(),
            collection: request.collection.clone(),
            query: super::Query::from_user_query(&request.query)?,
        })
    }

    pub fn to_url(&self) -> Result<String, String> {
        let mut builder = URLBuilder::new();

        builder
            .set_protocol(&self.api_endpoint.protocol)
            .set_host(&self.api_endpoint.authority)
            .add_route(&self.api_endpoint.path)
            .add_route(&self.collection)
            .add_param("$select", &self.query.odata_fields().iter().join(", "));

        if self.query.fields.relationships.len() > 0 {
            let mut expansions = Vec::new();

            for expansion in self.query.fields.relationships.values() {
                let subquery = produce_subquery(&expansion.query);
                expansions.push(format!("{}({})", expansion.relationship, subquery));
            }

            builder.add_param("$expand", expansions.join(",").as_str());
        }

        if let Some(super::order_by::OrderBy(elements)) = &self.query.order_by {
            builder.add_param(
                "$orderby",
                &elements
                    .iter()
                    .map(|order_by_element| {
                        let direction_ = match order_by_element.order_direction {
                            models::OrderDirection::Asc => "asc",
                            models::OrderDirection::Desc => "desc",
                        };

                        format!("{} {direction_}", order_by_element.target)
                    })
                    .join(", "),
            );
        }

        if let Some(limit) = self.query.limit {
            builder.add_param("$top", &limit.to_string());
        }

        if let Some(offset) = self.query.offset {
            builder.add_param("$skip", &offset.to_string());
        }

        Ok(builder.build())
    }
}

fn produce_subquery(query: &super::Fields) -> String {
    // Unwrap all the original names (i.e. the OData ones) and use them to build the `select`
    // query.
    format!("$select={}", query.columns.values().map(|x| &x.0).join(","))
}
