use std::collections::HashMap;
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkReader};
use crate::{icons, ImGuiIcon, MUMBLE_LINK, SETTINGS};
use crate::api::objective_definition::ObjectiveDefinition;
use crate::data::SharedData;
use crate::settings::Settings;

pub mod screen;
mod world3d;
mod world2d;


pub fn render3d() {
    unsafe {
        let handler = MUMBLE_LINK.as_ref();
        if handler.is_some() {
            let linked_memory = handler.unwrap().read().unwrap();

            if linked_memory.name == "Guild Wars 2" && world3d::get_current_map_id(&linked_memory) != 0 {}
        }
    }
}

pub unsafe fn render(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, settings: &mut Settings) {
    let ml = get_mumble_link();
    if ml.is_some() {
        world3d::render_hud(settings, ml.as_ref().unwrap());
    }
    world2d::render_map(objectives, icons, shared_data, settings);
    if ml.is_some() {
        world2d::render_compass(ml.as_ref().unwrap(), settings);
    } else {
        world2d::render_compass_dummy(settings);
    }
}


fn get_mumble_link() -> Option<MumbleLinkData> {
    unsafe {
        let handler = MUMBLE_LINK.as_ref();
        if handler.is_some() {
            let linked_memory = handler.unwrap().read().unwrap();

            if linked_memory.name == "Guild Wars 2" && world3d::get_current_map_id(&linked_memory) != 0 {
                return Some(linked_memory);
            }
        }
        return None;
    }
}