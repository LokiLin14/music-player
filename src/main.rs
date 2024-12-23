use log::{debug, info, warn};
use simple_logger::SimpleLogger;

mod model;
mod actions;
mod app;
pub mod components;
mod tui;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    // warn!("This is an example message.");
    // debug!("This is an example message.");
    // info!("This is an example message.");

    let app = app::App::new();
}
