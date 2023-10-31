use std::collections::HashMap;

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use crate::domain::answer::{Answer, AnswerId};
use crate::domain::question::QuestionId;
use crate::error::{Error, InvalidId};
use crate::infrastructure::store::Store;

pub async fn add_answer(question_id: String, store: Store, params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let question_id: QuestionId = match question_id.parse() {
        Ok(id) => match store.questions.read().await.get(&id) {
            Some(_) => id,
            None => return Err(warp::reject::custom(Error::QuestionNotFound))
        }
        Err(_) => return Err(warp::reject::custom(Error::InvalidId(InvalidId)))
    };
    let content = match params.get("content") {
        Some(c) => c,
        None => return Err(warp::reject::custom(Error::MissingParameters))
    };
    let answer = Answer {
        id: AnswerId(store.answers.read().await.len().to_string()),
        content: content.to_string(),
        question_id,
    };
    store.answers.write().await.insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status("Answer added", StatusCode::CREATED))
}
