#[derive(Hash, Eq, PartialEq)]
pub enum WorldMapType {
    RED,
    GREEN,
    BLUE,
    ETERNAL,
}


impl WorldMapType {
    pub fn into_iter() -> core::array::IntoIter<WorldMapType, 4> {
        [
            WorldMapType::RED,
            WorldMapType::GREEN,
            WorldMapType::BLUE,
            WorldMapType::ETERNAL,
        ]
            .into_iter()
    }
}
