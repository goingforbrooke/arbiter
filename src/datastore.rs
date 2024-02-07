// External crates.
use anyhow::Result;
use postgres::{Client, NoTls};

// Project crates.
use crate::CapacitySchedule;
use crate::ReservationRequest;
#[allow(unused)]
use log::{debug, error, info, trace, warn};

/// Initialize Arbiter's database.
///
/// Warning: If PostgreSQL was in stalled with Homebrew, then the "postgres" role needs to be added
/// before this will work. Without it, `Client::connect()` will hang forever.
/// `user@host: /opt/homebrew/opt/postgresql@14/bin/createuser -s postgres`
/// credit: https://stackoverflow.com/questions/15301826/psql-fatal-role-postgres-does-not-exist#comment91332745_15309551
pub fn initialize_database() -> Result<Client> {
    info!("Initializing database");
    let _ = cleanup_database();
    // todo: Clean up DB on init.
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    // Ensure tables exist.
    let _ = create_schedule_tables(&mut db_client);
    // Populate tables with dummy data.
    let _ = populate_schedule_tables(&mut db_client);
    info!("Initialized database");
    Ok(db_client)
}

/// Delete all known database tables.
fn cleanup_database() -> Result<()> {
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    db_client.execute("DROP TABLE capacity_schedule, user_reservations;", &[])?;
    info!("Deleted DB tables: capacity_schedule, user_reservations");
    Ok(())
}

///// Get mocked capacity schedule from testing function.
//pub fn get_schedule() -> Result<CapacitySchedule> {
//    let schedule = schedule_one();
//    Ok(schedule)
//}

/// Get capacity schedule from Database.
pub fn get_schedule() -> Result<CapacitySchedule> {
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    let mut capacities = Vec::new();
    for query_row in db_client.query(
        "SELECT id, start_time, end_time, capacity_amount, user_id FROM capacity_schedule",
        &[],
    )? {
        // todo: Disregard id.
        let _id: i32 = query_row.get(0);
        let start_time: u32 = query_row.get(1);
        let end_time: u32 = query_row.get(2);
        let capacity_amount: u32 = query_row.get(3);
        let user_id: u32 = query_row.get(4);
        let existing_reservation =
            ReservationRequest::new(start_time, end_time, capacity_amount, user_id);
        capacities.push(existing_reservation)
    }
    let queried_schedule = CapacitySchedule {
        reservations: capacities,
    };
    Ok(queried_schedule)
}

/// Add reservation to user reservation table.
///
/// Assume that the reservation's timeframe and capacity have already been validated.
pub fn add_user_reservation(new_reservation: &ReservationRequest) -> Result<()> {
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    db_client.execute(
        "INSERT INTO user_reservations 
                      (start_time, end_time, reservation_amount, user_id) 
                      VALUES ($1, $2, $3, $4)",
        &[
            &new_reservation.start_time,
            &new_reservation.end_time,
            &new_reservation.capacity_amount,
            &new_reservation.user_id,
        ],
    )?;
    info!("Added reservation to DB");
    Ok(())
}

/// Get user reservation schedule from Database.
pub fn get_user_reservation_schedule() -> Result<CapacitySchedule> {
    let mut db_client = Client::connect("host=localhost user=postgres", NoTls)?;
    let mut capacities = Vec::new();
    for query_row in db_client.query(
        "SELECT id, start_time, end_time, reservation_amount, user_id FROM user_reservations",
        &[],
    )? {
        // todo: Disregard id.
        let _id: i32 = query_row.get(0);
        let start_time: u32 = query_row.get(1);
        let end_time: u32 = query_row.get(2);
        let reservation_amount: u32 = query_row.get(3);
        let user_id: u32 = query_row.get(4);
        let user_reservations =
            ReservationRequest::new(start_time, end_time, reservation_amount, user_id);
        capacities.push(user_reservations)
    }
    let queried_schedule = CapacitySchedule {
        reservations: capacities,
    };
    Ok(queried_schedule)
}

fn create_schedule_tables(db_client: &mut Client) -> Result<()> {
    let _ = db_client.execute(
        "CREATE TABLE capacity_schedule (
                                 id                 SERIAL PRIMARY KEY,
                                 start_time         INTEGER NOT NULL,
                                 end_time           INTEGER NOT NULL,
                                 capacity_amount    INTEGER NOT NULL,
                                 user_id            INTEGER NOT NULL
                                 )",
        &[],
    );
    debug!("Created capacity schedule table");
    let _ = db_client.execute(
        "CREATE TABLE user_reservations (
                                 id                 SERIAL PRIMARY KEY,
                                 start_time         INTEGER NOT NULL,
                                 end_time           INTEGER NOT NULL,
                                 reservation_amount INTEGER NOT NULL,
                                 user_id            INTEGER NOT NULL
                                 )",
        &[],
    );
    debug!("Created user reservation table");
    info!("Created DB Tables");
    Ok(())
}

fn populate_schedule_row(
    existing_reservation: ReservationRequest,
    table_name: &str,
    db_client: &mut Client,
) -> Result<()> {
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

fn populate_schedule_tables(db_client: &mut Client) -> Result<()> {
    for existing_reservation in schedule_one().reservations {
        populate_schedule_row(existing_reservation, "capacity_schedule", db_client)?;
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
