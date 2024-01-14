use metadata::ndc;
use ndc_sdk::models;

pub fn translate(procedure: &ndc::Procedure) -> models::ProcedureInfo {
    models::ProcedureInfo {
        name: procedure.name.clone(),
        arguments: super::types::translate_arguments(&procedure.arguments),
        result_type: super::types::translate_type(&procedure.result_type),
        description: None,
    }
}
