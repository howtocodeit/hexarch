use std::fmt::{Display, Formatter};

use derive_more::From;
use thiserror::Error;

/// A uniquely identifiable author of blog posts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Author {
    id: uuid::Uuid,
    name: AuthorName,
}

impl Author {
    pub fn new(id: uuid::Uuid, name: AuthorName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorName(String);

#[derive(Clone, Debug, Error)]
#[error("author name cannot be empty")]
pub struct AuthorNameEmptyError;

impl AuthorName {
    pub fn new(raw: &str) -> Result<Self, AuthorNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(AuthorNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Display for AuthorName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The fields required by the domain to create an [Author].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
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
