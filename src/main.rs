// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.
use warp::{http::Response, Filter};

// Project modules
mod logging;
use logging::setup_native_logging;

// Extract the first parameter (a name) from the path and prepend a greeting to it.
fn create_greeting() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::path::param().map(|name: String| format!("Hello, {}!", name))
}

#[tokio::main]
async fn main() {
    let _ = setup_native_logging();

    // `user@host: wget -qO- localhost:4242/hello/Eisenhorn` -> Hello, Eisenhorn
    let hello_route = warp::path!("hello")
        .and(create_greeting())
        // Add created greeting to HTML response body.
        .map(|greeting: String| Response::builder().body(greeting));

    warp::serve(hello_route).run(([127, 0, 0, 1], 4242)).await;

    info!("Done");
}

#[tokio::test]
// Test the Warp filter that constructs the greeting
async fn test_hello() {
    let test_filter = create_greeting();

    let filter_response = warp::test::request()
        .path("/Eisenhorn")
        .filter(&test_filter)
        .await
        .unwrap();
    assert_eq!(filter_response, "Hello, Eisenhorn!");
}
