/*!
   Module `service` provides the canonical implementation of the [AuthorService] port. All
   author-domain logic is defined here.
*/

use crate::domain::author::models::author::{Author, CreateAuthorRequest};
use crate::domain::author::models::errors::CreateAuthorError;
use crate::domain::author::ports::{AuthorRepository, AuthorService};

/// Canonical implementation of the [AuthorService] port, through which the author domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<AR: AuthorRepository> {
    repo: AR,
}

impl<AR: AuthorRepository> Service<AR> {
    pub fn new(repo: AR) -> Self {
        Self { repo: repo }
    }
}

impl<AR: AuthorRepository> AuthorService for Service<AR> {
    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        self.repo.create_author(req).await
    }
}
