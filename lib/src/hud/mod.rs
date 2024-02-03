use std::collections::HashMap;
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkReader};
use crate::{icons, ImGuiIcon, is_game, is_in_loading_screen, MUMBLE_LINK};
use crate::api::objective_definition::ObjectiveDefinition;
use crate::data::SharedData;
use crate::hud::world2d::debug;
use crate::settings::Settings;

pub mod screen;
mod world3d;
mod world2d;


pub unsafe fn render(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, settings: &mut Settings) {
    let ml = get_mumble_link();
    debug::render_debug(settings, ml.as_ref());

    if ml.is_some() && !is_in_loading_screen() {
        world3d::render_hud(settings, ml.as_ref().unwrap(), icons, shared_data, objectives);
    }
    world2d::render_map(objectives, icons, shared_data, settings);
    if is_game() {
        if ml.is_some() && !is_in_loading_screen() {
            world2d::render_compass(ml.as_ref().unwrap(), settings);
        }
    } else {
        world2d::render_compass_dummy(settings);
    }
}


fn get_mumble_link() -> Option<MumbleLinkData> {
    unsafe {
        let handler = MUMBLE_LINK.as_ref();
        if handler.is_some() {
            let linked_memory = handler.unwrap().read().unwrap();

            if linked_memory.name == "Guild Wars 2" && world3d::helpers::get_current_map_id(&linked_memory) != 0 {
                return Some(linked_memory);
            }
        }
        return None;
    }
}