use std::collections::HashMap;
use std::ffi::{CString};
use std::fmt::Formatter;
use imgui_sys::{igBegin, igEnd, igGetBackgroundDrawList, igGetColorU32Vec4, igGetTextLineHeight, igImage, igSameLine, igSetNextWindowPos, igText, ImDrawList_PathLineTo, ImDrawList_PathStroke, ImGuiWindowFlags, ImVec2, ImVec4};
use mumblelink_reader::mumble_link::{MumbleLinkData};
use nalgebra::{distance_squared, Point2, Point3};
use time::{Duration, OffsetDateTime};
use crate::{api, icons, ImGuiIcon};
use crate::api::map_api::Map;
use crate::api::matchup::Matchup;
use crate::api::objective_definition::{ContinentCoordinates, ObjectiveDefinition, Type};
use crate::data::SharedData;
use crate::hud::{screen};
use crate::hud::world3d::graphics_uniform::{get_view_projection_matrix, GraphicsUniform};
use crate::hud::world3d::helpers::get_current_map_id;
use crate::settings::Settings;
use crate::utils::get_mumble_link_avatar_position;

const WINDOW_FLAGS: ImGuiWindowFlags = (
    imgui_sys::ImGuiWindowFlags_NoInputs
        | imgui_sys::ImGuiWindowFlags_NoNav
        | imgui_sys::ImGuiWindowFlags_NoDecoration
        | imgui_sys::ImGuiWindowFlags_NoSavedSettings
        | imgui_sys::ImGuiWindowFlags_NoFocusOnAppearing
        | imgui_sys::ImGuiWindowFlags_AlwaysAutoResize
        | imgui_sys::ImGuiWindowFlags_NoBringToFrontOnFocus) as ImGuiWindowFlags;

pub fn render_overlay(settings: &Settings, ml: &MumbleLinkData, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, objective_definitions: &Vec<ObjectiveDefinition>) {
    let screen_size = screen::get_screen_size();
    let view_projection = get_view_projection_matrix(&ml);

    if settings.overlay.show {
        let gu = GraphicsUniform {
            screen_size,
            view_projection,
        };

        //let target = Point3::new(-62.843487, 23.980408, 229.41426);

        let current_map_id = get_current_map_id(ml);

        if let Some(shared_data) = shared_data {
            if let Ok(matchup) = &shared_data.matchup {
                let map = shared_data.maps.as_ref().unwrap().iter().find(|e| e.id == current_map_id);
                if let Some(map) = map {
                    render_objectives(gu, current_map_id, icons, matchup, objective_definitions, map, get_mumble_link_avatar_position(ml), settings);
                }
            }
        }
        if shared_data.is_some() && shared_data.unwrap().matchup.is_ok() {}

        /*
        positions::RECORDS.iter().for_each(|e| {
            if current_map_id == e.map_id {
                let target = Point3::new(e.x, e.y, e.z);

                gu.render(target, |imgui_coords| {
                    render_quad(imgui_coords);
                })
            }
        });
        */
    }
}

fn render_objectives(gu: GraphicsUniform, current_map_id: u32, icons: &HashMap<icons::Icon, ImGuiIcon>, matchup: &Matchup, objective_definitions: &Vec<ObjectiveDefinition>, map: &api::map_api::Map, avatar: Point3<f32>, settings: &Settings) {
    let current = OffsetDateTime::now_utc();


    (&matchup.maps).iter().for_each(|world| {
        if world.id == current_map_id as i32 {
            let objectives = &world.objectives;
            objectives.iter().for_each(|obj| {
                let objective_definition = get_objective_definition(&obj.id, objective_definitions);

                if let Some(objective_definition) = objective_definition {
                    if let Some(continent_coords) = objective_definition.coord {
                        if !matches!(objective_definition.type_, Type::CAMP | Type::TOWER | Type::KEEP | Type::CASTLE) {
                            return;
                        }

                        let map_coordinates = continent_to_map_coordinates(map, continent_coords);


                        let target = Point3::new(map_coordinates[0], avatar.y, map_coordinates[1]);

                        let distance_in_game = distance_squared(&Point2::new(target.x, target.z), &Point2::new(avatar.x, avatar.z));
                        let distance_bay_to_spawn_camp = 13083.0;
                        let distance_max = (distance_bay_to_spawn_camp * settings.overlay.distance_max).powi(2);
                        if distance_in_game > distance_max {
                            // Don't render if far away
                            return;
                        } else if distance_in_game < 1000.0_f32.powi(2) {
                            return;
                        }

                        // TODO maybe allow rendering not on avatar height, but rather based on looking vector
                        // e.g. you generally look down on char, so offset pos below avatar height

                        gu.render(target, |imgui_coords| {
                            unsafe {
                                {
                                    igSetNextWindowPos(ImVec2::new(imgui_coords.x, imgui_coords.y), 0, ImVec2::new(0.5, 0.5));
                                    let name = CString::new(format!("{}##WvWExt{}", objective_definition.name, obj.id)).unwrap();
                                    igBegin(name.as_ptr(), &mut true, WINDOW_FLAGS as ImGuiWindowFlags);
                                    // Using the default window title bar is results in truncated titles.
                                    // Maybe there is a fix for that?
                                    // Also, the close button is shown, which shouldn't be.
                                }

                                {
                                    let name = CString::new(objective_definition.name.as_str()).unwrap();
                                    igText(name.as_ptr());
                                }

                                let mut line = 1;
                                let line_height = igGetTextLineHeight();

                                let buff_remaining = calculate_buff_remaining(obj.last_flipped, current);
                                if buff_remaining
                                    .map(|diff| diff < Duration::minutes(5))
                                    .unwrap_or(false) {
                                    let icon = icons.get(&icons::Icon::BuffRighteousIndignation).unwrap();
                                    igImage(
                                        icon.to_imgui_id(),
                                        ImVec2::new(line_height, line_height),
                                        ImVec2::new(0.0, 0.0),
                                        ImVec2::new(1.0, 1.0),
                                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                                        ImVec4::new(0.0, 0.0, 0.0, 0.0),
                                    );
                                    igSameLine(0.0, -1.0);

                                    let string = format_time_left(buff_remaining.unwrap());

                                    let name = CString::new(string).unwrap();
                                    igText(name.as_ptr());
                                    line += 1;
                                }

                                if settings.debug {
                                    let coordinates_string = CString::new(format!("Coords: {},{}", map_coordinates[0], map_coordinates[1])).unwrap();
                                    igText(coordinates_string.as_ptr());
                                    line += 1;


                                    let time = obj.last_flipped.map(|e| e.format(&time::format_description::well_known::Rfc3339).unwrap()).unwrap_or(String::from("-"));
                                    let coordinates_string = CString::new(format!("Last flipped: {}", time)).unwrap();
                                    igText(coordinates_string.as_ptr());
                                    line += 1;
                                }
                                igEnd();
                            }

                            if settings.debug {
                                // render_quad(imgui_coords);
                            }
                        })
                    }
                }
                //}

                //}
            })
        }
    });
}

