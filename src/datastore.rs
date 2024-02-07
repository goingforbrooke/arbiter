// External crates.
use anyhow::Result;
use postgres::{Client, NoTls};

// Project crates.
use crate::CapacitySchedule;
use crate::ReservationRequest;
#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub fn initialize_database() -> Result<Client> {
    // Clean old data.
    // SWEEP
    let db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    // Ensure tables exists.
    let _ = create_schedule_tables();
    // Populate tables with dummy data.
    let _ = populate_schedule_tables();
    Ok(db_client)
}

pub fn get_schedule() -> Result<CapacitySchedule> {
    let schedule = schedule_one();
    Ok(schedule)
}

fn create_schedule_tables() -> Result<()> {
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    let table_title = "capacity_schedule";
    let creation_cmd = format!(
        "CREATE TABLE {} (
           id                 SERIAL PRIMARY KEY,
           start_time         INTEGER NOT NULL,
           end_time           INTEGER NOT NULL,
           capacity_amount    INTEGER NOT NULL,
           user_id            INTEGER NOT NULL
        )",
        table_title
    );
    let _ = db_client.batch_execute(&creation_cmd);
    info!("Created DB Table: \"{}\"", table_title);
    Ok(())
}

fn populate_schedule_row(existing_reservation: ReservationRequest, table_name: &str) -> Result<()> {
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    let insertion_command = format!(
        "INSERT INTO {} \
                     (start_time, end_time, capacity_amount, user_id) \
                     VALUES ({}, {}, {}, {})",
        table_name,
        existing_reservation.start_time,
        existing_reservation.end_time,
        existing_reservation.capacity_amount,
        existing_reservation.user_id
    );
    let _ = db_client.batch_execute(&insertion_command);
    Ok(())
}

fn populate_schedule_tables() -> Result<()> {
    for existing_reservation in schedule_one().reservations {
        populate_schedule_row(existing_reservation, "capacity_schedule")?;
    }
    //for existing_reservation in schedule_two()
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
