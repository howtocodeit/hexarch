/*
   Module `ports` specifies the API by which external modules interact with the blog domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/

use std::future::Future;

use crate::domain::blog::models::author::{Author, CreateAuthorRequest};
#[allow(unused_imports)] // AuthorName is used in doc comments
use crate::domain::blog::models::author::AuthorName;
use crate::domain::blog::models::author::CreateAuthorError;

/// `BlogService` is the public API for the blog domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait BlogService: Clone + Send + Sync + 'static {
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

/// `BlogRepository` represents a store of blog data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait BlogRepository: Send + Sync + Clone + 'static {
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

/// `BlogMetrics` describes an aggregator of blog-related metrics, such as a time-series
/// database.
pub trait BlogMetrics: Send + Sync + Clone + 'static {
    /// Record a successful author creation.
    fn record_author_creation_success(&self) -> impl Future<Output = ()> + Send;

    /// Record an author creation failure.
    fn record_author_creation_failure(&self) -> impl Future<Output = ()> + Send;
}

/// `AuthorNotifier` triggers notifications to authors.
///
/// Whether or the notification medium (email, SMS, etc.) is known by the business logic is a
/// judgement call based on your use case.
///
/// Some domains will always require email, for example, so hiding this detail would be
/// pointless.
///
/// For others, code coordinating notifications will be complex enough to warrant its own domain.
/// In this case, an `AuthorNotifier` adapter will call that domain's `Service`.
pub trait AuthorNotifier: Send + Sync + Clone + 'static {
    fn author_created(&self, author: &Author) -> impl Future<Output = ()> + Send;
}
