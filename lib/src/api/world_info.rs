use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorldInfo {
    pub id: i32,
    pub name: String,
}
