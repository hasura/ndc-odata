use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "@odata.context")]
    pub context: String,

    #[serde(default)]
    pub value: Vec<Map<String, Value>>,
}

impl Response {
    pub fn interpret(
        result_row: &Map<String, Value>,
        query_structure: &super::Query
    ) -> Map<String, Value> {
        let mut prepared = Map::new();
        println!("{:?}", result_row);

        for (alias, original) in &query_structure.fields.columns {
            println!("{:?} -> {:?}", original, alias);

            if let Some(value) = result_row.get(original.0.as_str()) {
                prepared.insert(alias.0.clone(), value.clone());
            }
        }

        for (alias, expansion) in &query_structure.fields.relationships {
            println!("{:?} -> {:?}", alias, expansion.relationship);

            if let Some(Value::Object(obj)) = result_row.get(&expansion.relationship) {
                let subresponse = Response::interpret(obj, &expansion.query);
                prepared.insert(alias.0.clone(), Value::Object(subresponse));
            }
        }

        prepared
    }
}
