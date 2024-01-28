use std::collections::HashMap;
use std::ffi::{CString};
use imgui_sys::{igBeginChildFrame, igBeginChildID, igBeginChildStr, igGetBackgroundDrawList, igGetColorU32Vec4, igGetTextLineHeight, igGetTextLineHeightWithSpacing, igImage, igSameLine, igSetCursorScreenPos, igText, ImDrawList_PathLineTo, ImDrawList_PathStroke, ImVec2, ImVec4};
use mumblelink_reader::mumble_link::{MumbleLinkData, Vector3D};
use nalgebra::{distance_squared, Point2, Point3};
use time::{Duration, OffsetDateTime};
use crate::{api, icons, ImGuiIcon};
use crate::api::map_api::Map;
use crate::api::matchup::Matchup;
use crate::api::objective_definition::{ContinentCoordinates, ObjectiveDefinition};
use crate::data::SharedData;
use crate::hud::{screen};
use crate::hud::world3d::graphics_uniform::GraphicsUniform;
use crate::hud::world3d::helpers;
use crate::hud::world3d::helpers::get_current_map_id;
use crate::settings::Settings;

pub fn render_overlay(settings: &Settings, ml: &MumbleLinkData, icons: &HashMap<icons::Icon, ImGuiIcon>, shared_data: Option<&SharedData>, objective_definitions: &Vec<ObjectiveDefinition>) {
    let screen_size = screen::get_screen_size();
    let view_projection = helpers::get_view_projection_matrix(&ml);

    if settings.show_objectives_overlay {
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
                    let y = ml.avatar.position[1];
                    render_objectives(gu, current_map_id, icons, matchup, objective_definitions, y, map, ml.avatar.position);
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

fn render_objectives(gu: GraphicsUniform, current_map_id: u32, icons: &HashMap<icons::Icon, ImGuiIcon>, matchup: &Matchup, objective_definitions: &Vec<ObjectiveDefinition>, y: f32, map: &api::map_api::Map, avatar: Vector3D) {
    let current = OffsetDateTime::now_utc();


    (&matchup.maps).iter().for_each(|world| {
        if world.id == current_map_id as i32 {
            let objectives = &world.objectives;
            objectives.iter().for_each(|obj| {
                let objective_definition = get_objective_definition(&obj.id, objective_definitions);

                if let Some(objective_definition) = objective_definition {
                    let inch_to_meter = 0.0254;

                    if let Some(continent_coords) = objective_definition.coord {
                        let map_coordinates = continent_to_map_coordinates(map, continent_coords);


                        let target = Point3::new(map_coordinates[0] * inch_to_meter, y, -map_coordinates[1] * inch_to_meter);

                        let distance_squared = distance_squared(&Point2::new(target.x, target.z), &Point2::new(avatar[0], avatar[2]));
                        let distance_bay_to_spawn_camp_squared = 171185460.0;
                        if distance_squared > distance_bay_to_spawn_camp_squared {
                            // Don't render if far away
                            return;
                        }

                        gu.render(target, |imgui_coords| {
                            unsafe {
                                {
                                    igSetCursorScreenPos(ImVec2::new(imgui_coords.x, imgui_coords.y));
                                    //igBeginChildID(ImGuiID::from_f32(f32::abs(imgui_coords.x) * f32::abs(imgui_coords.y)), );
                                    let name = CString::new(objective_definition.name.as_str()).unwrap();
                                    igText(name.as_ptr());
                                }

                                let mut line = 1;
                                let line_height = igGetTextLineHeight();
                                let line_height_with_spacing = igGetTextLineHeightWithSpacing();

                                let buff_remaining = calculate_buff_remaining(obj.last_flipped, current);
                                if buff_remaining
                                    .map(|diff| diff < Duration::minutes(5))
                                    .unwrap_or(false) {
                                    igSetCursorScreenPos(ImVec2::new(imgui_coords.x, imgui_coords.y + line_height_with_spacing * line as f32));

                                    let icon = icons.get(&crate::icons::Icon::BuffRighteousIndignation).unwrap();
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

                                {
                                    igSetCursorScreenPos(ImVec2::new(imgui_coords.x, imgui_coords.y + line_height_with_spacing * line as f32));
                                    let coordinates_string = CString::new(format!("{},{}", map_coordinates[0], -map_coordinates[1])).unwrap();
                                    igText(coordinates_string.as_ptr());
                                    line += 1;
                                }
                            }

                            render_quad(imgui_coords);
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
    }
    if seconds > 0 {
        let seconds_in_minute = seconds - minutes * 60;
        if seconds_in_minute > 10 {
            string.push(('0' as u8 + (seconds_in_minute / 10) as u8) as char);
        }
        string.push(('0' as u8 + (seconds_in_minute % 10) as u8) as char);
        string.push('s');
    }
    format!("{:>6}", string)
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

