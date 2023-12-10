mod rendering;

use std::collections::HashMap;
use std::ffi::CString;
use std::time::Instant;
use imgui_sys::*;
use crate::api::objective_definition;
use crate::api::objective_definition::ObjectiveDefinition;
use crate::{icons, ImGuiIcon};
use crate::api::matchup::Matchup;
use crate::api::objective::Objective;
use crate::api::world_map_type::WorldMapType;
use crate::data::SharedData;
use crate::map_renderer::rendering::render_map;
use crate::settings::Settings;

pub struct MapWindow<'a> {
    objectives: &'a Vec<ObjectiveDefinition>,
    icons: &'a HashMap<icons::Icon, ImGuiIcon>,
    shared_data: Option<&'a SharedData>,
}

impl MapWindow<'_> {}

struct Data<'a> {
    objective_definitions: Vec<&'a ObjectiveDefinition>,
    objective_states: HashMap<&'a String, &'a Objective>,
    icons: &'a HashMap<icons::Icon, ImGuiIcon>,
}

pub unsafe fn render(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, settings: &mut Settings) {
    //let map_types_to_render = get_map_types_to_render(settings, );

    let pre_computed: HashMap<WorldMapType, Data> = pre_compute(objectives, icons, shared_data);


    if settings.show_eternal {
        let option = pre_computed.get(&WorldMapType::ETERNAL);
        render_pre("Eternal battlegrounds", option.unwrap(), shared_data, &mut settings.show_eternal);
    }
    if settings.show_red {
        render_pre("Red borderlands", pre_computed.get(&WorldMapType::RED).unwrap(), shared_data, &mut settings.show_red);
    }
    if settings.show_green {
        render_pre("Green borderlands", pre_computed.get(&WorldMapType::GREEN).unwrap(), shared_data, &mut settings.show_green);
    }
    if settings.show_blue {
        render_pre("Blue borderlands", pre_computed.get(&WorldMapType::BLUE).unwrap(), shared_data, &mut settings.show_blue);
    }
    if settings.show_current {
        render_pre("Current borderlands", pre_computed.get(&WorldMapType::ETERNAL).unwrap(), shared_data, &mut settings.show_current);
    }
}

fn pre_compute<'a>(objectives: &'a Vec<ObjectiveDefinition>, icons: &'a HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&'a SharedData>) -> HashMap<WorldMapType, Data<'a>> {
    let mut result: HashMap<WorldMapType, Data<'a>> = HashMap::new();

    let interesting_objective_definitions: Vec<&ObjectiveDefinition> = objectives.iter()
        .filter(is_interesting_objective)
        .collect();


    for wmt in WorldMapType::into_iter() {
        let map_type = match wmt {
            WorldMapType::RED => "RedHome",
            WorldMapType::GREEN => "GreenHome",
            WorldMapType::BLUE => "BlueHome",
            WorldMapType::ETERNAL => "Center",
        };

        let single_map_objective_definitions: Vec<&ObjectiveDefinition> = filter_objective_defs_by_map(&interesting_objective_definitions, map_type);

        let matchup_opt: Option<Result<&Matchup, &()>> = shared_data.map(|x| x.matchup.as_ref());

        let matchup: Option<&Matchup>;
        if matchup_opt.is_none() {
            matchup = None;
        } else {
            let e = matchup_opt.unwrap();
            if e.is_err() {
                matchup = None;
            } else {
                matchup = e.ok();
            }
        }

        let objective_states: HashMap<&String, &Objective>;
        if matchup.is_some() {
            objective_states = matchup.as_ref().unwrap().maps.iter()
                .filter(|e| e.type_ == map_type)
                .flat_map(|e| &e.objectives)
                .map(|e| (&e.id, e))
                .collect();
        } else {
            objective_states = HashMap::new();
        }

        result.insert(wmt, Data {
            icons: icons,
            objective_states: objective_states,
            objective_definitions: single_map_objective_definitions,
        });
    }

    result
}

unsafe fn render_pre(title: &str, data: &Data, shared_data: Option<&SharedData>, window_open: &mut bool) {
    let window_name = CString::new(title).unwrap();
    if igBegin(window_name.as_ptr(), window_open, 0) {}

    let string = CString::new(get_last_updated_text(shared_data)).unwrap();
    igText(string.as_ptr());

    render_map(&data.objective_definitions, data.icons, &data.objective_states);

    igEnd();
}

fn get_last_updated_text(shared_data: Option<&SharedData>) -> String {
    let text: String;
    if shared_data.is_some() {
        let data_timestamp = shared_data.unwrap().timestamp;
        let now = Instant::now();

        let diff = now.duration_since(data_timestamp);

        text = format!("Last update: {} sec", diff.as_secs());
    } else {
        text = String::from("No data");
    }
    text
}

fn filter_objective_defs_by_map<'a>(interesting_objective_definitions: &Vec<&'a ObjectiveDefinition>, map_type: &str) -> Vec<&'a ObjectiveDefinition> {
    interesting_objective_definitions.iter()
        .copied()
        .filter(|&e| e.map_type == map_type)
        .collect()
}

fn is_interesting_objective(e: &&ObjectiveDefinition) -> bool {
    match &e.type_ {
        objective_definition::Type::CAMP | objective_definition::Type::TOWER | objective_definition::Type::KEEP | objective_definition::Type::CASTLE => true,
        _default => false
    }
}

