// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};
// Serialize JSON payloads.
use serde_derive::{Deserialize, Serialize};

// A capacity reservation requests.
//
// This is used for RESTful JSON parameters, reservation logic, test creation, and datastore
// retrieval. It can represent a request for a portion of a resource or a portion that's already
// been allocated.
#[derive(Deserialize, Serialize)]
pub struct ReservationRequest {
    pub start_time: i64,
    pub end_time: i64,
    pub capacity_amount: u32,
    pub user_id: u32,
}

impl ReservationRequest {
    /// Create a new `ReservationRequest`.
    ///
    ///
    /// # Arguments
    /// - `start_time`: Reservation start time, represented by Unix epoch format.
    /// - `end_time`: Reservation end time, represented by Unix epoch format.
    /// - `capacity_amount`: Amount of resource the user would like to have allocated.
    /// - `user_id`: Your unique identifier.
    pub fn new(start_time: i64, end_time: i64, capacity_amount: u32, user_id: u32) -> Self {
        Self {
            start_time,
            end_time,
            capacity_amount,
            user_id,
        }
    }
}
