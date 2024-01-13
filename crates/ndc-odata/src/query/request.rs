use itertools::Itertools;
use metadata::ndc;
use std::collections::BTreeSet;
use url_builder::URLBuilder;

/// The structure of an OData request that we can convert into a URL.
/// @TODO: deal with $expand subfields.
pub struct Request {
    pub endpoint: ndc::Endpoint,
    pub collection: String,
    pub fields: Option<BTreeSet<String>>,
    pub order_by: Option<Vec<(String, Direction)>>,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub enum Direction {
    Ascending,
    Descending,
}

impl Request {
    /// Convert a structured `Request` into an OData URL. This should be used by both the
    /// `/explain` endpoint and the `/query` endpoint.
    pub fn to_url(&self) -> String {
        let mut builder = URLBuilder::new();

        builder
            .set_protocol(&self.endpoint.protocol)
            .set_host(&self.endpoint.authority)
            .add_route(&self.endpoint.path)
            .add_route(&self.collection);

        if let Some(fields) = &self.fields {
            builder.add_param("$select", &fields.iter().join(", "));
        }

        if let Some(elements) = &self.order_by {
            builder.add_param(
                "$orderby",
                &elements
                    .iter()
                    .map(|(name, direction)| {
                        let direction_ = match direction {
                            Direction::Ascending => "asc",
                            Direction::Descending => "desc",
                        };

                        format!("{name} {direction_}")
                    })
                    .join(", "),
            );
        }

        if let Some(limit) = self.limit {
            builder.add_param("$top", &limit.to_string());
        }

        if let Some(offset) = self.offset {
            builder.add_param("$skip", &offset.to_string());
        }

        builder.build()
    }
}
