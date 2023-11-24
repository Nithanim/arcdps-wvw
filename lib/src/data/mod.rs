use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use crate::api::matchup::Matchup;

pub static DATA: Lazy<Arc<Mutex<Option<SharedData>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn init() {
    let data_clone = Arc::clone(&DATA);

    thread::spawn(move || {
        loop {
            let matchup = fetch_matchup();

            let new_data = SharedData {
                matchup: matchup.map_err(|_| ()),
                timestamp: Instant::now(),
            };

            let mut data_lock = data_clone.lock().unwrap();
            *data_lock = Some(new_data);
            drop(data_lock);

            thread::sleep(Duration::from_millis(9500));
        }
    });
}

fn fetch_matchup() -> Result<Matchup, reqwest::Error> { // Change Value to your specific type
    let url = "https://api.guildwars2.com/v2/wvw/matches?world=2204";
    let client = reqwest::blocking::Client::new();
    let result = client.get(url).send();

    let response;
    match result {
        Ok(d) => response = d,
        Err(e) => return Err(e),
    }

    let json_decoded: Result<Matchup, _> = response.json();
    json_decoded
}

pub struct SharedData {
    pub matchup: Result<Matchup, ()>,
    pub timestamp: Instant,
}
