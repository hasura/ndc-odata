use metadata::ndc;
use ndc_sdk::models;
use std::collections::BTreeMap;

pub fn translate(functions: &Vec<ndc::Function>) -> Vec<models::FunctionInfo> {
    let mut results = Vec::new();

    for function in functions {
        results.push(models::FunctionInfo {
            name: function.name.clone(),
            arguments: super::types::translate_arguments(&function.arguments),
            result_type: super::types::translate_type(&function.result_type),
            description: None,
        });
    }

    results
}
