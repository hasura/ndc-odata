use metadata::ndc;
use ndc_sdk::models;

pub fn translate(function: &ndc::Function) -> models::FunctionInfo {
    models::FunctionInfo {
        name: function.name.clone(),
        arguments: super::types::translate_arguments(&function.arguments),
        result_type: super::types::translate_type(&function.result_type),
        description: None,
    }
}
