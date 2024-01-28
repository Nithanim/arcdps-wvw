pub mod helpers;
pub mod positions;
mod objectives_overlay;
mod graphics_uniform;

use std::collections::HashMap;
use c_str_macro::c_str;
use imgui_sys::{igBegin, igEnd, igSetNextWindowPos, igSetNextWindowSize, ImGuiWindowFlags, ImVec2};
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkDataReader, MumbleLinkReader};
use crate::api::objective_definition::ObjectiveDefinition;
use crate::data::SharedData;
use crate::hud::{screen};
use crate::{icons, ImGuiIcon};
use crate::settings::Settings;

const WINDOW_FLAGS: ImGuiWindowFlags = (
    imgui_sys::ImGuiWindowFlags_NoBackground
        | imgui_sys::ImGuiWindowFlags_NoInputs
        | imgui_sys::ImGuiWindowFlags_NoNav
        | imgui_sys::ImGuiWindowFlags_NoDecoration
        | imgui_sys::ImGuiWindowFlags_NoSavedSettings
        | imgui_sys::ImGuiWindowFlags_NoFocusOnAppearing
        | imgui_sys::ImGuiWindowFlags_NoBringToFrontOnFocus) as ImGuiWindowFlags;

pub unsafe fn render_hud(settings: &Settings, ml: &MumbleLinkData, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, objective_defintions: &Vec<ObjectiveDefinition>) {
    igSetNextWindowPos(ImVec2::new(0f32, 0f32), 0, ImVec2::zero());
    igSetNextWindowSize(screen::get_screen_size(), 0);

    igBegin(c_str!("3d World Overlay").as_ptr(), &mut true, WINDOW_FLAGS as ImGuiWindowFlags);
    objectives_overlay::render_overlay(settings, ml, icons, shared_data, objective_defintions);

    //up is (0, 1, 0)

    igEnd();
}

