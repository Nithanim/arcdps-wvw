use imgui_sys::{ImGuiWindowFlags, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoDecoration, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoNav};
use mumblelink_reader::mumble_link::{MumbleLinkReader};
use crate::{MUMBLE_LINK, SETTINGS};

pub mod screen;
mod world3d;

const WINDOW_FLAGS: ImGuiWindowFlags = (ImGuiWindowFlags_NoBackground
    | ImGuiWindowFlags_NoInputs
    | ImGuiWindowFlags_NoNav
    | ImGuiWindowFlags_NoDecoration) as ImGuiWindowFlags;

pub fn render() {
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
