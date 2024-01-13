use metadata::ndc::ScalarType;
use metadata::ndc;
use ndc_sdk::models;
use std::collections::{BTreeMap, BTreeSet};

pub fn scalar_types(scalar_types: &BTreeSet<ScalarType>) -> BTreeMap<String, models::ScalarType> {
    let mut results = BTreeMap::new();

    for ScalarType(scalar_type) in scalar_types {
        results.insert(
            scalar_type.clone(),
            models::ScalarType {
                aggregate_functions: BTreeMap::new(),
                comparison_operators: BTreeMap::new(),
            },
        );
    }

    results
}

/// Translate an `ndc-odata` type into an `ndc-spec` type. They're structured identically.
pub fn translate_type(r#type: &ndc::Type) -> models::Type {
    match r#type {
        ndc::Type::Named { name } => models::Type::Named {
            name: name.to_string(),
        },

        ndc::Type::Nullable { underlying_type } => models::Type::Nullable {
            underlying_type: Box::new(translate_type(&*underlying_type)),
        },

        ndc::Type::Collection { element_type } => models::Type::Array {
            element_type: Box::new(translate_type(&*element_type)),
        },
    }
}

/// Translate a set of `ndc-odata` arguments to `ndc-spec` arguments. OData doesn't provide a
/// description for arguments to its functions or procedures, so this function really just adds
/// empty descriptions to each argument.
pub fn translate_arguments(
    arguments: &BTreeMap<String, ndc::Type>,
) -> BTreeMap<String, models::ArgumentInfo> {
    let mut results = BTreeMap::new();

    for (argument_name, argument_type) in arguments {
        results.insert(
            argument_name.clone(),
            models::ArgumentInfo {
                argument_type: translate_type(&argument_type),
                description: None,
            },
        );
    }

    results
}
