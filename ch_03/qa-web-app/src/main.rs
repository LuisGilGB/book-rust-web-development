use std::fmt;
use std::str::FromStr;

use serde::Serialize;
use warp::{Filter, http::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct QuestionId(String);

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

impl Reject for InvalidId {}

async fn get_questions() -> Result<impl Reply, Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        }
        Ok(_) => {
            Ok(warp::reply::json(&question))
        }
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    match r.find() {
        Some(InvalidId) => {
            Ok(warp::reply::with_status(
                "No valid id provided",
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
        }
        _ => {
            Ok(warp::reply::with_status(
                "Route not found",
                StatusCode::NOT_FOUND,
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    let hello_handler = warp::path("health").map(|| format!("Alive"));

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items
        .or(hello_handler)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
