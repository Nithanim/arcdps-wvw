use std::collections::HashMap;
use std::ffi::CString;
use std::time::Instant;
use c_str_macro::c_str;
use imgui_sys::*;
use crate::api::objective_definition;
use crate::api::objective_definition::ObjectiveDefinition;
use crate::{icons, ImGuiIcon};
use crate::api::matchup::Matchup;
use crate::api::objective::Objective;
use crate::api::owner::OwningForce;
use crate::data::SharedData;


pub unsafe fn render_map(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>) {
    const MAP: &str = "Center";
    let single_map_objective_definitions: Vec<&ObjectiveDefinition> = objectives.iter()
        .filter(|&e| e.map_type == MAP)
        .filter(|&e| match &e.type_ {
            objective_definition::Type::CAMP | objective_definition::Type::TOWER | objective_definition::Type::KEEP | objective_definition::Type::CASTLE => true,
            _default => false
        })
        .collect();

    let matchup_opt: Option<Result<&Matchup, &()>>;
    if shared_data.is_none() {
        matchup_opt = None;
    } else {
        let x: &SharedData = shared_data.unwrap();
        let result = x.matchup.as_ref();
        matchup_opt = Some(result);
    }

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
            .filter(|e| e.type_ == MAP)
            .flat_map(|e| &e.objectives)
            .map(|e| (&e.id, e))
            .collect();
    } else {
        objective_states = HashMap::new();
    }

    igBegin(c_str!("WvW").as_ptr(), &mut true, 0);

    let text: String;
    if shared_data.is_some() {
        let data_timestamp = shared_data.unwrap().timestamp;
        let now = Instant::now();

        let diff = now.duration_since(data_timestamp);

        text = format!("Last update: {} sec", diff.as_secs());
    } else {
        text = String::from("No data");
    }
    let string = CString::new(text).unwrap();
    igText(string.as_ptr());

    render_map_(&single_map_objective_definitions, icons, &objective_states);

    igEnd();
}

pub unsafe fn render_map_(objective_definitions: &Vec<&ObjectiveDefinition>,
                          icons: &HashMap<icons::Icon, ImGuiIcon>,
                          objectives: &HashMap<&String, &Objective>) {
    let mut pos = ImVec2::zero();
    igGetCursorPos(&mut pos);
    //println!("{}, {}", pos.x, pos.y);
    let mut available_area = ImVec2::zero();
    igGetContentRegionAvail(&mut available_area);

    let uv0 = ImVec2::new(0.0, 0.0);
    let uv1 = ImVec2::new(1.0, 1.0);
    let border_color = ImVec4::new(0.0, 0.0, 0.0, 0.0);

    let map_dimensions = calc_map_dimensions(objective_definitions);

    let icon_size = ImVec2::new(32f32, 32f32);

    for objective_def in objective_definitions {
        let objective_live = objectives.get(&objective_def.id);

        let objective_icon = match &objective_def.type_ {
            objective_definition::Type::CAMP => Some(&icons::Icon::ObjectiveCamp),
            objective_definition::Type::TOWER => Some(&icons::Icon::ObjectiveTower),
            objective_definition::Type::KEEP => Some(&icons::Icon::ObjectiveKeep),
            objective_definition::Type::CASTLE => Some(&icons::Icon::ObjectiveCastle),
            _default => None,
        }.map(|m| icons.get(m).unwrap());

        let x = (objective_def.coord.unwrap()[0] - map_dimensions.min_x) / map_dimensions.w * (available_area.x - icon_size.x);
        let y = (objective_def.coord.unwrap()[1] - map_dimensions.min_y) / map_dimensions.h * (available_area.y - icon_size.x);

        igSetCursorPos(ImVec2::new(pos.x + x, pos.y + y));

        if objective_icon.is_some() {
            let tint = get_owning_force_tint_objective(&objective_live);

            igImage(
                objective_icon.unwrap().to_imgui_id(),
                objective_icon.unwrap().size, uv0,
                uv1,
                tint,
                border_color,
            );

            if igIsItemHovered(ImGuiHoveredFlags_None as ImGuiHoveredFlags) {
                igBeginTooltip();
                let string = CString::new(objective_def.name.as_str()).unwrap();
                igText(string.as_ptr());
                igEndTooltip();
            }
        }
    }

    //igSetCursorPos(ImVec2::new(pos.x + available_area.x, pos.y + available_area.y));
    igSetCursorPos(pos);
    igDummy(ImVec2::new(available_area.x, available_area.y));

    //igDummy(ImVec2::new(available_area.x, available_area.y));
}

unsafe fn get_owning_force_tint_objective(objective_live: &Option<&&Objective>) -> ImVec4 {
    get_owning_force_tint_force(&objective_live
        .map(|e| &e.owner)
        .unwrap_or(&OwningForce::NEUTRAL))
}

fn get_owning_force_tint_force(o: &OwningForce) -> ImVec4 {
    let power = 0.3;
    match o {
        OwningForce::RED => ImVec4::new(1.0, power, power, 1.0),
        OwningForce::GREEN => ImVec4::new(power, 1.0, power, 1.0),
        OwningForce::BLUE => ImVec4::new(power, power, 1.0, 1.0),
        OwningForce::NEUTRAL => ImVec4::new(1.0, 1.0, 1.0, 1.0),
    }
}

fn calc_map_dimensions(objective_definitions: &Vec<&ObjectiveDefinition>) -> MapDimensions {
    let all_coords: Vec<[f32; 3]> = objective_definitions
        .iter()
        .map(|c| c.coord)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect();

    let min_x = all_coords
        .iter()
        .map(|c| c[0])
        .reduce(f32::min)
        .unwrap_or(0.0);
    let max_x = all_coords.iter()
        .map(|c| c[0])
        .reduce(f32::max).unwrap_or(0.0);
    let min_y = all_coords
        .iter()
        .map(|c| c[1])
        .reduce(f32::min).unwrap_or(0.0);
    let max_y = all_coords.iter()
        .map(|c| c[1])
        .reduce(f32::max).unwrap_or(0.0);

    let w = max_x - min_x;
    let h = max_y - min_y;

    MapDimensions {
        min_x,
        max_x,
        min_y,
        max_y,
        w,
        h,
    }
}

struct MapDimensions {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    w: f32,
    h: f32,
}
