// Standard library crates.
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// External crates.
use anyhow::{anyhow, ensure, Result};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// Project crates.
use crate::datastore::{add_user_reservation, get_schedule, get_user_reservation_schedule};
use crate::CapacitySchedule;
use crate::ReservationRequest;

/// Convenience function for getting the active schedule in one place.
pub fn process_reservation(reservation_request: &ReservationRequest) -> Result<bool> {
    let active_schedule: CapacitySchedule = get_schedule().unwrap();
    // See if we're able to meet the reservation request's requirements.
    let is_reservable = evaluate_reservation_request(&reservation_request, &active_schedule);
    match is_reservable {
        Ok(true) => add_user_reservation(reservation_request)?,
        _ => (),
    };
    is_reservable
}

/// Ensure that reservation begin time is in the future.
///
/// No one has a time machine for using caapacity reseved in the past.
#[allow(unused)]
fn starts_in_future(start_time: u32) -> Result<()> {
    let system_now = SystemTime::now();
    let time_since_epoch: Duration = system_now.duration_since(UNIX_EPOCH).unwrap();
    let epoch_now = time_since_epoch.as_secs();
    ensure!(
        start_time as u64 > epoch_now,
        format!(
            "Reservation request with `start_time` \"{start_time}\" doesn't start in the future."
        )
    );
    Ok(())
}

/// Ensure that start and end times are valid Unix epochs.
///
/// Since there's no maximum or minimum number of seconds before or after Jan 1, 1970,
/// we'll treat everything from Jan 1, 1970 to Jan 1, 2070 as a valid epoch date. Using 100 years
/// (3,153,600,000 seconds) keeps us within a `u32` (max 4,294,967,295 seconds).
///
/// We can assume that no one wants to reserve capacity in the past, so time before the epcoch ("0")
/// is ignored. The unsigned-ness of the integer type will bounce negative numbers at the API, but we
/// check for them here, just to be certain.
fn validate_unix_epoch(suspect_epoch: u32) -> Result<()> {
    // 365 * 24 * 60 * 60 = 31536000 seconds in a year
    // 315360000 * 100 = 3153600000 seconds in 100 years
    // Set validity ceiling to 100 years in future from epoch.
    let epoch_century: u32 = 3153600000;
    // If it's a positive number corresponding to a date before 2870...
    ensure!(
        suspect_epoch > 0 && suspect_epoch < epoch_century,
        format!("Integer \"{suspect_epoch}\" isn't a valid Unix epoch")
    );
    debug!(
        "Validated timestamp in unix epoch format: \"{}\"",
        suspect_epoch
    );
    Ok(())
}

