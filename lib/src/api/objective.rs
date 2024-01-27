use serde_derive::Deserialize;
use crate::api::objective_definition::Type;
use crate::api::owner::OwningForce;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
pub struct Objective {
    pub id: String,
    pub owner: OwningForce,
    #[serde(alias = "type")]
    pub type_: Type,
    #[serde(default)]
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_flipped: Option<OffsetDateTime>,
}
