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
            "start_time: {}, end_time: {}, capacity_amount: {}, user_id: {}",
            self.start_time, self.end_time, self.capacity_amount, self.user_id
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
    use super::CapacitySchedule;
    use super::ReservationRequest;

    /// Test reservation request
    ///
    /// This is based off Schedule 1's first reservation.
    pub fn test_reservation_alpha() -> ReservationRequest {
        ReservationRequest::new(1707165008, 1708374608, 64, 42)
    }

    /// - Schedule 1
    ///    - `{1707165008, 1708374608, 64}`
    ///    - `{1708374608, 1710793808, 96}`
    ///    - `{1710793808, 1711398608, 32}`
    ///    - `{1711398608, 1713213008, 128}`
    pub fn schedule_one() -> CapacitySchedule {
        CapacitySchedule {
            // Assume that `user_id` "88" is on-site maintenance team.
            reservations: vec![
                ReservationRequest::new(1707165008, 1708374608, 64, 88),
                ReservationRequest::new(1708374608, 1710793808, 96, 88),
                ReservationRequest::new(1710793808, 1711398608, 32, 88),
                ReservationRequest::new(1711398608, 1713213008, 128, 88),
            ],
        }
    }

    /// - Schedule 2
    ///    - `{1707165008, 1707769808, 50}`
    ///    - `{1707769808, 1708979408, 80}`
    ///    - `{1708979408, 1709584208, 40}`
    ///    - `{1709584208, 1712003408, 100}`
    ///    - `{1712003408, 1712608208, 20}`
    ///    - `{1712608208, 1714422608, 60}`
    pub fn schedule_two() -> CapacitySchedule {
        CapacitySchedule {
            // Assume that `user_id` "88" is on-site maintenance team.
            reservations: vec![
                ReservationRequest::new(1707165008, 1707769808, 50, 88),
                ReservationRequest::new(1707769808, 1708979408, 80, 88),
                ReservationRequest::new(1708979408, 1709584208, 40, 88),
                ReservationRequest::new(1709584208, 1712003408, 100, 88),
                ReservationRequest::new(1712003408, 1712608208, 20, 88),
                ReservationRequest::new(1712608208, 1714422608, 60, 88),
            ],
        }
    }
}
