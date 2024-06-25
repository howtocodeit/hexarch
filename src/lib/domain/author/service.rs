/*!
   Module `service` provides the canonical implementation of the [AuthorService] port. All
   author-domain logic is defined here.
*/

use crate::domain::author::models::author::{Author, CreateAuthorRequest};
use crate::domain::author::models::errors::CreateAuthorError;
use crate::domain::author::ports::{
    AuthorMetrics, AuthorNotifier, AuthorRepository, AuthorService,
};

/// Canonical implementation of the [AuthorService] port, through which the author domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R, M, N>
where
    R: AuthorRepository,
    M: AuthorMetrics,
    N: AuthorNotifier,
{
    repo: R,
    metrics: M,
    notifier: N,
}

impl<R, M, N> Service<R, M, N>
where
    R: AuthorRepository,
    M: AuthorMetrics,
    N: AuthorNotifier,
{
    pub fn new(repo: R, metrics: M, notifier: N) -> Self {
        Self {
            repo,
            metrics,
            notifier,
        }
    }
}

impl<R, M, N> AuthorService for Service<R, M, N>
where
    R: AuthorRepository,
    M: AuthorMetrics,
    N: AuthorNotifier,
{
    /// Create the [Author] specified in `req` and trigger notifications.
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateAuthorError] returned by the [AuthorRepository].
    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let result = self.repo.create_author(req).await;
        if result.is_err() {
            self.metrics.record_creation_failure().await;
        } else {
            self.metrics.record_creation_success().await;
            self.notifier.author_created(result.as_ref().unwrap()).await;
        }

        result
    }
}
