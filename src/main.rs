// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.
use warp::Filter;

// Project modules
mod logging;
use logging::setup_native_logging;

// Greet the user by name.
//
// "Hello" will be prepended to the name provided in the URL and returned in the HTML body.
//
// # Returns
// HTML body with `"Hello, <given_name>!"`.
fn greeting_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::path!("hello" / String).map(|name: String| format!("Hello, {}!", name))
}

#[tokio::main]
async fn main() {
    let _ = setup_native_logging();

    // Start RESTful API.
    warp::serve(greeting_route())
        .run(([127, 0, 0, 1], 4242))
        .await;

    info!("Done");
}

// Test if the greeting route works correctly.
//
// This is the equivalent of:
// `user@host: wget -qO- localhost:4242/hello/Eisenhorn`
// `Hello, Eisenhorn`
#[tokio::test]
async fn test_greeting_route() {
    let route_filter = greeting_route();

    let api_response = warp::test::request()
        .method("GET")
        .path("/hello/Eisenhorn")
        .reply(&route_filter)
        .await;
    assert_eq!(api_response.body(), "Hello, Eisenhorn!");
    assert_eq!(api_response.status(), 200);
}
