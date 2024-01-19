use itertools::Itertools;
use ndc_sdk::models;

#[derive(Eq, PartialEq)]
pub enum Filter {
    And {
        expressions: Vec<Filter>,
    },

    Or {
        expressions: Vec<Filter>,
    },

    Not {
        expression: Box<Filter>,
    },

    IsNull {
        column: String,
    },
    Equals {
        column: String,
        comparison_value: ComparisonValue,
    },
}

#[derive(Eq, PartialEq)]
pub enum ComparisonValue {
    Column { column: String },
    Scalar { value: serde_json::Value },
}

impl Filter {
    pub fn from_user_query(query: &models::Query) -> Result<Option<Self>, String> {
        match &query.predicate {
            Some(predicate) => Self::from_predicate(predicate).map(Some),
            None => Ok(None),
        }
    }

    fn from_predicate(expression: &models::Expression) -> Result<Self, String> {
        match expression {
            models::Expression::And { expressions } => {
                let mut prepared = Vec::new();

                for predicate in expressions {
                    prepared.push(Self::from_predicate(predicate)?);
                }

                Ok(Filter::And {
                    expressions: prepared,
                })
            }

            models::Expression::Or { expressions } => {
                let mut prepared = Vec::new();

                for predicate in expressions {
                    prepared.push(Self::from_predicate(predicate)?);
                }

                Ok(Filter::Or {
                    expressions: prepared,
                })
            }

            models::Expression::Not { expression } => Ok(Filter::Not {
                expression: Box::new(Self::from_predicate(expression)?),
            }),

            models::Expression::UnaryComparisonOperator {
                column,
                operator: _,
            } => match column {
                models::ComparisonTarget::Column { name, path: _ } => Ok(Filter::IsNull {
                    column: name.clone(),
                }),
                models::ComparisonTarget::RootCollectionColumn { name: _ } => {
                    Err("Root comparisons are not yet implemented.".to_string())
                }
            },

            models::Expression::BinaryArrayComparisonOperator {
                column: _,
                operator: _,
                values: _,
            } => Err("Binary array comparison operators are not yet supported.".to_string()),

            models::Expression::Exists {
                in_collection: _,
                predicate: _,
            } => Err("Existential queries are not yet supported.".to_string()),

            models::Expression::BinaryComparisonOperator {
                column,
                operator,
                value,
            } => {
                if operator != &models::BinaryComparisonOperator::Equal {
                    return Err("Non-equality filtering is not yet implemented.".to_string());
                }

                let comparison_value = match value {
                    models::ComparisonValue::Column { column } => match column {
                        models::ComparisonTarget::Column { name, path } => {
                            if !path.is_empty() {
                                return Err("Column paths not yet supported.".to_string());
                            }

                            ComparisonValue::Column {
                                column: name.clone(),
                            }
                        }

                        models::ComparisonTarget::RootCollectionColumn { name: _ } => {
                            return Err(
                                "Root references in filters are not yet supported.".to_string()
                            );
                        }
                    },
                    models::ComparisonValue::Scalar { value } => ComparisonValue::Scalar {
                        value: value.clone(),
                    },
                    models::ComparisonValue::Variable { name: _ } => {
                        return Err("Filtering with variables is not yet implemented.".to_string())
                    }
                };

                let column = match column {
                    models::ComparisonTarget::Column { name, path } => {
                        if !path.is_empty() {
                            return Err("Column paths not yet supported.".to_string());
                        }

                        name.clone()
                    }

                    models::ComparisonTarget::RootCollectionColumn { name: _ } => {
                        return Err("Root comparisons are not yet implemented.".to_string())
                    }
                };

                Ok(Filter::Equals {
                    column,
                    comparison_value,
                })
            }
        }
    }

    pub fn to_odata_filter(&self) -> String {
        match self {
            Filter::And { expressions } => {
                let subexpressions = expressions
                    .iter()
                    .map(|expression| expression.to_odata_filter())
                    .join(" and ");

                format!("({})", subexpressions)
            }

            Filter::Or { expressions } => {
                let subexpressions = expressions
                    .iter()
                    .map(|expression| expression.to_odata_filter())
                    .join(" or ");

                format!("({})", subexpressions)
            }

            Filter::Not { expression } => {
                format!("(not {})", expression.to_odata_filter())
            }

            Filter::IsNull { column } => {
                format!("({} eq null)", column)
            }

            Filter::Equals {
                column,
                comparison_value,
            } => match comparison_value {
                ComparisonValue::Column { column } => format!("({} eq {})", column, column.clone()),
                ComparisonValue::Scalar {
                    value: serde_json::Value::String(s),
                } => format!("({} eq '{}')", column, s.clone()),
                ComparisonValue::Scalar { value } => {
                    format!("({} eq {})", column, value)
                }
            },
        }
    }
}
