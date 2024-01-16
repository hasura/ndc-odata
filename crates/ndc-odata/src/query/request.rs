use metadata::ndc;
use ndc_sdk::models;
use url_builder::URLBuilder;

pub struct Request {
    pub api_endpoint: ndc::Endpoint,
    pub collection: String,
    pub query: super::Query,
}

impl Request {
    pub fn from_user_request(
        configuration: &ndc::Configuration,
        request: &models::QueryRequest,
    ) -> Result<Self, String> {
        Ok(Request {
            api_endpoint: configuration.api_endpoint.clone(),
            collection: request.collection.clone(),
            query: super::Query::from_user_query(&request.query)?,
        })
    }

    pub fn to_url(&self) -> Result<String, String> {
        let mut builder = URLBuilder::new();

        builder
            .set_protocol(&self.api_endpoint.protocol)
            .set_host(&self.api_endpoint.authority)
            .add_route(&self.api_endpoint.path)
            .add_route(&self.collection);

        for (parameter_name, parameter_value) in super::Query::prepare_parameters(&self.query) {
            builder.add_param(&parameter_name, &parameter_value);
        }

        Ok(builder.build())
    }
}
