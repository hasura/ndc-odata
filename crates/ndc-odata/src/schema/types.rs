use metadata::ndc;
use ndc_sdk::models;
use std::collections::BTreeMap;

/// Translate an `ndc-odata` type into an `ndc-spec` type. They're structured identically.
pub fn translate_type(r#type: &ndc::Type) -> models::Type {
    match r#type {
        ndc::Type::Qualified { qualified_type } => models::Type::Named {
            name: format!("{}.{}", qualified_type.schema, qualified_type.name)
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
