use metadata::ndc::ScalarType;
use ndc_sdk::models;
use std::collections::BTreeMap;

pub fn translate(scalar_type: &ScalarType) -> (String, models::ScalarType) {
    (
        scalar_type.0.clone(),
        models::ScalarType {
            aggregate_functions: BTreeMap::new(),
            comparison_operators: BTreeMap::new(),
        },
    )
}
