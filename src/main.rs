// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.
use warp::Filter;

// Project modules
mod logging;
use logging::setup_native_logging;

#[tokio::main]
async fn main() {
    let _ = setup_native_logging();

    // `user@host: wget -qO- localhost:4242/hello/Eisenhorn` -> Hello, Eisenhorn
    let hello_route = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

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
