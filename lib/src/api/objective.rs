use serde_derive::Deserialize;
use crate::api::owner::Faction;

#[derive(Deserialize, Debug)]
pub struct Objective {
    id: String,
    owner: Faction,
    #[serde(alias = "type")]
    type_: Type,

}

#[derive(Deserialize, Debug)]
pub enum Type {
    #[serde(alias = "Spawn")]
    SPAWN,
    #[serde(alias = "Tower")]
    TOWER,
    #[serde(alias = "Camp")]
    CAMP,
    #[serde(alias = "Keep")]
    KEEP,
    #[serde(alias = "Castle")]
    CASTLE,
    #[serde(alias = "Mercenary")]
    MERCENARY,
    #[serde(alias = "Ruins")]
    RUINS,
}