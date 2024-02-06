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
        capacity_schedule.reservations.is_empty(),
        "Given Capacity Schedule has no reservations."
    );
    debug!("Evaluating reservation request: {}", reservation_request);

    // Track the total capacity for each timeframe-compatible reservation.
    let mut reservation_capacities: Vec<u32> = Vec::new();

    // todo: Evaluate existing reservations in reverse (newest-to-oldest) so there's fewer to go through.
    for existing_reservation in capacity_schedule.reservations.iter() {
        debug!(
            "Evaluating against existing reservation: {}",
            existing_reservation
        );
        // If this reservation starts within the first reservation's timeframe...
        if existing_reservation.start_time == reservation_request.start_time {
            debug!(
                "Found that existing reservation's timeframe {} applies to reservation request's timeframe {}",
                existing_reservation, reservation_request
            );
            // ... then note its total capacity as a limiting factor.
            reservation_capacities.push(existing_reservation.capacity_amount);
        }
    }

    // Find lowest total resource capacity among existing reservation's with applicable timeframes.
    let minimum_capacity: &u32 = match reservation_capacities.iter().min() {
        Some(min_found) => min_found,
        None => return Err(anyhow!("No applicable reservation capacities were found.")),
    };

    // Check if lowest available capacity across existing reservations can sate request.
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
    //
    // A laughably easy test that requests exactly what's available in exactly the
    // timeframe where it first becomes available.
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
