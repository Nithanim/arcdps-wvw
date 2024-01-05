pub mod http_client;
mod threads;
mod shared_data_fetcher;

use std::time::Instant;
pub use shared_data_fetcher::setup;
pub use shared_data_fetcher::shutdown;
pub use shared_data_fetcher::tick;
pub use shared_data_fetcher::get_shared_data;
use crate::api::matchup::Matchup;

pub struct SharedData {
    pub matchup: Result<Matchup, ()>,
    pub timestamp: Instant,
}
