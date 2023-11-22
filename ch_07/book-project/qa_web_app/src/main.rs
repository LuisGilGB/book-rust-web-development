#![warn(clippy::all)]

use std::path::Path;

use sqlx::migrate::Migrator;
use warp::{Filter, http::Method};

use errors::return_error;
use infrastructure::store::Store;

use crate::infrastructure::router::answer::add_answer;
use crate::infrastructure::router::question::{
    add_question, delete_question, get_questions, update_question,
};

mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::info!("Starting server...");

    let log = warp::log::custom(|info| {
        log::info!("{} {} {} - {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        )
    });

    let store = Store::new("postgres://localhost:5432/rustwebdev")
        .await;

    let migrator = Migrator::new(Path::new("../migrations"))
        .await.unwrap();

    migrator.run(&store.clone().connection).await.unwrap();

    let store_filter = warp::any().map(move || store.clone());

    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ]);

    let health = warp::path("health").map(|| "Alive".to_string());

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and(id_filter.clone())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and(id_filter.clone())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(id_filter.clone())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and(id_filter.clone())
        .and_then(add_answer);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .or(health)
        .with(cors)
        .with(log)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
