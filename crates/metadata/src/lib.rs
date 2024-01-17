/// A library for parsing OData Common Schema Definition Language (CSDL) into NDC metadata.
pub mod ndc;
pub mod odata;

/// Translate an EDMX document into the ndc-odata metadata type.
pub fn prepare_odata_edmx(metadata: odata::EDMX) -> ndc::Schema {
    ndc::Schema {
        scalar_types: metadata
            .data_services
            .schema
            .iter()
            .flat_map(ndc::ScalarType::extract_from)
            .collect(),
        functions: metadata
            .data_services
            .schema
            .iter()
            .flat_map(ndc::Function::extract_from)
            .collect(),
        procedures: metadata
            .data_services
            .schema
            .iter()
            .flat_map(ndc::Procedure::extract_from)
            .collect(),
        collections: metadata
            .data_services
            .schema
            .iter()
            .flat_map(|schema| ndc::Collection::extract_from(&metadata, schema))
            .collect(),
        object_types: metadata
            .data_services
            .schema
            .iter()
            .flat_map(|schema| ndc::ObjectType::extract_from(&metadata, schema))
            .collect(),
    }
}
