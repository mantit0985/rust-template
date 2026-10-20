use crate::features::example_feature::model::{ExampleRequest, ExampleResponse};
use crate::features::example_feature::service::ExampleService;
use anyhow::Result;

pub async fn handle_request(req: ExampleRequest) -> Result<ExampleResponse> {
    let service = ExampleService;
    service.handle(req).await
}
