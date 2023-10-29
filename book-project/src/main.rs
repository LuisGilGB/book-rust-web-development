use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
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
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct InvalidId;

#[derive(Debug)]
enum Error {
    CORSForbidden(CorsForbidden),
    ParseError(std::num::ParseIntError),
    InvalidId(InvalidId),
    MissingParameters,
    StartGreaterThanEnd,
    QuestionNotFound,
    QuestionAlreadyExists,
}

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
            questions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> Self {
        let file = include_str!("../questions.json");
        let parsed_hash_map: HashMap<QuestionId, Question> = serde_json::from_str(file).expect("Can't parse questions.json file");

        Store {
            questions: Arc::new(RwLock::new(parsed_hash_map)),
        }
    }
}

impl fmt::Display for InvalidId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid id")
    }
}

impl Reject for InvalidId {}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CORSForbidden(error) => write!(formatter, "CORS error: {}", error),
            Error::ParseError(error) => write!(formatter, "Parse error: {}", error),
            Error::InvalidId(error) => write!(formatter, "Invalid id: {}", error),
            Error::MissingParameters => write!(formatter, "Missing parameters"),
            Error::StartGreaterThanEnd => write!(formatter, "Start cannot be greater than end"),
            Error::QuestionNotFound => write!(formatter, "Question not found"),
            Error::QuestionAlreadyExists => write!(formatter, "Question already exists"),
        }
    }
}

impl Reject for Error {}

fn cap_number(max: usize) -> impl Fn(usize) -> usize {
    move |x| {
        if x > max {
            max
        } else {
            x
        }
    }
}

fn extract_pagination(params: HashMap<String, String>, total_length: usize) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        if start > end {
            return Err(Error::StartGreaterThanEnd);
        }
        return Ok(Pagination {
            start: cap_number(total_length)(start),
            end: cap_number(total_length)(end),
        });
    }
    Err(Error::MissingParameters)
}

async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl Reply, Rejection> {
    println!("{:?}", params);
    if !params.is_empty() {
        let pagination = extract_pagination(params, store.questions.read().await.len())?;
        let raw_response: Vec<Question> = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect();
        let response = raw_response[pagination.start..pagination.end].to_vec();
        Ok(warp::reply::json(&response))
    } else {
        let response = store.questions
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<Question>>();
        Ok(warp::reply::json(&response))
    }
}

async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store.questions.write().await.insert(question.id.clone(), question.clone());
    Ok(warp::reply::with_status("Question added", StatusCode::CREATED))
}

async fn update_question(question_id: String, store: Store, question: Question) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(question_id)) {
        Some(q) => {
            *q = question;
            Ok(warp::reply::with_status("Question updated", StatusCode::OK))
        }
        None => Err(warp::reject::custom(Error::QuestionNotFound))
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    match r.find::<Error>() {
        Some(Error::CORSForbidden(error)) => {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::FORBIDDEN,
            ))
        }
        Some(Error::InvalidId(_error)) => {
            Ok(warp::reply::with_status(
                "No valid id provided".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
        }
        Some(Error::MissingParameters) => {
            Ok(warp::reply::with_status(
                "Missing parameters".to_string(),
                StatusCode::BAD_REQUEST,
            ))
        }
        Some(Error::StartGreaterThanEnd) => {
            Ok(warp::reply::with_status(
                "Start cannot be greater than end".to_string(),
                StatusCode::BAD_REQUEST,
            ))
        }
        Some(Error::ParseError(_error)) => {
            Ok(warp::reply::with_status(
                "Parse error".to_string(),
                StatusCode::BAD_REQUEST,
            ))
        }
        Some(Error::QuestionNotFound) => {
            Ok(warp::reply::with_status(
                "Question not found".to_string(),
                StatusCode::NOT_FOUND,
            ))
        }
        Some(Error::QuestionAlreadyExists) => {
            Ok(warp::reply::with_status(
                "Question already exists".to_string(),
                StatusCode::CONFLICT,
            ))
        }
        err => {
            println!("Unhandled rejection: {:?}", r);
            println!("Unhandled error: {:?}", err);
            Ok(warp::reply::with_status(
                "Route not found".to_string(),
                StatusCode::NOT_FOUND,
            ))
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

    let health = warp::path("health").map(|| format!("Alive"));

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(health)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
