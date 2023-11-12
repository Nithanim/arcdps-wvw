use serde_derive::Deserialize;
use crate::api::objective::Objective;

#[derive(Deserialize, Debug)]
pub struct WorldMap {
    pub id: i32,
    #[serde(alias = "type")]
    pub type_: String,
    pub objectives: Vec<Objective>,
}