fn calculate_buff_remaining(last_flipped: Option<OffsetDateTime>, now: OffsetDateTime) -> Option<Duration> {
    match last_flipped {
        None => None,
        Some(last_flipped) => {
            let diff = now - last_flipped;
            if diff < Duration::minutes(5) {
                Some(Duration::minutes(5) - diff)
            } else {
                None
            }
        }
    }
}

fn format_time_left(diff: Duration) -> String {
    let mut string = String::new();
    let minutes = diff.whole_minutes();
    let seconds = diff.whole_seconds();
    if seconds >= 60 {
        string.push(('0' as u8 + minutes as u8) as char);
        string.push('m');
        string.push(' ');
    }
    if seconds > 0 || minutes > 0 {
        let seconds_in_minute = seconds - minutes * 60;
        if seconds_in_minute > 10 {
            string.push(('0' as u8 + (seconds_in_minute / 10) as u8) as char);
        } else {
            string.push(' ');
        }
        string.push(('0' as u8 + (seconds_in_minute % 10) as u8) as char);
        string.push('s');
    }
    format!("{:>7}", string)
}

fn continent_to_map_coordinates(map: &Map, continent_coords: ContinentCoordinates) -> Point2<f32> {
    // The coordinates of objectives are given as continent coordinates.
    // (Not internal to the map but rather the whole map you see ingame.)

    // z (coord 2) is up in continent coords.
    // y (coord 1) is up in mumble link space.

    // First step is to convert the absolute coordinates to a percentage in the map rect.
    // (As in x and y as values between 0 and 1).
    let normalized_coordinates = Point2::new(
        (continent_coords[0] - map.continent_rect[0].0 as f32) / (map.continent_rect[1].0 as f32 - map.continent_rect[0].0 as f32),
        (continent_coords[1] - map.continent_rect[0].1 as f32) / (map.continent_rect[1].1 as f32 - map.continent_rect[0].1 as f32),
    );

    // Then we apply that to the dimensions of the world coordinate rectangle.
    // This gives us the coordinates in the context of the map.
    let map_coordinates = Point2::new(
        normalized_coordinates.x * (map.map_rect[1].0 - map.map_rect[0].0) as f32 + map.map_rect[0].0 as f32,
        normalized_coordinates.y * (map.map_rect[1].1 - map.map_rect[0].1) as f32 + map.map_rect[0].1 as f32,
    );
    // For some reason, the y coordinate is flipped in contrast to mumble link.
    // I mean, plotting the conversion everything lines up like you would expect from top left (-, -) to bottom right (+, +).
    // But the mumble link data is flipped, not sure why.

    map_coordinates
}

fn get_objective_definition<'a>(id: &String, objective_definitions: &'a Vec<ObjectiveDefinition>) -> Option<&'a ObjectiveDefinition> {
    objective_definitions.iter().find(|o| o.id.eq(id))
}

fn render_quad(imgui_coords: Point2<f32>) {
    unsafe {
        let draw_list = igGetBackgroundDrawList();
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y - 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y - 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y + 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y + 100.0));
        ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(ImVec4::new(1.0, 0.0, 0.0, 1.0)), true, 6.0);
    }
}

