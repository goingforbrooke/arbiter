// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.

// Project modules
mod logging;
use logging::setup_native_logging;

fn main() {
    let _ = setup_native_logging();
    info!("Done");
}
