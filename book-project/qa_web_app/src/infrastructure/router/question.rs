use std::collections::HashMap;

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use errors::{Error, InvalidId};

use crate::domain::question::{Question, QuestionId};
use crate::infrastructure::pagination::extract_pagination;
use crate::infrastructure::store::Store;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Querying questions...", &id);
    if !params.is_empty() {
        let pagination = extract_pagination(params, store.questions.read().await.len())?;
        log::info!("{} - Pagination set: {:?}", &id, &pagination);
        let raw_response: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let response = raw_response[pagination.start..pagination.end].to_vec();
        Ok(warp::reply::json(&response))
    } else {
        log::info!("{} - No pagination used", &id);
        let response = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<Question>>();
        Ok(warp::reply::json(&response))
    }
}

pub async fn add_question(
    store: Store,
    question: Question,
    id: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} - Adding question...", &id);
    if store.questions.read().await.contains_key(&question.id) {
        log::warn!("{} - Question already exists", &id);
        return Err(warp::reject::custom(Error::QuestionAlreadyExists));
    }
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question.clone());
    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::CREATED,
    ))
}

pub async fn update_question(
    question_id: String,
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
    question_id: String,
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
