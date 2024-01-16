pub mod ndc;
pub mod odata;

/// Translate an EDMX document into the ndc-odata metadata type.
pub fn prepare_odata_edmx(metadata: odata::EDMX) -> ndc::Schema {
    let schemata = metadata.data_services.schema;

    let object_types = schemata
        .iter()
        .flat_map(ndc::ObjectType::extract_from)
        .collect();
    let scalar_types = schemata
        .iter()
        .flat_map(ndc::ScalarType::extract_from)
        .collect();
    let functions = schemata
        .iter()
        .flat_map(ndc::Function::extract_from)
        .collect();
    let procedures = schemata
        .iter()
        .flat_map(ndc::Procedure::extract_from)
        .collect();
    let collections = schemata
        .iter()
        .flat_map(ndc::Collection::extract_from)
        .collect();

    ndc::Schema {
        collections,
        object_types,
        scalar_types,
        functions,
        procedures,
    }
}
