// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::CapacitySchedule;
use crate::ReservationRequest;

fn evaluate_reservation_request() {
    info!("wow");
}

// Test if schedules are being assessed correctly.
//
// - Schedule 1
//    - `{1707165008, 1708374608, 64}`
//    - `{1708374608, 1710793808, 96}`
//    - `{1710793808, 1711398608, 32}`
//    - `{1711398608, 1713213008, 128}`
// - Schedule 2
//    - `{1707165008, 1707769808, 50}`
//    - `{1707769808, 1708979408, 80}`
//    - `{1708979408, 1709584208, 40}`
//    - `{1709584208, 1712003408, 100}`
//    - `{1712003408, 1712608208, 20}`
//    - `{1712608208, 1714422608, 60}`
#[cfg(test)]
mod tests {
    use crate::ReservationRequest;

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
