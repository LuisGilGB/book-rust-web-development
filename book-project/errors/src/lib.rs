use std::fmt;
use std::fmt::{Display, Formatter};

use reqwest::Error as ReqwestError;
use warp::{Rejection, Reply};
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::http::StatusCode;
use warp::reject::Reject;

#[derive(Debug)]
pub struct InvalidId;

#[derive(Debug)]
pub enum Error {
    CORSForbidden(CorsForbidden),
    BodyDeserializeError(BodyDeserializeError),
    ParseError(std::num::ParseIntError),
    InvalidId(InvalidId),
    MissingParameters,
    StartGreaterThanEnd,
    QuestionNotFound,
    QuestionAlreadyExists,
    DatabaseQueryError,
    ExternalAPIError(ReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError),
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl Display for APILayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
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
            Error::BodyDeserializeError(error) => {
                write!(formatter, "Body deserialize error: {}", error)
            }
            Error::ParseError(error) => write!(formatter, "Parse error: {}", error),
            Error::InvalidId(error) => write!(formatter, "Invalid id: {}", error),
            Error::MissingParameters => write!(formatter, "Missing parameters"),
            Error::StartGreaterThanEnd => write!(formatter, "Start cannot be greater than end"),
            Error::QuestionNotFound => write!(formatter, "Question not found"),
            Error::QuestionAlreadyExists => write!(formatter, "Question already exists"),
            Error::DatabaseQueryError => write!(formatter, "Query could not be executed"),
            Error::ExternalAPIError(error) => write!(formatter, "External API error: {}", error),
            Error::ClientError(error) => write!(formatter, "External Client error: {}", error),
            Error::ServerError(error) => write!(formatter, "External Server error: {}", error),
        }
    }
}

impl Reject for Error {}

impl Reject for APILayerError {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    match r.find::<Error>() {
        Some(Error::CORSForbidden(error)) => Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        )),
        Some(Error::BodyDeserializeError(_error)) => Ok(warp::reply::with_status(
            "Body deserialize error".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        )),
        Some(Error::InvalidId(_error)) => Ok(warp::reply::with_status(
            "No valid id provided".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        )),
        Some(Error::MissingParameters) => Ok(warp::reply::with_status(
            "Missing parameters".to_string(),
            StatusCode::BAD_REQUEST,
        )),
        Some(Error::StartGreaterThanEnd) => Ok(warp::reply::with_status(
            "Start cannot be greater than end".to_string(),
            StatusCode::BAD_REQUEST,
        )),
        Some(Error::ParseError(_error)) => Ok(warp::reply::with_status(
            "Parse error".to_string(),
            StatusCode::BAD_REQUEST,
        )),
        Some(Error::QuestionNotFound) => Ok(warp::reply::with_status(
            "Question not found".to_string(),
            StatusCode::NOT_FOUND,
        )),
        Some(Error::QuestionAlreadyExists) => Ok(warp::reply::with_status(
            "Question already exists".to_string(),
            StatusCode::CONFLICT,
        )),
        Some(Error::DatabaseQueryError) => Ok(warp::reply::with_status(
            "Query could not be executed".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Some(Error::ExternalAPIError(_error)) => Ok(warp::reply::with_status(
            "External API error".to_string(),
            StatusCode::BAD_GATEWAY,
        )),
        Some(Error::ClientError(_error)) => Ok(warp::reply::with_status(
            "External Client error".to_string(),
            StatusCode::BAD_GATEWAY,
        )),
        Some(Error::ServerError(_error)) => Ok(warp::reply::with_status(
            "External Server error".to_string(),
            StatusCode::BAD_GATEWAY,
        )),
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
