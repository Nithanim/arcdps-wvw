use std::ops::Deref;
use imgui_sys::ImVec2;
use once_cell::sync::Lazy;


static OBJECTIVE_CASTLE: Lazy<IconData> = Lazy::new(|| to_data(include_bytes!("../resources/icons/Objective_Castle.png")));
static OBJECTIVE_KEEP: Lazy<IconData> = Lazy::new(|| to_data(include_bytes!("../resources/icons/Objective_Keep.png")));
static OBJECTIVE_TOWER: Lazy<IconData> = Lazy::new(|| to_data(include_bytes!("../resources/icons/Objective_Tower.png")));
static OBJECTIVE_CAMP: Lazy<IconData> = Lazy::new(|| to_data(include_bytes!("../resources/icons/Objective_Camp.png")));
static OBJECTIVE_SENTRY: Lazy<IconData> = Lazy::new(|| to_data(include_bytes!("../resources/icons/Objective_Castle.png")));

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Icon {
    ObjectiveCastle,
    ObjectiveKeep,
    ObjectiveTower,
    ObjectiveCamp,
    ObjectiveSentry,
}

pub struct IconData {
    pub bytes: Vec<u8>,
    pub size: ImVec2,
}

impl Icon {
    pub fn value(&self) -> &IconData {
        match *self {
            Icon::ObjectiveCastle => OBJECTIVE_CASTLE.deref(),
            Icon::ObjectiveKeep => OBJECTIVE_KEEP.deref(),
            Icon::ObjectiveTower => OBJECTIVE_TOWER.deref(),
            Icon::ObjectiveCamp => OBJECTIVE_CAMP.deref(),
            Icon::ObjectiveSentry => OBJECTIVE_SENTRY.deref(),
        }
    }
}

fn to_data(file: &[u8]) -> IconData {
    let img = image::load_from_memory(file).unwrap();
    IconData {
        bytes: img.to_rgba8().into_raw(),
        size: ImVec2::new(img.width() as f32, img.height() as f32),
    }
}