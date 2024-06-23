use serde::Serialize;

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}
