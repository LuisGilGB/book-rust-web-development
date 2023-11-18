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
    ) -> Result<Vec<Question>, Error> {
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

    pub async fn update_question(
        &self,
        question: Question,
    ) -> Result<Question, Error> {
        match sqlx::query("UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING *")
            .bind(&question.title)
            .bind(&question.content)
            .bind(&question.tags)
            .bind(question.id.0)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await {
            Ok(question) => Ok(question),
            Err(e) => {
                log::error!("Error updating question: {}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn delete_question(&self, id: QuestionId) -> Result<bool, Error> {
        let result = sqlx::query("DELETE FROM questions WHERE id = ?")
            .bind(id)
            .execute(&self.connection)
            .await;

        let rows_affected = match result {
            Ok(r) => r.rows_affected(),
            Err(e) => {
                log::error!("Error deleting question: {}", e);
                return Err(Error::DatabaseQueryError);
            }
        };

        if rows_affected == 0 {
            return Err(Error::QuestionNotFound);
        }

        Ok(true)
    }
}
