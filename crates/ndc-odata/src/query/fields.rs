use ndc_sdk::models;
use std::collections::BTreeMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Original(pub String);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Alias(pub String);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Fields {
    pub columns: BTreeMap<Alias, Original>,
    pub relationships: BTreeMap<Alias, Relationship>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Relationship {
    pub query: super::Query,
    pub relationship: String,
}

impl Fields {
    pub fn from_user_query(query: &models::Query) -> Result<Self, String> {
        let mut columns = BTreeMap::new();
        let mut relationships = BTreeMap::new();

        if let Some(fields) = &query.fields {
            for (field_name, field_specification) in fields {
                match field_specification {
                    models::Field::Column { column } => {
                        columns.insert(Alias(field_name.clone()), Original(column.clone()));
                    }

                    models::Field::Relationship {
                        query,
                        relationship,
                        arguments: _,
                    } => {
                        let query = super::Query::from_user_query(&*query)?;
                        let relationship = relationship.clone();

                        relationships.insert(
                            Alias(field_name.clone()),
                            Relationship {
                                query,
                                relationship,
                            },
                        );
                    }
                }
            }
        }

        Ok(Fields {
            columns,
            relationships,
        })
    }
}
