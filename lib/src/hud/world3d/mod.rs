mod helpers;
pub mod positions;

use std::ffi::{CString};
use std::ops::Div;
use c_str_macro::c_str;
use imgui_sys::{igBegin, igBeginChildFrame, igBeginChildID, igBeginChildStr, igEnd, igGetBackgroundDrawList, igGetColorU32Vec4, igGetTextLineHeight, igSetCursorScreenPos, igSetNextWindowPos, igSetNextWindowSize, igText, ImDrawList_PathLineTo, ImDrawList_PathStroke, ImGuiID, ImGuiWindowFlags, ImVec2, ImVec4};
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkDataReader, MumbleLinkReader};
use nalgebra::{Const, OMatrix, Point2, Point3, U4, Vector4};
use time::{Duration, OffsetDateTime};
use tokio::time::Instant;
use crate::api;
use crate::api::matchup::Matchup;
use crate::api::objective_definition::ObjectiveDefinition;
use crate::data::SharedData;
use crate::mumble::GuildwarsContext;
use crate::hud::{screen};
use crate::settings::Settings;

const WINDOW_FLAGS: ImGuiWindowFlags = (
    imgui_sys::ImGuiWindowFlags_NoBackground
        | imgui_sys::ImGuiWindowFlags_NoInputs
        | imgui_sys::ImGuiWindowFlags_NoNav
        | imgui_sys::ImGuiWindowFlags_NoDecoration
        | imgui_sys::ImGuiWindowFlags_NoSavedSettings
        | imgui_sys::ImGuiWindowFlags_NoFocusOnAppearing
        | imgui_sys::ImGuiWindowFlags_NoBringToFrontOnFocus) as ImGuiWindowFlags;

pub unsafe fn render_hud(settings: &Settings, ml: &MumbleLinkData, shared_data: Option<&SharedData>, objective_defintions: &Vec<ObjectiveDefinition>) {
    igSetNextWindowPos(ImVec2::new(0f32, 0f32), 0, ImVec2::zero());
    igSetNextWindowSize(screen::get_screen_size(), 0);

    igBegin(c_str!("3d World Overlay").as_ptr(), &mut true, WINDOW_FLAGS as ImGuiWindowFlags);
    render_hud_(settings, ml, shared_data, objective_defintions);

    //up is (0, 1, 0)

    igEnd();
}

pub fn render_hud_(settings: &Settings, ml: &MumbleLinkData, shared_data: Option<&SharedData>, objective_definitions: &Vec<ObjectiveDefinition>) {
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
                    render_objectives(gu, current_map_id, matchup, objective_definitions, y, map);
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

fn render_objectives(gu: GraphicsUniform, current_map_id: u32, matchup: &Matchup, objective_definitions: &Vec<ObjectiveDefinition>, y: f32, map: &api::map_api::Map) {
    let current = OffsetDateTime::now_utc();


    (&matchup.maps).iter().for_each(|world| {
        if world.id == current_map_id as i32 {
            let objectives = &world.objectives;
            objectives.iter().for_each(|obj| {
                //if let Some(last_flipped) = obj.last_flipped {
                //let diff = current - last_flipped;
                //if diff < Duration::minutes(5) {

                let objective_definition = get_objective_definition(&obj.id, objective_definitions);

                if let Some(objective_definition) = objective_definition {
                    let inch_to_meter = 0.0254;

                    if !((objective_definition.map_id == 94
                        || objective_definition.map_id == 95
                        || objective_definition.map_id == 96) &&
                        (objective_definition.type_ == api::objective_definition::Type::KEEP))
                    {
                        return;
                    }


                    if let Some(continent_coords) = objective_definition.coord {
                        // The coordinates of objectives are given as continent coordinates.
                        // (Not internal to the map but rather the whole map you see ingame.)

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


                        let target = Point3::new(map_coordinates[0] * inch_to_meter, y, -map_coordinates[1] * inch_to_meter);

                        gu.render(target, |imgui_coords| {
                            unsafe {
                                let pos = ImVec2::new(imgui_coords.x, imgui_coords.y);
                                igSetCursorScreenPos(pos);
                                //igBeginChildID(ImGuiID::from_f32(f32::abs(imgui_coords.x) * f32::abs(imgui_coords.y)), );
                                let name = CString::new(objective_definition.name.as_str()).unwrap();
                                igText(name.as_ptr());
                                let pos = ImVec2::new(pos.x, pos.y + igGetTextLineHeight());
                                // Not sure why this is needed here. But without it, positioning is completely wrong.
                                //igSetCursorScreenPos(pos);
                                let pos = CString::new(format!("{},{}", map_coordinates[0], map_coordinates[1])).unwrap();
                                igText(pos.as_ptr());
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

pub(crate) fn get_current_map_id(ml: &MumbleLinkData) -> u32 {
    let gw2context = ml.read_context_into_struct::<GuildwarsContext>();
    return gw2context.map_id;
}

struct GraphicsUniform {
    screen_size: ImVec2,
    view_projection: OMatrix<f32, U4, Const<4>>,
}

impl GraphicsUniform {
    fn render<F>(&self, world_coords: Point3<f32>, render_fn: F)
        where
            F: FnOnce(Point2<f32>) {
        let clip_space_coords: Vector4<f32> = self.view_projection * world_coords.to_homogeneous();
        if clip_space_coords.z > 0.0 {
            let normalized_device_coordinates = clip_space_coords.div(clip_space_coords.w);
            let normalized_device_coordinates = Point2::new(normalized_device_coordinates.x * -1.0 /* quick fix for inverted x, fix root cause! */, normalized_device_coordinates.y);

            let screen_coords = Point2::new((normalized_device_coordinates.x + 1.0) / 2.0 * self.screen_size.x, (normalized_device_coordinates.y + 1.0) / 2.0 * self.screen_size.y);
            let imgui_coords = Point2::new(screen_coords.x, self.screen_size.y - screen_coords.y);

            render_fn(imgui_coords);
        }
    }
}
