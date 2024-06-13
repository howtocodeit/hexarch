use std::str::FromStr;

use anyhow::Context;
use sqlx::{Executor, SqlitePool, Transaction};
use sqlx::sqlite::SqliteConnectOptions;
use uuid::Uuid;

use crate::domain::posts::models::author::{Author, CreateAuthorRequest};
use crate::domain::posts::models::errors::{CreateAuthorError, CreatePostError};
use crate::domain::posts::models::post::{CreatePostRequest, Post};
use crate::domain::posts::ports::PostRepository;

#[derive(Debug, Clone)]
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

    async fn save_author(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        name: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let query = sqlx::query!(
            "INSERT INTO authors (id, name) VALUES ($1, $2)",
            id_as_string,
            name
        );
        tx.execute(query).await?;
        Ok(id)
    }
}

impl PostRepository for Sqlite {
    async fn create_post(&self, req: &CreatePostRequest) -> Result<Post, CreatePostError> {
        todo!()
    }

    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .unwrap_or_else(|e| panic!("failed to start SQLite transaction: {}", e));

        let author_id = self.save_author(&mut tx, req.name()).await.map_err(|e| {
            if is_unique_constraint_violation(&e) {
                CreateAuthorError::Duplicate {
                    name: req.name().clone(),
                }
            } else {
                panic!("received unexpected SQLite error: {}", e);
            }
        })?;

        tx.commit()
            .await
            .unwrap_or_else(|e| panic!("failed to commit SQLite transaction: {}", e));

        Ok(Author::new(author_id, req.name()))
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}
