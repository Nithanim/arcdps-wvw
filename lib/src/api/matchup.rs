use std::iter::Map;
use serde_derive::Deserialize;
use crate::api::world_map::WorldMap;

#[derive(Deserialize, Debug)]
pub struct Matchup {
    pub id: String,
    pub all_worlds: Worlds,
    pub maps: Vec<WorldMap>,
}

#[derive(Deserialize, Debug)]
pub struct Worlds {
    pub red: Vec<i32>,
    pub blue: Vec<i32>,
    pub green: Vec<i32>
}