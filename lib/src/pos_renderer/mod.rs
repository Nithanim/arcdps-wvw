use std::ops::Div;
use c_str_macro::c_str;
use imgui_sys::*;
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkDataReader, MumbleLinkReader, Position};
use nalgebra::{Isometry3, OMatrix, Perspective3, Point2, Point3, Vector3, Vector4};
use nalgebra as na;
use crate::mumble::{GuildwarsContext, MumbleLinkIdentity};
use crate::MUMBLE_LINK;

const WINDOW_FLAGS: ImGuiWindowFlags = (ImGuiWindowFlags_NoBackground
    | ImGuiWindowFlags_NoInputs
    | ImGuiWindowFlags_NoNav
    | ImGuiWindowFlags_NoDecoration) as ImGuiWindowFlags;

pub unsafe fn render() {
    igSetNextWindowPos(ImVec2::new(0f32, 0f32), 0, ImVec2::zero());
    let mut v: ImVec2 = ImVec2::new(1920.0, 1080.0);
    igSetNextWindowSize(v, 0);

    igBegin(c_str!("Full").as_ptr(), &mut true, WINDOW_FLAGS as ImGuiWindowFlags);

    // position: [478.8522, 41.259987, -805.55994]


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
    let window_w = 1920.0;
    let window_h = 1080.0;

    let view_projection = get_view_projection_matrix(&ml);

    let clip_space_coords: Vector4<f32> = view_projection * target.to_homogeneous();
    if clip_space_coords.z > 0.0 {
        let normalized_device_coordinates = clip_space_coords.div(clip_space_coords.w);
        let normalized_device_coordinates = Point2::new(normalized_device_coordinates.x * -1.0 /* quick fix for inverted x, fix root cause! */, normalized_device_coordinates.y);

        let screen_coords = Point2::new((normalized_device_coordinates.x + 1.0) / 2.0 * window_w, (normalized_device_coordinates.y + 1.0) / 2.0 * window_h);
        let imgui_coords = Point2::new(screen_coords.x, window_h - screen_coords.y);

        let draw_list = igGetForegroundDrawList();
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y - 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y - 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y + 100.0));
        ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y + 100.0));
        ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(ImVec4::new(1.0, 0.0, 0.0, 1.0)), true, 6.0);
    }
}

fn get_view_projection_matrix(ml: &MumbleLinkData) -> OMatrix<f32, na::U4, na::Const<4>> {
    let yfov = get_yfov(&ml);

    let view = calc_view_matrix(ml);

    let projection = Perspective3::new(16.0 / 9.0, yfov, 1.0, 1000.0);
    let view_projection = projection.as_matrix() * view.to_homogeneous();
    view_projection
}

fn get_yfov(ml: &MumbleLinkData) -> f32 {
    let yfov = if ml.identity.len() > 5 {
        serde_json::from_str::<MumbleLinkIdentity>(ml.identity.as_str()).map(|e| e.fov).unwrap_or(1.222)
    } else {
        1.222
    };
    yfov
}

fn calc_view_matrix(ml: &MumbleLinkData) -> Isometry3<f32> {
    let camera_pos: Point3<f32> = Point3::new(ml.camera.position[0], ml.camera.position[1], ml.camera.position[2]);
    let camera_vec: Vector3<f32> = Vector3::new(ml.camera.front[0], ml.camera.front[1], ml.camera.front[2]);
    let camera_target: Point3<f32> = camera_pos + camera_vec;

    Isometry3::look_at_rh(&camera_pos, &camera_target, &Vector3::y())
}

fn to_point(p0: &Position) -> Point3<f32> {
    let v = &p0.position;
    Point3::new(v[0], v[1], v[2])
}
