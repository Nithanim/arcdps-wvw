use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObjectiveDefinition {
    pub id: String,
    pub name: String,
    #[serde(alias = "type")]
    pub type_: Type,
    pub map_id: i32,
    pub map_type: String,
    /**
    0: - to + is left to right on map (x),
    1: - to + is top to bottom on map (z)
    2: - to + is higher (hill) to lower (water-level) (y) | water-level is 0; above is negative
     */
    pub coord: Option<[f32; 3]>,
}


#[derive(Deserialize, Debug, PartialEq)]
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
