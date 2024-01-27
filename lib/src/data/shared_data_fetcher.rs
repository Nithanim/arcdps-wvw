use std::sync::{mpsc};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use crate::api::map_api::Map;
use crate::api::matchup::Matchup;
use crate::data::http_client::{get_async_client};
use crate::data::{SharedData};
use crate::settings::{get_settings, Settings};
use crate::utils::{drop_static_mut_option, swap_static_mut_option};

static MAPS: Lazy<Vec<Map>> = Lazy::new(|| {
    let bytes = include_bytes!("../../resources/cache/maps.json");
    let r = serde_json::from_slice::<Vec<Map>>(bytes);
    match r {
        Ok(o) => o,
        Err(e) => {
            panic!("ERROR: {}", e);
        }
    }
});
static mut DATA: Option<SharedData> = None;

static mut RUNTIME: Option<tokio::runtime::Runtime> = None;
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

static mut RECEIVER: Option<Receiver<Option<SharedData>>> = None;

pub fn get_shared_data() -> Option<&'static SharedData> {
    unsafe {
        DATA.as_ref()
    }
}


pub fn setup() {
    unsafe {
        SHUTDOWN.store(false, Ordering::Relaxed);
        RUNTIME = Some(tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap());
    }

    let (channel_sender, channel_receiver) = mpsc::channel::<Option<SharedData>>();

    unsafe {
        RECEIVER = Some(channel_receiver);
    }

    thread::spawn(move || {
        loop {
            if settings_need_data(get_settings()) {
                let started_at = Instant::now();

                let future = download_and_get_data_async();
                // TODO make this not shitty but this async result stuff was too complicated
                unsafe {
                    if RUNTIME.is_some() {
                        let result = RUNTIME.as_ref().unwrap().block_on(future);
                        channel_sender.send(result);
                    }
                }

                let time_to_wait = 9800 - started_at.elapsed().as_millis() as u64;
                if time_to_wait > 1 {
                    thread::sleep(Duration::from_millis(time_to_wait));
                }
            } else {
                thread::sleep(Duration::from_secs(10));
            }
            if SHUTDOWN.load(Ordering::Relaxed) {
                break;
            }
        }
    });
}

pub fn shutdown() {
    unsafe {
        SHUTDOWN.store(true, Ordering::Relaxed);

        // TODO All this nonsense around static mut vars is inherently not correctly managed and should be replaced
        // But for GW2 this has to suffice for now.
        drop_static_mut_option(&mut RUNTIME);
        drop_static_mut_option(&mut RECEIVER);
    }
}

pub fn tick() {
    unsafe {
        match &RECEIVER {
            None => {}
            Some(receiver) => {
                match receiver.try_recv() {
                    Ok(data) => {
                        unsafe {
                            swap_static_mut_option(&mut DATA, data);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}


pub async fn download_and_get_data_async() -> Option<SharedData> {
    let world_id = get_world_id();

    let matchup: Result<Matchup, String>;
    if world_id > 1000 {
        matchup = fetch_matchup_async(world_id).await;
    } else {
        return None;
    }


    if matchup.is_err() {
        eprintln!("Error updating data! {}", matchup.unwrap_err());
        return None;
    } else {
        let new_data = SharedData {
            matchup: matchup.map_err(|_| ()),
            maps: Some(MAPS.clone()),
            timestamp: Instant::now(),
        };

        return Some(new_data);
    }
}

async fn fetch_matchup_async(world_id: i32) -> Result<Matchup, String> { // Change Value to your specific type
    let url = format!("https://api.guildwars2.com/v2/wvw/matches?world={}", world_id);


    let client = match get_async_client() {
        Some(c) => c,
        None => return Err("Http client not available!".to_string())
    };

    let result = client.get(url).send().await;

    let response;
    match result {
        Ok(d) => response = d,
        Err(e) => return Err(e.to_string()),
    }

    let json_decoded: Result<Matchup, _> = response.json().await;
    json_decoded.map_err(|e| e.to_string())
}

fn settings_need_data(settings: &Settings) -> bool {
    settings.show_red || settings.show_green || settings.show_blue || settings.show_eternal || settings.show_objectives_overlay
}

fn get_world_id() -> i32 {
    unsafe {
        core::ptr::read_volatile(&get_settings().world_id) // This is not thread-safe but maybe works.
    }
}
