// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::CapacitySchedule;
use crate::ReservationRequest;

fn evaluate_reservation_request() {
    info!("wow");
}

/// Test if schedules are being assessed correctly.
#[cfg(test)]
mod tests {
    // Reservation request that fit neatly inside of a "schedule fence" that has capacity.
    #[test]
    fn within_fences_with_capacity() {
        // Theoretical reservation request.
        let test_reservation = ReservationRequest::new(1707165008, 1708374608, 64, 42);
    }

    // Reservation request that fit neatly inside of a "schedule fence" with insufficient capacity.
    #[test]
    fn within_fences_no_capacity() {}

    // Reservation request that crosses "schedule fences" that has capacity.
    #[test]
    fn outside_fences_with_capacity() {}

    // Reservation request that crosses "schedule fences" with insufficient capacity.
    #[test]
    fn outside_fences_no_capacity() {}
}
