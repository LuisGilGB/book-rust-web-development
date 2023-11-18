use std::collections::HashMap;

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use crate::domain::answer::AnswerDraft;
use crate::domain::question::QuestionId;
use crate::infrastructure::store::Store;

pub async fn add_answer(
    question_id: i32,
    store: Store,
    param: HashMap<String, String>,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Adding answer...", &id);
    let answer_draft = AnswerDraft {
        content: param.get("content").unwrap().to_string(),
        question_id: QuestionId(question_id),
    };
    match store.add_answer(answer_draft).await {
        Ok(answer) => answer,
        Err(e) => {
            log::error!("{} - Error adding answer: {}", &id, e);
            return Err(warp::reject::custom(e));
        }
    };
    Ok(warp::reply::with_status(
        "Answer added",
        StatusCode::CREATED,
    ))
}
