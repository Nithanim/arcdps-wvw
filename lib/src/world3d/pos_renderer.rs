use c_str_macro::c_str;
use imgui_sys::*;
use mumblelink_reader::mumble_link::{MumbleLinkReader};
use crate::world3d::screen::get_screen_size;
use crate::{MUMBLE_LINK, SETTINGS};
use crate::world3d::hud::render_hud;

const WINDOW_FLAGS: ImGuiWindowFlags = (ImGuiWindowFlags_NoBackground
    | ImGuiWindowFlags_NoInputs
    | ImGuiWindowFlags_NoNav
    | ImGuiWindowFlags_NoDecoration) as ImGuiWindowFlags;

pub unsafe fn render() {
    igSetNextWindowPos(ImVec2::new(0f32, 0f32), 0, ImVec2::zero());
    igSetNextWindowSize(get_screen_size(), 0);

    igBegin(c_str!("Full").as_ptr(), &mut true, WINDOW_FLAGS as ImGuiWindowFlags);

    let handler = MUMBLE_LINK.as_ref();
    if handler.is_some() {
        let linked_memory = handler.unwrap().read().unwrap();

        if linked_memory.name == "Guild Wars 2" {
            render_hud(&SETTINGS, &linked_memory)
        }
    }

    //up is (0, 1, 0)

    igEnd();
}
