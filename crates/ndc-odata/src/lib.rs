mod configuration;
mod health_check;
mod query;
mod schema;

use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::{connector, models};
use std::collections::BTreeMap;

#[derive(Clone, Default)]
pub struct OData {}

#[async_trait::async_trait]
impl connector::Connector for OData {
    type RawConfiguration = metadata::ndc::Configuration;
    type Configuration = metadata::ndc::Configuration;
    type State = ();

    fn make_empty_configuration() -> Self::RawConfiguration {
        Default::default()
    }

    async fn update_configuration(
        configuration: Self::RawConfiguration,
    ) -> Result<Self::RawConfiguration, connector::UpdateConfigurationError> {
        Ok(metadata::ndc::Configuration {
            api_endpoint: configuration.api_endpoint.clone(),
            schema: configuration::fetch_metadata(&configuration.api_endpoint).await?,
        })
    }

    async fn validate_raw_configuration(
        configuration: Self::RawConfiguration,
    ) -> Result<Self::Configuration, connector::ValidateError> {
        match configuration.api_endpoint.parse() {
            Ok(uri) => Ok(metadata::ndc::Configuration {
                api_endpoint: uri,
                schema: configuration.schema,
            }),

            Err(_err) => todo!(), // connector::ValidateError::ValidateError(())
        }
    }

    async fn try_init_state(
        _configuration: &Self::Configuration,
        _registry: &mut prometheus::Registry,
    ) -> Result<Self::State, connector::InitializationError> {
        Ok(())
    }

    fn fetch_metrics(
        _configuration: &Self::Configuration,
        _state: &Self::State,
    ) -> Result<(), connector::FetchMetricsError> {
        Ok(())
    }

    async fn health_check(
        configuration: &Self::Configuration,
        _state: &Self::State,
    ) -> Result<(), connector::HealthError> {
        health_check::health_check(&configuration.api_endpoint.to_string()).await
    }

    async fn get_capabilities() -> JsonResponse<models::CapabilitiesResponse> {
        models::CapabilitiesResponse {
            capabilities: models::Capabilities {
                query: models::QueryCapabilities {
                    aggregates: None,
                    variables: None,
                },

                explain: None,
                relationships: None,
            },
            versions: "^0.1.0".to_string(),
        }
        .into()
    }

    async fn get_schema(
        configuration: &Self::Configuration,
    ) -> Result<JsonResponse<models::SchemaResponse>, connector::SchemaError> {
        Ok(schema::get_schema(configuration).into())
    }

    async fn explain(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _query: models::QueryRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>, connector::ExplainError> {
        // TODO: probably the only useful thing we could do here is list the requests we're going
        // to have to make to the OData API?
        Ok(models::ExplainResponse {
            details: BTreeMap::new(),
        }
        .into())
    }

    async fn mutation(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _query: models::MutationRequest,
    ) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
        todo!()
    }

    async fn query(
        configuration: &Self::Configuration,
        _state: &Self::State,
        request: models::QueryRequest,
    ) -> Result<JsonResponse<models::QueryResponse>, connector::QueryError> {
        Ok(query::execute_query(configuration, request).await?.into())
    }
}
