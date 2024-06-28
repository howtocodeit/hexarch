/*
   Module `create_author` specifies an HTTP handler for creating a new [Author], and the
   associated data structures.
*/

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::blog::models::author::{
    Author, AuthorName, AuthorNameEmptyError, CreateAuthorRequest, EmailAddress, EmailAddressError,
};
use crate::domain::blog::models::author::CreateAuthorError;
use crate::domain::blog::ports::BlogService;
use crate::inbound::http::AppState;

#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<CreateAuthorError> for ApiError {
    fn from(e: CreateAuthorError) -> Self {
        match e {
            CreateAuthorError::Duplicate { name } => {
                Self::UnprocessableEntity(format!("blog with name {} already exists", name))
            }
            CreateAuthorError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ParseCreateAuthorHttpRequestError> for ApiError {
    fn from(e: ParseCreateAuthorHttpRequestError) -> Self {
        let message = match e {
            ParseCreateAuthorHttpRequestError::Name(_) => "blog name cannot be empty".to_string(),
            ParseCreateAuthorHttpRequestError::EmailAddress(cause) => {
                format!("email address {} is invalid", cause.invalid_email)
            }
        };

        Self::UnprocessableEntity(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateAuthorRequestBody {
    name: String,
}

/// The response body data field for successful [Author] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateAuthorResponseData {
    id: String,
}

impl From<&Author> for CreateAuthorResponseData {
    fn from(author: &Author) -> Self {
        Self {
            id: author.id().to_string(),
        }
    }
}

/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateAuthorHttpRequestBody {
    name: String,
    email_address: String,
}

#[derive(Debug, Clone, Error)]
enum ParseCreateAuthorHttpRequestError {
    #[error(transparent)]
    Name(#[from] AuthorNameEmptyError),
    #[error(transparent)]
    EmailAddress(#[from] EmailAddressError),
}

impl CreateAuthorHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    fn try_into_domain(self) -> Result<CreateAuthorRequest, ParseCreateAuthorHttpRequestError> {
        let name = AuthorName::new(&self.name)?;
        let email = EmailAddress::new(&self.email_address)?;
        Ok(CreateAuthorRequest::new(name, email))
    }
}

/// Create a new [Author].
///
/// # Responses
///
/// - 201 Created: the [Author] was successfully created.
/// - 422 Unprocessable entity: An [Author] with the same name already exists.
pub async fn create_author<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Json(body): Json<CreateAuthorHttpRequestBody>,
) -> Result<ApiSuccess<CreateAuthorResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .author_service
        .create_author(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref author| ApiSuccess::new(StatusCode::CREATED, author.into()))
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::Arc;

    use anyhow::anyhow;
    use uuid::Uuid;

    use crate::domain::blog::models::author::{Author, CreateAuthorRequest};
    use crate::domain::blog::models::author::CreateAuthorError;
    use crate::domain::blog::ports::BlogService;

    use super::*;

    #[derive(Clone)]
    struct MockBlogService {
        create_author_result: Arc<std::sync::Mutex<Result<Author, CreateAuthorError>>>,
    }

    impl BlogService for MockBlogService {
        async fn create_author(
            &self,
            _: &CreateAuthorRequest,
        ) -> Result<Author, CreateAuthorError> {
            let mut guard = self.create_author_result.lock();
            let mut result = Err(CreateAuthorError::Unknown(anyhow!("substitute error")));
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_author_success() {
        let author_name = AuthorName::new("Angus").unwrap();
        let author_email = EmailAddress::new("angus@howtocodeit.com").unwrap();
        let author_id = Uuid::new_v4();
        let service = MockBlogService {
            create_author_result: Arc::new(std::sync::Mutex::new(Ok(Author::new(
                author_id,
                author_name.clone(),
                author_email.clone(),
            )))),
        };
        let state = axum::extract::State(AppState {
            author_service: Arc::new(service),
        });
        let body = axum::extract::Json(CreateAuthorHttpRequestBody {
            name: author_name.to_string(),
            email_address: author_email.to_string(),
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateAuthorResponseData {
                id: author_id.to_string(),
            },
        );

        let actual = create_author(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_author to succeed, but got {:?}",
            actual
        );

        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        )
    }
}
