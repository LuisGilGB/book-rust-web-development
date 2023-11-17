use sqlx::{PgPool, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};

use errors::Error;

use crate::domain::question::{Question, QuestionDraft, QuestionId};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
            Ok(pool) => pool,
            Err(e) => panic!("Can't connect to database: {}", e),
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Question>, Error::DatabaseQueryError> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset.unwrap_or(0))
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await {
            Ok(questions) => Ok(questions),
            Err(e) => {
                log::error!("Error getting questions: {}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn add_question(
        &self,
        question: QuestionDraft,
    ) -> Result<Question, Error> {
        match sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1 $2 $3) RETURNING id, title, content, tags")
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .map(|row| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            }) {
            Ok(question) => Ok(question),
            Err(e) => {
                log::error!("Error adding question: {}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}
