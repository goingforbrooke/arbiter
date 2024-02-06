// Standard library crates.
use std::error::Error;

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::CapacitySchedule;
use crate::ReservationRequest;

/// Decide if a user reservation request can be fulfilled.
///
/// The given timeslot's checked against the capacity schedule to see if there's enough idle
/// capacity's available during that timeframe.
fn evaluate_reservation_request(
    reservation_request: ReservationRequest,
    capacity_schedule: CapacitySchedule,
) -> Result<(), Box<dyn Error>> {
    capacity_schedule
        .reservations
        .iter()
        .for_each(|reservation| {
            info!("Reservation: {}", reservation);
        });
    Ok(())
}

/// Test if schedules are being assessed correctly.
#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};
    // Project crates.
    use super::evaluate_reservation_request;
    use crate::common::test_examples::schedule_one;
    use crate::common::test_examples::test_reservation_alpha;

    //
    // Corner Cases: Impossible requests that are more than malformed arguments (which would have
    // been caught by the RESTful API)
    //

    //
    // Happy Paths: (In)sufficent capacity for reservation request, within or across schedule time
    // boundaries.
    //

    // Reservation request that fit neatly inside of a "schedule fence" that has capacity.
    #[test]
    fn test_within_fences_with_capacity() {
        // Reservation request that exactly matches available timeframe and capacity.
        let test_reservation = test_reservation_alpha();
        let is_reservable = evaluate_reservation_request(test_reservation_alpha(), schedule_one());
    }

    // Reservation request that fit neatly inside of a "schedule fence" with insufficient capacity.
    #[test]
    fn test_within_fences_no_capacity() {}

    // Reservation request that crosses "schedule fences" that has capacity.
    #[test]
    fn test_outside_fences_with_capacity() {}

    // Reservation request that crosses "schedule fences" with insufficient capacity.
    #[test]
    fn test_outside_fences_no_capacity() {}
}
