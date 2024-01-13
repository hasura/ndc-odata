use metadata::ndc;
use ndc_sdk::models;

pub fn translate(procedures: &Vec<ndc::Procedure>) -> Vec<models::ProcedureInfo> {
    let mut results = Vec::new();

    for procedure in procedures {
        results.push(models::ProcedureInfo {
            name: procedure.name.clone(),
            arguments: super::types::translate_arguments(&procedure.arguments),
            result_type: super::types::translate_type(&procedure.result_type),
            description: None,
        });
    }

    results
}
