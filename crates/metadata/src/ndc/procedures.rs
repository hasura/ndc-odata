//! Functions relating to extracting procedures from OData metadata.

use crate::ndc;
use crate::odata::{functions, schema};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A description of a procedure in `ndc-odata` metadata.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct Procedure {
    pub name: String,
    pub arguments: BTreeMap<String, ndc::Type>,
    pub result_type: ndc::Type,
}

impl Procedure {
    pub fn extract_from(schema: &schema::Schema) -> Vec<ndc::Procedure> {
        let mut procedures = Vec::new();

        for action in &schema.actions {
            if let Some(return_type) = &action.return_type {
                let mut arguments = BTreeMap::new();
                let result_type = super::types::Type::from_type_data(&return_type);

                for functions::Parameter { name, r#type } in &action.parameters {
                    arguments.insert(name.clone(), ndc::types::Type::from_type_data(&r#type));
                }

                procedures.push(ndc::Procedure {
                    name: action.name.clone(),
                    arguments,
                    result_type,
                });
            }
        }

        procedures
    }
}
