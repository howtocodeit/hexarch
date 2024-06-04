use crate::common::newtypes::TrimmedString;

/// A blog post.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    id: uuid::Uuid,
    title: PostTitle,
    content: String,
    author_id: uuid::Uuid,
}

/// The validated, formatted title of a blog post.
pub type PostTitle = TrimmedString;

/// The fields required by the domain to create a [Post].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreatePostRequest {
    title: PostTitle,
    content: String,
    author_id: uuid::Uuid,
}

impl CreatePostRequest {
    pub fn new(title: PostTitle, content: String, author_id: uuid::Uuid) -> Self {
        Self {
            title,
            content,
            author_id,
        }
    }
}
