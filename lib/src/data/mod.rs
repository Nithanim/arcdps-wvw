use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use rustls::ClientConfig;
use crate::api::matchup::Matchup;

pub static DATA: Lazy<Arc<Mutex<Option<SharedData>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

static mut HTTP_CLIENT: Option<reqwest::blocking::Client> = None;

pub fn init() {
    unsafe {
        HTTP_CLIENT = Some(create_http_client());
    }

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
        }
    });
}

fn fetch_matchup() -> Result<Matchup, reqwest::Error> { // Change Value to your specific type
    let url = "https://api.guildwars2.com/v2/wvw/matches?world=2204";

    let result = unsafe { HTTP_CLIENT.as_ref() }.unwrap().get(url).send();

    let response;
    match result {
        Ok(d) => response = d,
        Err(e) => return Err(e),
    }

    let json_decoded: Result<Matchup, _> = response.json();
    json_decoded
}

fn create_http_client() -> reqwest::blocking::Client {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        roots
            .add(&rustls::Certificate(cert.0))
            .unwrap();
    }

    let builder = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();


    reqwest::blocking::Client::builder()
        .use_preconfigured_tls(builder)
        .build()
        .unwrap()
}

pub struct SharedData {
    pub matchup: Result<Matchup, ()>,
    pub timestamp: Instant,
}
