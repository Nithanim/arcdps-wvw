use std::collections::HashMap;
use mumblelink_reader::mumble_link::MumbleLinkData;
use crate::api::objective_definition::ObjectiveDefinition;
use crate::{icons, ImGuiIcon};
use crate::data::SharedData;
use crate::settings::Settings;

pub mod map_renderer;
mod compass;

pub unsafe fn render_map(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, settings: &mut Settings) {
    map_renderer::render(objectives, icons, shared_data, settings);
}

pub unsafe fn render_compass(ml: &MumbleLinkData, settings: &mut Settings) {
    compass::render2d(ml, settings)
}

pub unsafe fn render_compass_dummy(settings: &mut Settings) {
    compass::render2d_dummy(settings)
}
