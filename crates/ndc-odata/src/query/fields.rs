use ndc_sdk::models;
use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Original(pub String);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Alias(pub String);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Fields {
    pub columns: BTreeMap<Alias, Original>,
    pub relationships: BTreeMap<Alias, Relationship>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Relationship {
    pub query: Fields,
    pub relationship: String,
}

impl Fields {
    pub fn from_user_query(query: &models::Query) -> Option<Self> {
        let mut columns = BTreeMap::new();
        let mut relationships = BTreeMap::new();

        for (field_name, field_specification) in query.fields.as_ref()? {
            match field_specification {
                models::Field::Column { column } => {
                    columns.insert(Alias(field_name.clone()), Original(column.clone()));
                }

                models::Field::Relationship {
                    query,
                    relationship,
                    arguments: _,
                } => {
                    let query = Fields::from_user_query(&*query)?;
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

        Some(Fields {
            columns,
            relationships,
        })
    }

    fn from_odata_response(
        &self,
        response: &serde_json::Map<String, serde_json::Value>,
    ) -> serde_json::Map<String, serde_json::Value> {
        let mut record = serde_json::Map::new();

        for (Alias(output_name), Original(odata_name)) in &self.columns {
            let value = response.get(odata_name).unwrap_or(&serde_json::Value::Null);
            record.insert(output_name.clone(), value.clone());
        }

        for (
            Alias(output_name),
            Relationship {
                query,
                relationship,
            },
        ) in &self.relationships
        {
            if let Some(serde_json::Value::Object(object)) = response.get(relationship) {
                let value = serde_json::Value::Object(query.from_odata_response(object));
                record.insert(output_name.clone(), value.clone());
            }
        }

        record
    }
}
