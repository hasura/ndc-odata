use ndc_odata::OData;
use ndc_sdk::default_main::default_main;

#[tokio::main]
pub async fn main() {
    default_main::<OData>().await.unwrap()
}
