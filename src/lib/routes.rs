use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sqlite: Arc<SqlitePool>,
}

pub struct ApiSuccess<T: Serialize>((StatusCode, Json<ApiResponseBody<T>>));

impl<T: Serialize> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess((status, Json(ApiResponseBody::new(status, data))))
    }
}

impl<T: Serialize> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(anyhow::Error),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e)
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
pub struct ApiResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> ApiResponseBody<T> {
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

pub type ErrorResponseBody = ApiResponseBody<ApiErrorData>;

pub type ErrorResponse = (StatusCode, ErrorResponseBody);

/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateAuthorRequestBody {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateAuthorResponseData {
    id: String,
}

pub async fn create_author(
    State(state): State<AppState>,
    Json(author): Json<CreateAuthorRequestBody>,
) -> Result<ApiSuccess<CreateAuthorResponseData>, ApiError> {
    if author.name.is_empty() {
        return Err(ApiError::UnprocessableEntity(
            "author name cannot be empty".to_string(),
        ));
    }

    let mut tx = state
        .sqlite
        .begin()
        .await
        .context("failed to start transaction")?;

    let author_id = save_author(&mut tx, &author.name).await.map_err(|e| {
        if is_unique_constraint_violation(&e) {
            ApiError::UnprocessableEntity(format!(
                "author with name {} already exists",
                &author.name
            ))
        } else {
            anyhow!(e).into()
        }
    })?;

    tx.commit().await.context("failed to commit transaction")?;

    Ok(ApiSuccess::new(
        StatusCode::CREATED,
        CreateAuthorResponseData {
            id: author_id.to_string(),
        },
    ))
}

async fn save_author(tx: &mut Transaction<'_, Sqlite>, name: &str) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let id_as_string = id.to_string();
    let query = sqlx::query!(
        "INSERT INTO authors (id, name) VALUES ($1, $2)",
        id_as_string,
        name
    );
    tx.execute(query).await?;
    Ok(id)
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}
