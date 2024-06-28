use crate::domain::blog::models::author::Author;
use crate::domain::blog::ports::AuthorNotifier;

/// An unimplemented example of an adapter to [AuthorNotifier].
#[derive(Debug, Clone)]
pub struct EmailClient;

impl EmailClient {
    pub fn new() -> Self {
        Self
    }
}

impl AuthorNotifier for EmailClient {
    async fn author_created(&self, _: &Author) {}
}
