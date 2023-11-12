use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Faction {
    #[serde(alias = "Red")]
    RED,
    #[serde(alias = "Green")]
    GREEN,
    #[serde(alias = "Blue")]
    BLUE,
}