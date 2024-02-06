// Standard library crates.

// External crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// Project modules
mod hostess;
mod logging;
use logging::setup_native_logging;
mod restful_api;
use restful_api::start_restful_api;

#[tokio::main]
async fn main() {
    let _ = setup_native_logging();

    let _ = start_restful_api();

    info!("Done");
}
