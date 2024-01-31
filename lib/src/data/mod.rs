pub mod http_client;
mod threads;
mod shared_data_fetcher;
mod shared_data_fetcher_thread;

use std::time::Instant;
pub use shared_data_fetcher::tick;
pub use shared_data_fetcher::get_shared_data;
use crate::api::matchup::Matchup;
use crate::api::map_api::Map;

pub struct SharedData {
    pub matchup: Result<Matchup, ()>,
    pub maps: Option<Vec<Map>>,
    pub timestamp: Instant,
}

pub fn setup() {
    threads::setup();
    shared_data_fetcher::setup();
}

pub fn shutdown() {
    shared_data_fetcher::shutdown();
    threads::shutdown();
}
