use std::collections::HashMap;
use imgui_sys::{ImGuiWindowFlags, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoDecoration, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoNav};
use mumblelink_reader::mumble_link::{MumbleLinkReader};
use crate::{icons, ImGuiIcon, MUMBLE_LINK, SETTINGS};
use crate::api::objective_definition::ObjectiveDefinition;
use crate::data::SharedData;
use crate::settings::Settings;

pub mod screen;
mod world3d;
mod world2d;

const WINDOW_FLAGS: ImGuiWindowFlags = (ImGuiWindowFlags_NoBackground
    | ImGuiWindowFlags_NoInputs
    | ImGuiWindowFlags_NoNav
    | ImGuiWindowFlags_NoDecoration) as ImGuiWindowFlags;

pub fn render3d() {
    unsafe {
        let handler = MUMBLE_LINK.as_ref();
        if handler.is_some() {
            let linked_memory = handler.unwrap().read().unwrap();

            if linked_memory.name == "Guild Wars 2" && world3d::get_current_map_id(&linked_memory) != 0 {
                world3d::render_hud(&SETTINGS, &linked_memory)
            }
        }
    }
}

pub unsafe fn render2d(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, settings: &mut Settings) {
    world2d::render(objectives, icons, shared_data, settings);
}
