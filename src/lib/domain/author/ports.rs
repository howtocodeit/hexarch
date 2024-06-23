/*
   Module `ports` specifies the API by which external modules interact with the author domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/

use std::future::Future;

use crate::domain::author::models::author::{Author, CreateAuthorRequest};
#[allow(unused_imports)] // AuthorName is used in doc comments
use crate::domain::author::models::author::AuthorName;
use crate::domain::author::models::errors::CreateAuthorError;

/// `AuthorService` is the public API for the author domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait AuthorService: Clone + Send + Sync + 'static {
    /// Asynchronously create a new [Author].
    ///
    /// # Errors
    ///
    /// - [CreateAuthorError::Duplicate] if an [Author] with the same [AuthorName] already exists.
    fn create_author(
        &self,
        req: &CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}

/// `AuthorRepository` represents a store of author data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait AuthorRepository: Send + Sync + Clone + 'static {
    /// Asynchronously persist a new [Author].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateAuthorError::Duplicate] if an [Author] with the same [AuthorName]
    ///   already exists.
    fn create_author(
        &self,
        req: &CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}
