// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};
// Serialize JSON payloads.
use serde_derive::{Deserialize, Serialize};

// Define capacity reservation requests used for RESTful JSON parameters and reservation logic
// tests.
#[derive(Deserialize, Serialize)]
pub struct ReservationRequest {
    pub start_time: i64,
    pub end_time: i64,
    pub capacity_amount: u32,
    pub user_id: u32,
}
