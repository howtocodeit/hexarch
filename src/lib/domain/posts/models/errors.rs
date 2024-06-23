/*
   Module `errors` specifies the error types that outbound adapters are permitted to return, and
   which inbound adapters must be prepared to handle.

   Error enums are specified exhaustively, and adapters must be prepared to update their error
   handling logic as new error scenarios are introduced.
*/

use thiserror::Error;

use crate::domain::posts::models::author::AuthorName;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CreatePostError {
    #[error("author with id {id} was not found")]
    AuthorNotFound { id: uuid::Uuid },
    // to be extended as new error scenarios are introduced
}

#[derive(Debug, Error)]
pub enum CreateAuthorError {
    #[error("author with name {name} already exists")]
    Duplicate { name: AuthorName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    // to be extended as new error scenarios are introduced
}
