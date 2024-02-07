// External crates.
use anyhow::Result;

// Project crates.
use crate::CapacitySchedule;
use crate::ReservationRequest;

pub fn get_schedule() -> Result<CapacitySchedule> {
    let schedule = schedule_one();
    Ok(schedule)
}

// Get Capacity Schedule One from the database.
//fn get_schedule_one() {}
//
//// Get Capacity Schedule Two from the database.
//fn get_schedule_two() {}
//
//fn populate_schedule_tables() {
//    let schedule_one = get_schedule_one();
//    let schedule_two = get_schedule_two();
//}
//
pub fn initialize_database() -> Result<()> {
    //populate_schedule_tables();
    //database_connection
    Ok(())
}

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
            test_reservation_alpha(),
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
///
#[allow(unused)]
pub fn schedule_two() -> CapacitySchedule {
    CapacitySchedule {
        // Assume that `user_id` "88" is on-site maintenance team.
        reservations: vec![
            test_reservation_alpha(),
            ReservationRequest::new(1707769808, 1708979408, 80, 88),
            ReservationRequest::new(1708979408, 1709584208, 40, 88),
            ReservationRequest::new(1709584208, 1712003408, 100, 88),
            ReservationRequest::new(1712003408, 1712608208, 20, 88),
            ReservationRequest::new(1712608208, 1714422608, 60, 88),
        ],
    }
}

#[cfg(test)]
pub mod test_examples {
    // Make mock schedules available to other tests.
    pub use super::schedule_one;
    pub use super::schedule_two;
}
