// Standard library crates.

// External crates.
use anyhow::{anyhow, ensure, Result};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// Project crates.
use crate::CapacitySchedule;
use crate::ReservationRequest;

/// Decide if a user reservation request can be fulfilled.
///
/// The given timeslot's checked against the capacity schedule to see if there's enough idle
/// capacity's available during that timeframe.
///
/// While there are more efficient algorithms for finding a timeslot, here we prioritize a
/// solution that's easy to modify and reason about. We're not anticipating a ton of requests
/// every second, so performance isn't the first concern. Rather, the most likely question
/// to follow an allocation denial is "why not?" Followed shortly by "then when?"
fn evaluate_reservation_request(
    reservation_request: ReservationRequest,
    capacity_schedule: CapacitySchedule,
) -> Result<bool> {
    // Ensure the given schedule isn't empty.
    ensure!(
        !capacity_schedule.reservations.is_empty(),
        "Given Capacity Schedule has no reservations."
    );
    debug!("Evaluating {}", reservation_request);
    // Track the total capacity for each timeframe-compatible reservation.
    let mut reservation_capacities: Vec<u32> = Vec::new();

    // Find where the request's period overlaps with existing reservations.
    for existing_reservation in capacity_schedule.reservations.iter() {
        let starts_during: bool = reservation_request.start_time < existing_reservation.end_time;
        let ends_during: bool = reservation_request.end_time > existing_reservation.start_time;
        // If requested timeframe overlaps with an existing reservation...
        if starts_during && ends_during {
            debug!("Found overlap with existing {}", existing_reservation);
            // todo: ... find how much they overlap so we can suggest right/left alternative timeframes later.
            reservation_capacities.push(existing_reservation.capacity_amount);
        }
    }
    debug!("Competing reservation usages: {:?}", reservation_capacities);

    // Find lowest total resource capacity among existing reservation's with applicable timeframes.
    let minimum_capacity: &u32 = match reservation_capacities.iter().min() {
        Some(min_found) => min_found,
        None => return Err(anyhow!("No applicable reservation capacities were found.")),
    };
    debug!("Limiting factor: {}", minimum_capacity);

    // Check if lowest available capacity across concurrent reservations can sate request.
    let is_reservable: bool = minimum_capacity >= &reservation_request.capacity_amount;

    let verbal_decree: &str = if is_reservable { "Approved" } else { "Denied" };
    info!(
        "{} request by user ID \"{}\": {}",
        verbal_decree, reservation_request.user_id, reservation_request
    );

    Ok(is_reservable)
}

/// Test if schedules are being assessed correctly.
#[cfg(test)]
mod tests {
    // Standard library crates.
    use log::{debug, error, info, trace, warn};

    // Project crates.
    use super::evaluate_reservation_request;
    use crate::common::test_examples::schedule_one;
    use crate::common::test_examples::test_reservation_alpha;
    use crate::logging::setup_native_logging;

    //
    // Corner Cases: Impossible requests that are more than malformed arguments (which would have
    // been caught by the RESTful API)
    //

    //
    // Happy Paths: (In)sufficent capacity for reservation request, within or across schedule time
    // boundaries.
    //

    // Reservation request that fit neatly inside of a "schedule fence" that has capacity.
    //
    // A laughably easy test that requests exactly what's available in exactly the
    // timeframe where it first becomes available.
    #[test]
    fn test_within_fences_with_capacity() {
        let _ = setup_native_logging();
        // Reservation request that exactly matches available timeframe and capacity.
        let test_reservation = test_reservation_alpha();
        let is_reservable =
            evaluate_reservation_request(test_reservation_alpha(), schedule_one()).unwrap();
        assert!(is_reservable);
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
