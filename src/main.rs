mod model;
mod ui;
mod music_player;
mod app;

use log::{debug, info, warn};
use simple_logger::{SimpleLogger};

fn main() {
    SimpleLogger::new().init().unwrap();

    warn!("This is an example message.");
    debug!("This is an example message.");
    info!("This is an example message.");
}
