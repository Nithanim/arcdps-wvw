use std::time::{Instant};
use once_cell::sync::Lazy;
use crate::api::map_api::Map;
use crate::api::matchup::Matchup;
use crate::data::http_client::{get_async_client};
use crate::data::{SharedData};
use crate::settings::{get_settings};

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

pub async fn download_and_get_data_async() -> Option<SharedData> {
    let world_id = get_world_id();

    let matchup: Result<Matchup, String>;
    if world_id > 1000 {
        matchup = fetch_matchup_async(world_id).await;
    } else {
        return None;
    }


    return if matchup.is_err() {
        eprintln!("Error updating data! {}", matchup.unwrap_err());
        None
    } else {
        let new_data = SharedData {
            matchup: matchup.map_err(|_| ()),
            maps: Some(MAPS.clone()),
            timestamp: Instant::now(),
        };

        Some(new_data)
    };
}

async fn fetch_matchup_async(world_id: i32) -> Result<Matchup, String> { // Change Value to your specific type
    let url = format!("https://api.guildwars2.com/v2/wvw/matches?world={}", world_id);


    let client = match get_async_client() {
        Some(c) => c,
        None => return Err("Http client not available!".to_string())
    };

    let result = client.get(url).send().await;

    // This weirdly can result in a 404 with
    //{
    //   "text": "world not currently in a match"
    // }

    let response;
    match result {
        Ok(d) => response = d,
        Err(e) => return Err(e.to_string()),
    }

    let json_decoded: Result<Matchup, _> = response.json().await;
    json_decoded.map_err(|e| e.to_string())
}

fn get_world_id() -> i32 {
    unsafe {
        core::ptr::read_volatile(&get_settings().world_id) // This is not thread-safe but maybe works.
    }
}
