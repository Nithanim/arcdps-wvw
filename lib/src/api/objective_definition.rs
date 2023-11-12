use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObjectiveDefinition {
    pub id: String,
    #[serde(alias = "type")]
    pub type_: Type,
    pub map_id: i32,
    pub map_type: String,
    pub coord: Option<[f32; 3]>,
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
    #[serde(alias = "Resource")]
    RESOURCE,
    #[serde(alias = "Generic")]
    GENERIC,
}
