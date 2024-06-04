/*
   Module `ports` specifies the API by which external modules interact with the posts domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/

use std::future::Future;

use crate::domain::posts::models::author::{Author, CreateAuthorRequest};
#[allow(unused_imports)] // AuthorName is used in doc comments.
use crate::domain::posts::models::author::AuthorName;
use crate::domain::posts::models::errors::{CreateAuthorError, CreatePostError};
use crate::domain::posts::models::post::{CreatePostRequest, Post};

/// `PostService` is the public API for the posts domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait PostService: Send + Sync + 'static {
    /// Asynchronously create a new [Post].
    ///
    /// # Errors
    ///
    /// - [CreatePostError::AuthorNotFound] if the [Author] specified in the request does not exist.
    fn create_post(
        &self,
        req: CreatePostRequest,
    ) -> impl Future<Output = Result<Post, CreatePostError>> + Send;

    /// Asynchronously create a new [Author].
    ///
    /// # Errors
    ///
    /// - [CreateAuthorError::Duplicate] if an [Author] with the same [AuthorName] already exists.
    fn create_author(
        &self,
        req: CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}

/// `PostRepository` represents a store of post data.
///
/// Repository implementations must conform to this contract – the domain is not concerned with
/// the implementation details or underlying technology of any particular implementation.
pub trait PostRepository: Send + Sync + 'static {
    /// Asynchronously persist a new [Post].
    ///
    /// # Errors
    ///
    /// - MUST return [CreatePostError::AuthorNotFound] if the [Author] specified in the request
    ///   does not exist.
    fn create_post(
        &self,
        req: CreatePostRequest,
    ) -> impl Future<Output = Result<Post, CreatePostError>> + Send;

    /// Asynchronously persist a new [Author].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateAuthorError::Duplicate] if an [Author] with the same [AuthorName]
    ///   already exists.
    fn create_author(
        &self,
        req: CreateAuthorRequest,
    ) -> impl Future<Output = Result<Author, CreateAuthorError>> + Send;
}
