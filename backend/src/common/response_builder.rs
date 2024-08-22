use crate::common::response::{ApiResponse, ResponseContent};

#[derive(Default)]
pub struct ApiResponseBuilder {
    status: String,
    result: Option<ResponseContent>,
}

impl ApiResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: String) -> Self {
        self.status = status;
        self
    }

    pub fn result(mut self, result: ResponseContent) -> Self {
        self.result = Some(result);
        self
    }

    pub fn build(self) -> ApiResponse {
        ApiResponse {
            status: self.status,
            result: self.result,
        }
    }
}
