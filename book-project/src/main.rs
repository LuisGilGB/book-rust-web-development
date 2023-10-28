use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use warp::{Filter, filters::cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
struct QuestionId(String);

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

#[derive(Debug)]
struct InvalidId;

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl fmt::Display for Question {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Question: {}, {}, {}, {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl fmt::Display for QuestionId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No id provided",
            )),
        }
    }
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    fn init() -> Self {
        let file = include_str!("../questions.json");
        let parsed_hash_map: HashMap<QuestionId, Question> = serde_json::from_str(file).expect("Can't parse questions.json file");

        Store {
            questions: parsed_hash_map,
        }
    }

    fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}

impl Reject for InvalidId {}

async fn get_questions(store: Store) -> Result<impl Reply, Rejection> {
    let response: Vec<Question> = store.questions.values().cloned().collect();
    if response.iter().any(|question| question.id.0.parse::<i32>().is_err()) {
        return Err(warp::reject::custom(InvalidId));
    }

    Ok(warp::reply::json(&response))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    match r.find::<CorsForbidden>() {
        Some(error) => {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::FORBIDDEN,
            ))
        }
        None => match r.find() {
            Some(InvalidId) => {
                Ok(warp::reply::with_status(
                    "No valid id provided".to_string(),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ))
            }
            _ => {
                Ok(warp::reply::with_status(
                    "Route not found".to_string(),
                    StatusCode::NOT_FOUND,
                ))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let store = Store::init();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE]);

    let hello_handler = warp::path("health").map(|| format!("Alive"));

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions);

    let routes = get_items
        .or(hello_handler)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
