/*!
   Module `service` provides the canonical implementation of the [BlogService] port. All
   blog-domain logic is defined here.
*/

use crate::domain::blog::models::author::{Author, CreateAuthorRequest};
use crate::domain::blog::models::author::CreateAuthorError;
use crate::domain::blog::ports::{AuthorNotifier, BlogMetrics, BlogRepository, BlogService};

/// Canonical implementation of the [BlogService] port, through which the blog domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R, M, N>
where
    R: BlogRepository,
    M: BlogMetrics,
    N: AuthorNotifier,
{
    repo: R,
    metrics: M,
    author_notifier: N,
}

impl<R, M, N> Service<R, M, N>
where
    R: BlogRepository,
    M: BlogMetrics,
    N: AuthorNotifier,
{
    pub fn new(repo: R, metrics: M, author_notifier: N) -> Self {
        Self {
            repo,
            metrics,
            author_notifier,
        }
    }
}

impl<R, M, N> BlogService for Service<R, M, N>
where
    R: BlogRepository,
    M: BlogMetrics,
    N: AuthorNotifier,
{
    /// Create the [Author] specified in `req` and trigger notifications.
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateAuthorError] returned by the [BlogRepository].
    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let result = self.repo.create_author(req).await;
        if result.is_err() {
            self.metrics.record_author_creation_failure().await;
        } else {
            self.metrics.record_author_creation_success().await;
            self.author_notifier
                .author_created(result.as_ref().unwrap())
                .await;
        }

        result
    }
}
