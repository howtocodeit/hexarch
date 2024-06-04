/*
   Module `create_author` specifies an HTTP handler for creating a new [Author], and the
   associated data structures.
*/

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::domain::posts::models::author::{Author, CreateAuthorRequest};
use crate::domain::posts::models::errors::CreateAuthorError;
use crate::domain::posts::ports::PostService;
use crate::inbound::http::AppState;
use crate::inbound::http::responses::{ErrorResponseBody, ErrorResponseData, ResponseBody};

/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CreateAuthorHttpRequestBody {
    name: String,
}

impl CreateAuthorHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    fn into_domain(self) -> CreateAuthorRequest {
        CreateAuthorRequest::new(self.name.into())
    }
}

/// The response body data field for successful [Author] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CreateAuthorResponseData {
    id: String,
    name: String,
}

type CreateAuthorResponseBody = ResponseBody<CreateAuthorResponseData>;

impl From<Author> for CreateAuthorResponseBody {
    fn from(author: Author) -> Self {
        ResponseBody::new(
            StatusCode::CREATED,
            CreateAuthorResponseData {
                id: author.id().to_string(),
                name: author.name().to_string(),
            },
        )
    }
}

impl From<CreateAuthorError> for ErrorResponseBody {
    fn from(e: CreateAuthorError) -> Self {
        match e {
            CreateAuthorError::Duplicate { name } => ResponseBody::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponseData {
                    message: format!("author with name {} already exists", name),
                },
            ),
        }
    }
}

/// Create a new [Author].
///
/// # Responses
///
/// - 201 Created: the [Author] was successfully created.
/// - 422 Unprocessable entity: An [Author] with the same name already exists.
pub async fn create_author<PS: PostService>(
    State(state): State<AppState<PS>>,
    Json(body): Json<CreateAuthorHttpRequestBody>,
) -> Response {
    state
        .post_service
        .create_author(body.into_domain())
        .await
        .map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponseBody::from(e)),
            )
        })
        .map(|author| {
            (
                StatusCode::CREATED,
                Json(CreateAuthorResponseBody::from(author)),
            )
        })
        .into_response()
}
