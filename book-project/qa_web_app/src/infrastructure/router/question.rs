use std::collections::HashMap;

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use errors::{Error, InvalidId};

use crate::domain::question::{Question, QuestionDraft, QuestionId};
use crate::infrastructure::pagination::{extract_pagination, Pagination};
use crate::infrastructure::response::{BadWordsResponse, create_api_layer_error};
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
    log::info!("{} - Checking bad worths...", &id);
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "sjPxmHoxo8lD2DkKFSGDjCPWd9nMykXE")
        .body(String::from(&question_draft.content))
        .send()
        .await
        .map_err(|e| {
            log::error!("{} - Error checking bad words: {}", &id, e);
            Error::ExternalAPIError(e)
        })?;

    if !response.status().is_success() {
        if response.status().is_client_error() {
            log::warn!("{} - Bad request", &id);
            let err = create_api_layer_error(response).await;
            return Err(warp::reject::custom(Error::ClientError(err)));
        } else {
            log::error!("{} - Server error", &id);
            let err = create_api_layer_error(response).await;
            return Err(warp::reject::custom(Error::ServerError(err)));
        }
    }

    let response = response.json::<BadWordsResponse>()
        .await
        .map_err(|e| {
            log::error!("{} - Error checking bad words: {}", &id, e);
            Error::ExternalAPIError(e)
        })?;

    let content = response.censored_content;

    let question = QuestionDraft {
        content,
        ..question_draft
    };

    match store.add_question(question).await {
        Ok(question) => {
            log::info!("{} - Adding question...", &id);
            Ok(warp::reply::with_status(
                warp::reply::json(&question),
                StatusCode::CREATED,
            ))
        }
        Err(e) => {
            log::error!("{} - Error adding question: {}", &id, e);
            Err(warp::reject::custom(e))
        }
    }
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
        .update_question(question)
        .await {
        Ok(_) => Ok(warp::reply::with_status(
            "Question updated",
            StatusCode::ACCEPTED,
        )),
        Err(e) => {
            log::error!("{} - Error updating question: {}", &id, e);
            Err(warp::reject::custom(e))
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
        .delete_question(QuestionId(question_id))
        .await {
        Ok(_) => Ok(warp::reply::with_status(
            "Question deleted",
            StatusCode::NO_CONTENT,
        )),
        Err(e) => {
            log::error!("{} - Error deleting question: {}", &id, e);
            Err(warp::reject::custom(e))
        }
    }
}
