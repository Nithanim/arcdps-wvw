use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::images::{ImageData, load_image_file};


static OBJECTIVE_CASTLE: Lazy<ImageData> = Lazy::new(|| load_image_file(include_bytes!("../resources/icons/Objective_Castle.png")));
static OBJECTIVE_KEEP: Lazy<ImageData> = Lazy::new(|| load_image_file(include_bytes!("../resources/icons/Objective_Keep.png")));
static OBJECTIVE_TOWER: Lazy<ImageData> = Lazy::new(|| load_image_file(include_bytes!("../resources/icons/Objective_Tower.png")));
static OBJECTIVE_CAMP: Lazy<ImageData> = Lazy::new(|| load_image_file(include_bytes!("../resources/icons/Objective_Camp.png")));
static OBJECTIVE_SENTRY: Lazy<ImageData> = Lazy::new(|| load_image_file(include_bytes!("../resources/icons/Objective_Castle.png")));

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Icon {
    ObjectiveCastle,
    ObjectiveKeep,
    ObjectiveTower,
    ObjectiveCamp,
    ObjectiveSentry,
}

impl Icon {
    pub fn value(&self) -> &ImageData {
        match *self {
            Icon::ObjectiveCastle => OBJECTIVE_CASTLE.deref(),
            Icon::ObjectiveKeep => OBJECTIVE_KEEP.deref(),
            Icon::ObjectiveTower => OBJECTIVE_TOWER.deref(),
            Icon::ObjectiveCamp => OBJECTIVE_CAMP.deref(),
            Icon::ObjectiveSentry => OBJECTIVE_SENTRY.deref(),
        }
    }
}

