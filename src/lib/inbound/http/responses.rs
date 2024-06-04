use axum::http;
use serde::Serialize;

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> ResponseBody<T> {
    pub fn new(status_code: http::StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}

pub type ErrorResponseBody = ResponseBody<ErrorResponseData>;
