// Standard library crates.
#[allow(unused)]
use log::{debug, error, info, trace, warn};

// External crates.
use warp::Filter;

// Project modules
mod logging;
use logging::setup_native_logging;

#[tokio::main]
async fn main() {
    let _ = setup_native_logging();

    // `user@host: wget -qO- localhost:4242/reserve/Eisenhorn` -> Hello, Eisenhorn
    let hello = warp::path!("reserve" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 4242)).await;

    info!("Done");
}
