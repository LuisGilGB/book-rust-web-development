use std::collections::HashMap;

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use errors::{Error, InvalidId};

use crate::domain::question::{Question, QuestionDraft, QuestionId};
use crate::infrastructure::pagination::{extract_pagination, Pagination};
use crate::infrastructure::store::Store;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Querying questions...", &id);
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        log::debug!("{} - Pagination used", &id);
        pagination = extract_pagination(params)?;
    }
    let response = match store.get_questions(pagination.limit, pagination.offset).await {
        Ok(questions) => questions,
        Err(e) => {
            log::error!("{} - Error getting questions: {}", &id, e);
            return Err(warp::reject::custom(e));
        }
    };
    Ok(warp::reply::json(&response))
}

pub async fn add_question(
    store: Store,
    question_draft: QuestionDraft,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Adding question...", &id);
    store.add_question(question_draft).await.map_err(|e| {
        log::error!("{} - Error adding question: {}", &id, e);
        warp::reject::custom(e)
    })?;
    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::CREATED,
    ))
}

pub async fn update_question(
    question_id: i32,
    store: Store,
    question: Question,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Updating question...", &id);
    if question_id != question.id.0 {
        log::warn!("{} - Invalid question id", &id);
        return Err(warp::reject::custom(Error::InvalidId(InvalidId)));
    }
    match store
        .questions
        .write()
        .await
        .get_mut(&QuestionId(question_id))
    {
        Some(q) => {
            *q = question;
            Ok(warp::reply::with_status(
                "Question updated",
                StatusCode::ACCEPTED,
            ))
        }
        None => {
            log::warn!("{} - Question not found", &id);
            Err(warp::reject::custom(Error::QuestionNotFound))
        }
    }
}

pub async fn delete_question(
    question_id: i32,
    store: Store,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Deleting question...", &id);
    match store
        .questions
        .write()
        .await
        .remove(&QuestionId(question_id))
    {
        Some(_) => Ok(warp::reply::with_status(
            "Question deleted",
            StatusCode::NO_CONTENT,
        )),
        None => {
            log::warn!("{} - Question not found", &id);
            Err(warp::reject::custom(Error::QuestionNotFound))
        }
    }
}
