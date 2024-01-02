use itertools::Itertools;
use ndc_sdk::models;
use url_builder::URLBuilder;

pub fn request_to_url(builder: &mut URLBuilder, request: &models::QueryRequest) {
    builder.add_route(&request.collection);

    if let Some(fields) = &request.query.fields {
        builder.add_param(
            "$select",
            &fields
                .values()
                .filter_map(|field| match field {
                    models::Field::Column { column } => Some(column),
                    models::Field::Relationship {
                        query: _,
                        relationship: _,
                        arguments: _,
                    } => None,
                })
                .join(", "),
        );
    }

    if let Some(limit) = request.query.limit {
        builder.add_param("$top", &limit.to_string());
    }

    if let Some(offset) = request.query.offset {
        builder.add_param("$skip", &offset.to_string());
    }

    if let Some(models::OrderBy { elements }) = &request.query.order_by {
        let mut components = elements.iter().filter_map(order_element_to_param);
        builder.add_param("$orderby", &components.join(", "));
    }
}

fn order_element_to_param(element: &models::OrderByElement) -> Option<String> {
    match &element.target {
        models::OrderByTarget::Column { name, path: _ } => format!(
            "{} {}",
            name,
            match element.order_direction {
                models::OrderDirection::Asc => "asc",
                models::OrderDirection::Desc => "desc",
            }
        )
        .into(),
        models::OrderByTarget::StarCountAggregate { path: _ } => None,
        models::OrderByTarget::SingleColumnAggregate {
            column: _,
            function: _,
            path: _,
        } => None,
    }
}
