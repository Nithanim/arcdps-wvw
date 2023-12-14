use std::ops::Div;
use c_str_macro::c_str;
use imgui_sys::*;
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkReader};
use nalgebra::{Point2, Point3, Vector4};
use crate::world3d::screen::get_screen_size;
use crate::MUMBLE_LINK;
use crate::world3d::helpers;

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

        do_magic(linked_memory);
    }

    //up is (0, 1, 0)

    igEnd();
}

unsafe fn do_magic(ml: MumbleLinkData) {
    //let gw2context = ml.read_context_into_struct::<GuildwarsContext>();
    let target = Point3::new(-62.843487, 23.980408, 229.41426);
    let screen_size = get_screen_size();

    let view_projection = helpers::get_view_projection_matrix(&ml);

    let clip_space_coords: Vector4<f32> = view_projection * target.to_homogeneous();
    if clip_space_coords.z > 0.0 {
        let normalized_device_coordinates = clip_space_coords.div(clip_space_coords.w);
        let normalized_device_coordinates = Point2::new(normalized_device_coordinates.x * -1.0 /* quick fix for inverted x, fix root cause! */, normalized_device_coordinates.y);

        let screen_coords = Point2::new((normalized_device_coordinates.x + 1.0) / 2.0 * screen_size.x, (normalized_device_coordinates.y + 1.0) / 2.0 * screen_size.y);
        let imgui_coords = Point2::new(screen_coords.x, screen_size.y - screen_coords.y);

        let draw_list = igGetForegroundDrawList();
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y - 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y - 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y + 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y + 100.0));
        ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(ImVec4::new(1.0, 0.0, 0.0, 1.0)), true, 6.0);
    }
}
