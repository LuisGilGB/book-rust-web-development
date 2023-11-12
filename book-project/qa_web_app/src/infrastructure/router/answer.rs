use std::collections::HashMap;

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use errors::Error;

use crate::domain::answer::{Answer, AnswerId};
use crate::domain::question::QuestionId;
use crate::infrastructure::store::Store;

pub async fn add_answer(
    question_id: i32,
    store: Store,
    params: HashMap<String, String>,
    id: String,
) -> Result<impl Reply, Rejection> {
    let question_id = QuestionId(question_id);
    log::info!("{} - Adding answer...", &id);
    match store.questions.read().await.get(&question_id) {
        Some(question_id) => question_id,
        None => {
            log::warn!("{} - Question not found", &question_id);
            return Err(warp::reject::custom(Error::QuestionNotFound));
        }
    };
    let content = match params.get("content") {
        Some(c) => c,
        None => {
            log::warn!("{} - Missing parameters", &id);
            return Err(warp::reject::custom(Error::MissingParameters));
        }
    };
    let answer = Answer {
        id: AnswerId(store.answers.read().await.len() as i32),
        content: content.to_string(),
        question_id,
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status(
        "Answer added",
        StatusCode::CREATED,
    ))
}