/// Validate a capacity request as being in Arbiter's purview.
///
/// Helper function for `evaluate_reservation_request()` that throws
/// errors when presented with imposssible allocation requests. It needs
/// optimization and maybe some enum variants for making more informative
/// error messages.
fn in_schedule_scope(
    reservation_request: &ReservationRequest,
    capacity_schedule: &CapacitySchedule,
) -> Result<bool> {
    // todo: Optimize scope check iterators.
    let schedule_begin: u32 = capacity_schedule
        .reservations
        .iter()
        .min_by_key(|existing_reservation| existing_reservation.start_time)
        .map(|existing_reservation| existing_reservation.start_time)
        .unwrap();
    debug!("Found capacity schedule's beginning: {}", schedule_begin);
    // todo: Optimize scope check iterators.
    let schedule_end: u32 = capacity_schedule
        .reservations
        .iter()
        .max_by_key(|existing_reservation| existing_reservation.end_time)
        .map(|existing_reservation| existing_reservation.end_time)
        .unwrap();
    debug!("Found capacity schedule's ending: {}", schedule_end);
    let begins_in_scope: bool = reservation_request.start_time >= schedule_begin;
    ensure!(begins_in_scope,
        format!( "Reservation request begins before Arbiter's purview begins on \"{schedule_begin}\": {reservation_request}")
    );
    let ends_in_scope: bool = reservation_request.end_time <= schedule_end;
    ensure!(ends_in_scope,
        format!( "Reservation request ends before Arbiter's purview begins on \"{schedule_begin}\": {reservation_request}")
    );
    let both_in_scope = begins_in_scope && ends_in_scope;

    Ok(both_in_scope)
}

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
    reservation_request: &ReservationRequest,
    capacity_schedule: &CapacitySchedule,
) -> Result<bool> {
    // Ensure the given schedule isn't empty.
    ensure!(
        !capacity_schedule.reservations.is_empty(),
        "Given Capacity Schedule has no reservations"
    );

    // Ensure start and stop times are valid unix epochs.
    validate_unix_epoch(reservation_request.start_time)?;
    validate_unix_epoch(reservation_request.end_time)?;

    // Ensure reservation request begins before it ends.
    ensure!(
        reservation_request.start_time < reservation_request.end_time,
        format!("Invalid reservation request begins before it ends: {reservation_request}")
    );

    // Ensure reservation request starts in the future.
    // temp: disable b/c it interferes with historiccal data.
    //starts_in_future(reservation_request.start_time);

    // Ensure requested period is in scope of capacity schedule.
    let _in_scope: bool = in_schedule_scope(&reservation_request, &capacity_schedule)?;

    debug!("Evaluating {}", reservation_request);
    // Track the total capacity for each timeframe-compatible capacity reservation.
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
    debug!("Competing capacity usages: {:?}", reservation_capacities);

    // Find sum of user capacities so we can check against total capacity ceilings.
    let mut total_user_reservations = 0;

    let user_reservations: CapacitySchedule = get_user_reservation_schedule().unwrap();
    // Find where the request's period overlaps with existing reservations.
    for user_reservation in user_reservations.reservations.iter() {
        let starts_during: bool = reservation_request.start_time < user_reservation.end_time;
        let ends_during: bool = reservation_request.end_time > user_reservation.start_time;
        // If requested timeframe overlaps with a user reservation...
        if starts_during && ends_during {
            debug!("Found overlap with existing {}", user_reservation);
            // todo: ... find how much they overlap so we can suggest right/left alternative timeframes later.
            total_user_reservations += user_reservation.capacity_amount;
        }
    }
    debug!(
        "Sum of competing user reservation usages: {:?}",
        total_user_reservations
    );

    // Find most limiting resource capacity among existing reservations during request timeframe
    let capacity_ceiling: u32 = match reservation_capacities.iter().min() {
        Some(min_found) => *min_found,
        // Throw a runtime error if no limiting factors were found b/c impossible inside schedule bounds
        None => return Err(anyhow!("No applicable reservation capacities were found.")),
    };
    debug!("Limiting factor: {}", capacity_ceiling);

    // Check if lowest available capacity across concurrent reservations can sate request.
    let is_reservable: bool =
        capacity_ceiling >= total_user_reservations + reservation_request.capacity_amount;

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
    #[allow(unused)]
    use log::{debug, error, info, trace, warn};

    // Project crates.
    use super::process_reservation;
    use crate::common::test_examples::test_reservation_alpha;
    use crate::common::ReservationRequest;

    //
    // Edge Cases: Impossible requests that are more than malformed arguments (which would have
    // been caught by the RESTful API)
    //

    #[test]
    // Request with an impossible timeframe that ends before it begins.
    fn test_reject_impossible_timeframe() {
        // First reservation of schedule one with swapped start and end times.
        let impossible_time_reservation = ReservationRequest::new(1708374608, 1707165008, 65, 42);
        let is_reservable = process_reservation(&impossible_time_reservation);
        assert!(is_reservable.is_err());
    }

    #[test]
    // Request with a time period that starts before the capacity schedule's scope.
    fn test_reject_before_schedule_scope() {
        // First reservation of schedule One that starts 42 seconds earlier.
        let too_early_reservation = ReservationRequest::new(1707164966, 1708374608, 64, 42);
        let is_reservable = process_reservation(&too_early_reservation);
        assert!(is_reservable.is_err());
    }

    #[test]
    // Request with a time period that starts after the capacity schedule's scope.
    fn test_reject_after_schedule_scope() {
        // Last reservation of schedule One that ends 42 seconds later.
        let too_late_reservation = ReservationRequest::new(1711398608, 1713213050, 64, 42);
        let is_reservable = process_reservation(&too_late_reservation);
        assert!(is_reservable.is_err());
    }

    //
    // Happy Paths: (In)sufficent capacity for reservation request, within or across schedule time
    // boundaries.
    //

    // Reservation request that fit neatly inside of a "schedule fence" that has capacity.
    //
    // **One of two that fails without DB cleanup between invocations**
    //
    // A laughably easy test that requests exactly what's available in exactly the
    // timeframe where it first becomes available.
    #[test]
    fn test_within_fences_with_capacity() {
        let is_reservable = process_reservation(&test_reservation_alpha()).unwrap();
        assert!(is_reservable);
    }

    // Reservation request that fit neatly inside of a "schedule fence" with insufficient capacity.
    #[test]
    fn test_within_fences_no_capacity() {
        // Exact match for slot, but exceeds total capacity by one.
        let too_big_reservation = ReservationRequest::new(1707165008, 1708374608, 65, 42);
        let is_reservable = process_reservation(&too_big_reservation).unwrap();
        assert!(!is_reservable);
    }

    // Reservation request that crosses "schedule fences" that has capacity.
    //
    // **One of two that fails without DB cleanup between invocations**
    //
    // This test crosses between the second and third reservations of Schedule One, but doesn't
    // exceed available capacity.
    // - `{1708374608, <start(left+42)> 1710793808, 96}`
    // - `{1710793808, <end(right-42)> 1711398608, 32}`
    #[test]
    fn test_outside_fences_with_capacity() {
        // Crosses schedule slots and within capacity.
        let interloper_sufficient_capacity =
            ReservationRequest::new(1708374650, 1711398566, 32, 42);
        let is_reservable = process_reservation(&interloper_sufficient_capacity).unwrap();
        assert!(is_reservable);
    }

    // Reservation request that crosses "schedule fences" with insufficient capacity.
    //
    // This test crosses between the second and third reservations of Schedule One and
    // exceeds available capacity of third reservation by one.
    // - `{1708374608, <start(left+42)> 1710793808, 96}`
    // - `{1710793808, <end(right-42)> 1711398608, 32}`
    #[test]
    fn test_outside_fences_no_capacity() {
        // Crosses schedule slots and within capacity.
        let interloper_insufficient_capacity =
            ReservationRequest::new(1708374650, 1711398566, 33, 42);
        let is_reservable = process_reservation(&interloper_insufficient_capacity).unwrap();
        assert!(!is_reservable);
    }
}
