use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Objective {
    id: String,
    map_id: i32,
}