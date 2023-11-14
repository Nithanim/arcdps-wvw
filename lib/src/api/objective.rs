use serde_derive::Deserialize;
use crate::api::objective_definition::Type;
use crate::api::owner::OwningForce;

#[derive(Deserialize, Debug)]
pub struct Objective {
    pub id: String,
    pub owner: OwningForce,
    #[serde(alias = "type")]
    pub type_: Type,
}
