use serde_derive::Deserialize;
use crate::api::objective_definition::Type;
use crate::api::owner::Faction;

#[derive(Deserialize, Debug)]
pub struct Objective {
    id: String,
    owner: Faction,
    #[serde(alias = "type")]
    type_: Type,
}
