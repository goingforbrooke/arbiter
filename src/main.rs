// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.
use serde_derive::{Deserialize, Serialize};
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

// Define JSON parameters for reservation REST requests.
#[derive(Deserialize, Serialize)]
struct ReservationRequest {
    start_time: i64,
    end_time: i64,
    capacity_amount: u32,
    user_id: u32,
}

// Reserve some resource capacity within a timeframe.
//
// # Parameters
// - `start_time`: Reservation start time, represented unix epoch format.
// - `end_time`: Reservation end time, represented by unix epoch format.
// - `capacity_amount`: Amount of resource you'd like to have allocated.
// - `user_id`: Your unique identifier.
fn reservation_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
    warp::path!("reserve")
        // Only POST requests can ferry JSON bodies (*usually*).
        .and(warp::post())
        // Expect JSON body format to follow our definition.
        //.and(warp::body::json::<ReservationRequest>())
        .and(warp::body::json())
        .map(|data: ReservationRequest| warp::reply::json(&data))
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
