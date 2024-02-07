// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// Project modules
mod common;
// Make reservation abstractions available everywhere via re-export b/c used often.
pub use common::CapacitySchedule;
pub use common::ReservationRequest;
mod hostess;
use hostess::evaluate_reservation_request;
mod logging;
use logging::setup_native_logging;
mod restful_api;
use restful_api::start_restful_api;

fn main() {
    let _ = setup_native_logging();

    let _ = start_restful_api();

    info!("Done");
}
