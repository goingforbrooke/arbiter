//! Common structs for Arbiter.

// Standard library crates.
use std::fmt;

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};
// Serialize JSON payloads.
use serde_derive::{Deserialize, Serialize};

/// A capacity reservation request.
///
/// This is used for RESTful JSON parameters, reservation logic, test creation, and datastore
/// retrieval. It can represent a request for a portion of a resource or a portion that's already
/// been allocated.
#[derive(Deserialize, Serialize)]
// Reject unknown REST JSON params with descriptive message.
#[serde(deny_unknown_fields)]
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

// Print instantiated struct nicely.
impl fmt::Display for ReservationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "reservation request from user ID \"{}\" \
            for \"{}\" of capacity \
            from \"{}\" to \"{}\"",
            self.user_id, self.capacity_amount, self.start_time, self.end_time
        )
    }
}

/// A capacity schedule for a resource.
///
/// This schedule will never change or fail.
pub struct CapacitySchedule {
    pub reservations: Vec<ReservationRequest>,
}

/// Example instances of common scheduling structs that are available to all tests.
#[cfg(test)]
pub mod test_examples {
    use super::ReservationRequest;

    /// Test reservation request
    ///
    /// This is based off Schedule 1's first reservation.
    pub fn test_reservation_alpha() -> ReservationRequest {
        ReservationRequest::new(1707165008, 1708374608, 64, 42)
    }
}
