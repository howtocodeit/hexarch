use std::str::FromStr;

use anyhow::{anyhow, Context};
use sqlx::{Executor, SqlitePool, Transaction};
use sqlx::sqlite::SqliteConnectOptions;
use uuid::Uuid;

use crate::domain::blog::models::author::{Author, AuthorName, CreateAuthorRequest};
use crate::domain::blog::models::author::CreateAuthorError;
use crate::domain::blog::ports::BlogRepository;

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
        name: &AuthorName,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let name = &name.to_string();
        let query = sqlx::query!(
            "INSERT INTO authors (id, name) VALUES ($1, $2)",
            id_as_string,
            name,
        );
        tx.execute(query).await?;
        Ok(id)
    }
}

impl BlogRepository for Sqlite {
    async fn create_author(&self, req: &CreateAuthorRequest) -> Result<Author, CreateAuthorError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

        let author_id = self.save_author(&mut tx, req.name()).await.map_err(|e| {
            if is_unique_constraint_violation(&e) {
                CreateAuthorError::Duplicate {
                    name: req.name().clone(),
                }
            } else {
                anyhow!(e)
                    .context(format!("failed to save blog with name {:?}", req.name()))
                    .into()
            }
        })?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(Author::new(
            author_id,
            req.name().clone(),
            req.email().clone(),
        ))
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
