use derive_more::From;

use crate::common::newtypes::TrimmedString;

/// A uniquely identifiable author of blog posts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Author {
    id: uuid::Uuid,
    name: AuthorName,
}

impl Author {
    pub fn new(id: uuid::Uuid, name: &str) -> Self {
        Self {
            id,
            name: AuthorName::new(name),
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }
}

/// A validated and formatted name.
pub type AuthorName = TrimmedString;

/// The fields required by the domain to create an [Author].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct CreateAuthorRequest {
    name: AuthorName,
}

impl CreateAuthorRequest {
    pub fn new(name: AuthorName) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }
}
