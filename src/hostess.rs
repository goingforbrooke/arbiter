// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// temp: Pull in existing rez request struct, which will be centralized later.
use crate::restful_api::ReservationRequest;

fn thing() {
    info!("wow")
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
    // temp: Pull in existing rez request struct, which will be centralized later.
    use crate::restful_api::ReservationRequest;
    // Theoretical reservation request.
    const TEST_RESERVATION: ReservationRequest = ReservationRequest {
        start_time: 1707165008,
        end_time: 1708374608,
        capacity_amount: 64,
        user_id: 42,
    };

    // Reservation request that fit neatly inside of a "schedule fence" that has capacity.
    #[test]
    fn within_fences_with_capacity() {}

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
