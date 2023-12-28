pub mod http_client;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use crate::api::matchup::Matchup;
use crate::data::http_client::get_http_client;

pub static DATA: Lazy<Arc<Mutex<Option<SharedData>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));


static mut SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn shutdown() {
    unsafe { SHUTDOWN.store(true, Ordering::Relaxed) }
}

pub fn init() {
    let data_clone = Arc::clone(&DATA);

    thread::spawn(move || {
        loop {
            let matchup = fetch_matchup();

            if matchup.is_err() {
                eprintln!("Error updating data! {}", matchup.unwrap_err());
            } else {
                let new_data = SharedData {
                    matchup: matchup.map_err(|_| ()),
                    timestamp: Instant::now(),
                };

                let mut data_lock = data_clone.lock().unwrap();
                *data_lock = Some(new_data);
                drop(data_lock);
            }

            thread::sleep(Duration::from_millis(9500));

            unsafe {
                if SHUTDOWN.load(Ordering::Relaxed) {
                    break;
                }
            }
        }
    });
}

fn fetch_matchup() -> Result<Matchup, String> { // Change Value to your specific type
    let url = "https://api.guildwars2.com/v2/wvw/matches?world=2204";


    let client = match get_http_client() {
        Some(c) => c,
        None => return Err("Http client not available!".to_string())
    };

    let result = client.get(url).send();

    let response;
    match result {
        Ok(d) => response = d,
        Err(e) => return Err(e.to_string()),
    }

    let json_decoded: Result<Matchup, _> = response.json();
    json_decoded.map_err(|e| e.to_string())
}

pub struct SharedData {
    pub matchup: Result<Matchup, ()>,
    pub timestamp: Instant,
}
