use crate::ndc;
use crate::odata::{functions, schema};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: BTreeMap<String, ndc::Type>,
    pub result_type: ndc::Type,
}

impl Function {
    pub fn extract_from(schema: &schema::Schema) -> Vec<ndc::Function> {
        let mut functions = Vec::new();

        for function in &schema.functions {
            let mut arguments = BTreeMap::new();
            let result_type = super::Type::from_type_data(&function.return_type);

            for functions::Parameter { name, r#type } in &function.parameters {
                arguments.insert(name.clone(), super::Type::from_type_data(r#type));
            }

            functions.push(ndc::Function {
                name: function.name.clone(),
                arguments,
                result_type,
            });
        }

        functions
    }
}
