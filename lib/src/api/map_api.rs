use nalgebra::{Point, Point2};
use serde_derive::Deserialize;
use crate::api::matchup::Worlds;
use crate::api::world_map::WorldMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    pub id: u32,
    pub map_rect: [(i32, i32); 2],
    pub continent_rect: [(u32, u32); 2],
}

