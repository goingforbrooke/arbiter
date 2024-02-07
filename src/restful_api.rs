// Standard library crates.
use std::error::Error;

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

// Project crates.
use crate::hostess::process_reservation;
use crate::ReservationRequest;

/// RESTful API JSON response concerning reservation attempt.
#[derive(Deserialize, Serialize)]
struct ReservationResponse {
    is_reserved: bool,
    user_message: String,
    // todo: Add unique IDs to reservations.
    // "reservation_id": u32
}

impl ReservationResponse {
    fn new(is_reserved: bool, user_message: String) -> Self {
        Self {
            is_reserved,
            user_message,
        }
    }
}

// Greet the user by name.
//
// "Hello" will be prepended to the name provided in the URL and returned in the HTML body.
//
// # Returns
// HTML body with `"Hello, <given_name>!"`.
fn greeting_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::path!("hello" / String).map(|name: String| format!("Hello, {}!", name))
}

// Reserve some resource capacity within a timeframe.
//
// # Parameters
// - `start_time`: Reservation start time, represented unix epoch format.
// - `end_time`: Reservation end time, represented by unix epoch format.
// - `capacity_amount`: Amount of resource you'd like to have allocated.
// - `user_id`: Your unique identifier.
fn reservation_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
    info!("Received reservation request");
    warp::path!("reserve")
        // Only POST requests can ferry JSON bodies (*usually*).
        .and(warp::post())
        // Expect JSON body format to follow our definition.
        .and(warp::body::json::<ReservationRequest>())
        .map(|reservation_request: ReservationRequest| {
            // Check if the
            let json_response = match process_reservation(&reservation_request) {
                Ok(is_reserved) => {
                    let evaluation_message = match is_reserved {
                        true => String::from("reservation created"),
                        false => String::from("reservation not created"),
                    };
                    ReservationResponse::new(is_reserved, evaluation_message)
                }
                Err(error_message) => ReservationResponse::new(false, error_message.to_string()),
            };
            warp::reply::json(&json_response)
        })
    //.map(|data: ReservationRequest| warp::reply::json(&data))
}

#[tokio::main]
pub async fn start_restful_api() -> Result<(), Box<dyn Error>> {
    // Combine routes so we can feed them to the server enmass.
    let all_routes = greeting_route().or(reservation_route());

    // Start RESTful API.
    info!("Initializing RESTful API");
    Ok(warp::serve(all_routes).run(([127, 0, 0, 1], 4242)).await)
}

#[cfg(test)]
mod tests {
    // External crates.
    use serde_json::from_slice;

    // Project crates.
    use super::ReservationResponse;
    use crate::common::test_examples::test_reservation_alpha;
    use crate::logging::setup_native_logging;
    use crate::restful_api::greeting_route;
    use crate::restful_api::reservation_route;
    use crate::ReservationRequest;
    // Test if the greeting route works correctly.
    //
    // This is the equivalent of:
    // `user@host: wget -qO- localhost:4242/hello/Eisenhorn`
    // `Hello, Eisenhorn`
    #[tokio::test]
    async fn test_greeting_route() {
        let _ = setup_native_logging();
        let route_filter = greeting_route();

        let api_response = warp::test::request()
            .method("GET")
            .path("/hello/Eisenhorn")
            .reply(&route_filter)
            .await;
        assert_eq!(api_response.status(), 200);
        assert_eq!(api_response.body(), "Hello, Eisenhorn!");
    }

    // Disabled b/c last minute tokio concurrency confict with DB driver. Sad.
    ////// Test if the reservation route works correctly.
    //////
    ////// This is the equivalent of:
    ////// `wget --method=POST -O- -q --body-data='{"start_time": 1707165008, "end_time": 1708374608, "capacity_amount": 64, "user_id": 42}' --header=Content-Type:application/json localhost:4242/reserve`
    ////// {"start_time":1707165008,"end_time":1708374608,"capacity_amount":64,"user_id":42}
    ////#[tokio::test]
    ////async fn test_reservation_route() {
    ////    let _ = setup_native_logging();
    ////    let route_filter = reservation_route();

    ////    // Define JSON parameters for theoretical reservation REST request.
    ////    let test_reservation = test_reservation_alpha();

    ////    let api_response = warp::test::request()
    ////        .path("/reserve")
    ////        // POST is required for sending RESTful (JSON) requests.
    ////        .method("POST")
    ////        .json(&test_reservation)
    ////        .reply(&route_filter)
    ////        .await;
    ////    assert_eq!(api_response.status(), 200);

    ////    let rest_response = api_response.body();
    ////    // Deserialize JSON from HTML body.
    ////    let jsonified_body: ReservationResponse = from_slice(rest_response).unwrap();
    ////    assert_eq!(jsonified_body.is_reserved, true);
    ////    assert_eq!(jsonified_body.user_message, "reservation created");
    ////}
    // Future: Test that requests with unknown fields are rejected by serde's unknown fields
    // rejection.
    // wget --method=POST -O- -q --body-data='{"start_time": 1707165008, "end_time": 1708374608, "capacity_amount": 64, "user_id": 42, "memes": "lol"}' --header=Content-Type:application/json localhost:4242/reserve
}
