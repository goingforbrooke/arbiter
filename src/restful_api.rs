// Standard library crates.
use std::error::Error;

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use warp::Filter;

// Project crates.
use crate::ReservationRequest;

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
    warp::path!("reserve")
        // Only POST requests can ferry JSON bodies (*usually*).
        .and(warp::post())
        // Expect JSON body format to follow our definition.
        //.and(warp::body::json::<ReservationRequest>())
        .and(warp::body::json())
        .map(|data: ReservationRequest| warp::reply::json(&data))
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
    use crate::common::test_examples::test_reservation_alpha;
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
        let route_filter = greeting_route();

        let api_response = warp::test::request()
            .method("GET")
            .path("/hello/Eisenhorn")
            .reply(&route_filter)
            .await;
        assert_eq!(api_response.status(), 200);
        assert_eq!(api_response.body(), "Hello, Eisenhorn!");
    }

    // Test if the reservation route works correctly.
    //
    // This is the equivalent of:
    // `user@host: wget -qO- localhost:4242/hello/Eisenhorn`
    // `wget --method=POST -O- -q --body-data='{"start_time": 1707165008, "end_time": 1708374608, "capacity_amount": 64, "user_id": 42}' --header=Content-Type:application/json localhost:4242/reserve`
    // {"start_time":1707165008,"end_time":1708374608,"capacity_amount":64,"user_id":42}
    #[tokio::test]
    async fn test_reservation_route() {
        let route_filter = reservation_route();

        // Define JSON parameters for theoretical reservation REST request.
        //1707165008, 1708374608, 64, 42
        let test_reservation = test_reservation_alpha();

        let api_response = warp::test::request()
            .path("/reserve")
            // POST is required for sending RESTful (JSON) requests.
            .method("POST")
            // Serialize request body into JSON.
            .json(&test_reservation)
            .reply(&route_filter)
            .await;
        assert_eq!(api_response.status(), 200);

        let rest_response = api_response.body();
        // Deserialize JSON from HTML body.
        let jsonified_body: ReservationRequest = from_slice(rest_response).unwrap();
        assert_eq!(jsonified_body.start_time, 1707165008);
        assert_eq!(jsonified_body.end_time, 1708374608);
        assert_eq!(jsonified_body.capacity_amount, 64);
        assert_eq!(jsonified_body.user_id, 42);
    }
}
