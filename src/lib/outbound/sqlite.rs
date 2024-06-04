use std::str::FromStr;

use anyhow::Context;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

use crate::domain::posts::models::author::{Author, CreateAuthorRequest};
use crate::domain::posts::models::errors::{CreateAuthorError, CreatePostError};
use crate::domain::posts::models::post::{CreatePostRequest, Post};
use crate::domain::posts::ports::PostRepository;

pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> Result<Sqlite, anyhow::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }
}

impl PostRepository for Sqlite {
    async fn create_post(&self, req: CreatePostRequest) -> Result<Post, CreatePostError> {
        todo!()
    }

    async fn create_author(&self, req: CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        todo!()
    }
}
