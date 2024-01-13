mod capabilities;
mod configuration;
mod explain;
mod health_check;
mod query;
mod schema;

use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::{connector, models};

#[derive(Clone, Default)]
pub struct OData {}

#[async_trait::async_trait]
impl connector::Connector for OData {
    type RawConfiguration = metadata::ndc::RawConfiguration;
    type Configuration = metadata::ndc::Configuration;
    type State = ();

    fn make_empty_configuration() -> Self::RawConfiguration {
        Default::default()
    }

    async fn update_configuration(
        configuration: Self::RawConfiguration,
    ) -> Result<Self::RawConfiguration, connector::UpdateConfigurationError> {
        configuration::update_configuration(configuration).await
    }

    async fn validate_raw_configuration(
        configuration: Self::RawConfiguration,
    ) -> Result<Self::Configuration, connector::ValidateError> {
        configuration::validate_raw_configuration(configuration).await
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
        health_check::health_check(&configuration.api_endpoint).await
    }

    async fn get_capabilities() -> JsonResponse<models::CapabilitiesResponse> {
        capabilities::get_capabilities().into()
    }

    async fn get_schema(
        configuration: &Self::Configuration,
    ) -> Result<JsonResponse<models::SchemaResponse>, connector::SchemaError> {
        Ok(schema::get_schema(configuration).into())
    }

    async fn explain(
        configuration: &Self::Configuration,
        _state: &Self::State,
        query: models::QueryRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>, connector::ExplainError> {
        Ok(explain::get_details(configuration, query)?.into())
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
