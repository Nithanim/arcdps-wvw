use std::collections::HashMap;
use c_str_macro::c_str;
use imgui_sys::*;
use crate::api::objective_definition;
use crate::api::objective_definition::ObjectiveDefinition;
use crate::{icons, ImGuiIcon};
use crate::api::matchup::Matchup;
use crate::api::objective::Objective;
use crate::api::owner::OwningForce;

fn get_owning_force_tint(o: &OwningForce) -> ImVec4 {
    let power = 0.3;
    match o {
        OwningForce::RED => ImVec4::new(1.0, power, power, 1.0),
        OwningForce::GREEN => ImVec4::new(power, 1.0, power, 1.0),
        OwningForce::BLUE => ImVec4::new(power, power, 1.0, 1.0),
        OwningForce::NEUTRAL => ImVec4::new(1.0, 1.0, 1.0, 1.0),
    }
}

pub unsafe fn render_map(objectives: &Vec<ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, matchup: &Matchup) {
    const MAP: &str = "Center";
    let single_map_objectives: Vec<&ObjectiveDefinition> = objectives.iter()
        .filter(|&e| e.map_type == MAP)
        .filter(|&e| match &e.type_ {
            objective_definition::Type::CAMP | objective_definition::Type::TOWER | objective_definition::Type::KEEP | objective_definition::Type::CASTLE => true,
            _default => false
        })
        .collect();
    let objective_states = matchup.maps.iter()
        .filter(|e| e.type_ == "Center")
        .flat_map(|e| &e.objectives)
        .map(|e| (&e.id, e))
        .collect();


    render_map_(&single_map_objectives, icons, &objective_states);
}

pub unsafe fn render_map_(objective_definitions: &Vec<&ObjectiveDefinition>, icons: &HashMap<icons::Icon, ImGuiIcon>, objectives: &HashMap<&String, &Objective>) {
    igBegin(c_str!("WvW").as_ptr(), &mut true, 0);
    igText(c_str!("HELLO").as_ptr());
    igButton(c_str!("gfgdfg").as_ptr(), ImVec2::new(200f32, 15f32));


    let mut pos = ImVec2::zero();
    igGetCursorPos(&mut pos);
    //println!("{}, {}", pos.x, pos.y);
    let mut available_area = ImVec2::zero();
    igGetContentRegionAvail(&mut available_area);

    let uv0 = ImVec2::new(0.0, 0.0);
    let uv1 = ImVec2::new(1.0, 1.0);
    let border_color = ImVec4::new(0.0, 0.0, 0.0, 0.0);

    let min_x = objective_definitions.iter()
        .map(|c| c.coord)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap()[0])
        .reduce(f32::min).unwrap_or(0.0);
    let max_x = objective_definitions.iter()
        .map(|c| c.coord)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap()[0])
        .reduce(f32::max).unwrap_or(0.0);
    let min_y = objective_definitions.iter()
        .map(|c| c.coord)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap()[1])
        .reduce(f32::min).unwrap_or(0.0);
    let max_y = objective_definitions.iter()
        .map(|c| c.coord)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap()[1])
        .reduce(f32::max).unwrap_or(0.0);

    let icon_size = ImVec2::new(32f32, 32f32);
    let map_size_x = max_x - min_x;
    let map_size_y = max_y - min_y;

    for objective_def in objective_definitions {
        let objective_live = objectives.get(&objective_def.id);
        let tint = get_owning_force_tint(&objective_live
            .map(|e| &e.owner)
            .unwrap_or(&OwningForce::NEUTRAL));

        let objective_icon = match &objective_def.type_ {
            objective_definition::Type::CAMP => Some(&icons::Icon::ObjectiveCamp),
            objective_definition::Type::TOWER => Some(&icons::Icon::ObjectiveTower),
            objective_definition::Type::KEEP => Some(&icons::Icon::ObjectiveKeep),
            objective_definition::Type::CASTLE => Some(&icons::Icon::ObjectiveCastle),
            _default => None,
        }.map(|m| icons.get(m).unwrap());

        let x = (objective_def.coord.unwrap()[0] - min_x) / map_size_x * (available_area.x - icon_size.x);
        let y = (objective_def.coord.unwrap()[1] - min_y) / map_size_y * (available_area.y - icon_size.x);

        igSetCursorPos(ImVec2::new(pos.x + x, pos.y + y));

        if objective_icon.is_some() {
            igImage(
                objective_icon.unwrap().to_imgui_id(),
                objective_icon.unwrap().size, uv0,
                uv1,
                tint,
                border_color,
            );
        }
    }

    //igSetCursorPos(ImVec2::new(pos.x + available_area.x, pos.y + available_area.y));
    igSetCursorPos(pos);
    igDummy(ImVec2::new(available_area.x, available_area.y));

    //igDummy(ImVec2::new(available_area.x, available_area.y));
    igEnd();
}