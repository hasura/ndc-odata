use ndc_sdk::models;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct OrderBy(pub Vec<OrderByElement>);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct OrderByElement {
    pub order_direction: models::OrderDirection,
    pub target: String,
}

impl OrderBy {
    pub fn from_user_query(query: &models::Query) -> Option<Self> {
        let mut order_by = Vec::new();

        for element in &query.order_by.as_ref()?.elements {
            order_by.push(OrderByElement {
                order_direction: element.order_direction,
                target: match &element.target {
                    models::OrderByTarget::Column { name, path } if path.len() == 0 => name.clone(),
                    _ => return None,
                },
            });
        }

        Some(OrderBy(order_by))
    }
}
