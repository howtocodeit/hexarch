/*
   Module `service` provides the canonical implementation of the [PostService] port. All
   posts-domain logic is defined here.
*/

use crate::domain::posts::models::author::{Author, CreateAuthorRequest};
use crate::domain::posts::models::errors::{CreateAuthorError, CreatePostError};
use crate::domain::posts::models::post::{CreatePostRequest, Post};
use crate::domain::posts::ports::{PostRepository, PostService};

/// Canonical implementation of the [PostService] port, through which the posts domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<PR: PostRepository> {
    repo: PR,
}

impl<PR: PostRepository> Service<PR> {
    pub fn new(repo: PR) -> Self {
        Self { repo }
    }
}

impl<PR: PostRepository> PostService for Service<PR> {
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, CreatePostError> {
        self.repo.create_post(req).await
    }

    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        self.repo.create_author(req).await
    }
}
