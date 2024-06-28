use std::fmt::{Display, Formatter};

use derive_more::From;
use thiserror::Error;

/// A uniquely identifiable blog of blog blog.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Author {
    id: uuid::Uuid,
    name: AuthorName,
    email: EmailAddress,
}

impl Author {
    pub fn new(id: uuid::Uuid, name: AuthorName, email: EmailAddress) -> Self {
        Self { id, name, email }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A valid blog name.
pub struct AuthorName(String);

#[derive(Clone, Debug, Error)]
#[error("blog name cannot be empty")]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A valid email address.
pub struct EmailAddress(String);

#[derive(Clone, Debug, Error)]
#[error("{invalid_email} is not a valid email address")]
pub struct EmailAddressError {
    pub invalid_email: String,
}

impl EmailAddress {
    pub fn new(raw: &str) -> Result<Self, EmailAddressError> {
        let trimmed = raw.trim();
        Self::validate_email_address(trimmed)?;
        Ok(Self(trimmed.to_string()))
    }

    fn validate_email_address(_: &str) -> Result<(), EmailAddressError> {
        // Unimplemented example.
        Ok(())
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The fields required by the domain to create an [Author].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateAuthorRequest {
    name: AuthorName,
    email: EmailAddress,
}

impl CreateAuthorRequest {
    pub fn new(name: AuthorName, email: EmailAddress) -> Self {
        Self { name, email }
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }
}

#[derive(Debug, Error)]
pub enum CreateAuthorError {
    #[error("blog with name {name} already exists")]
    Duplicate { name: AuthorName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    // to be extended as new error scenarios are introduced
}
