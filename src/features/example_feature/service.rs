use crate::features::example_feature::model::{ExampleRequest, ExampleResponse};
use anyhow::Result;

pub struct ExampleService;

impl ExampleService {
    pub async fn handle(&self, req: ExampleRequest) -> Result<ExampleResponse> {
        Ok(ExampleResponse {
            message: format!("Processed request for ID: {}", req.id),
        })
    }
}
