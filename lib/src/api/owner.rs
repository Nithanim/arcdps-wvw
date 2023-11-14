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

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum OwningForce {
    #[serde(alias = "Red")]
    RED,
    #[serde(alias = "Green")]
    GREEN,
    #[serde(alias = "Blue")]
    BLUE,
    #[serde(alias = "Neutral")]
    NEUTRAL,
}
