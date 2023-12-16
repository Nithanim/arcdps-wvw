use std::collections::HashMap;
use crate::api::objective_definition::ObjectiveDefinition;
use crate::{icons, ImGuiIcon};
use crate::data::SharedData;
use crate::settings::Settings;

pub mod map_renderer;

pub unsafe fn render(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, settings: &mut Settings) {
    map_renderer::render(objectives, icons, shared_data, settings);
}