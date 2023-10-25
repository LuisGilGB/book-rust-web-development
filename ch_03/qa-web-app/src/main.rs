use warp::Filter;

#[tokio::main]
async fn main() {
    let hello_handler = warp::path("hello").map(|| format!("Hello, World!"));

    warp::serve(hello_handler).run(([127, 0, 0, 1], 3030)).await;
}
