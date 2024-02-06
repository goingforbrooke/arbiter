// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.
use warp::Filter;

// Project modules
mod logging;
use logging::setup_native_logging;

fn greeting_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::path!("hello" / String).map(|name: String| format!("Hello, {}!", name))
}

#[tokio::main]
async fn main() {
    let _ = setup_native_logging();

    warp::serve(greeting_route())
        .run(([127, 0, 0, 1], 4242))
        .await;

    info!("Done");
}

#[tokio::test]
async fn test_hello() {
    let route_filter = greeting_route();

    // `user@host: wget -qO- localhost:4242/hello/Eisenhorn` -> Hello, Eisenhorn
    let api_response = warp::test::request()
        .method("GET")
        .path("/hello/Eisenhorn")
        .reply(&route_filter);
    assert_eq!(api_response.await.status(), 200);
}
