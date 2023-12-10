use crate::api::matchup::Worlds;
use crate::api::owner::Faction;
use crate::api::world_map_type::WorldMapType;

pub fn get_wold_map_type(faction: Faction) -> WorldMapType {
    return match faction {
        Faction::RED => WorldMapType::RED,
        Faction::GREEN => WorldMapType::GREEN,
        Faction::BLUE => WorldMapType::BLUE,
    };
}

pub fn get_home_world_faction(worlds: &Worlds, home_world: i32) -> Option<Faction> {
    return if worlds.red.iter().find(|&&w| w == home_world).is_some() {
        Some(Faction::RED)
    } else if worlds.green.iter().find(|&&w| w == home_world).is_some() {
        Some(Faction::GREEN)
    } else if worlds.blue.iter().find(|&&w| w == home_world).is_some() {
        Some(Faction::BLUE)
    } else {
        None
    };
}

