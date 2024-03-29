use std::f32::consts::PI;
use c_str_macro::c_str;
use imgui_sys::{igBegin, igDummy, igEnd, igGetColorU32Vec4, igGetCursorPos, igGetCursorScreenPos, igGetWindowDrawList, igGetWindowSize, igPopStyleVar, igPushStyleVarVec2, ImDrawList, ImDrawList_PathFillConvex, ImDrawList_PathLineTo, ImDrawList_PathStroke, ImGuiStyleVar, ImGuiStyleVar_FramePadding, ImGuiStyleVar_WindowPadding, ImGuiWindowFlags, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoDecoration, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoNav, ImVec2, ImVec4};
use mumblelink_reader::mumble_link::MumbleLinkData;
use nalgebra::{Isometry2, Matrix3, min, Point2, Point3, Rotation2, Vector2};
use once_cell::sync::Lazy;
use crate::hud::world3d::helpers::get_current_map_id;
use crate::is_in_loading_screen;
use crate::settings::{Settings, SettingsCompass};
use crate::utils::is_wvw_map_id;

struct Colors {
    primary: ImVec4,
    secondary: ImVec4,
}

pub unsafe fn render2d(ml: &MumbleLinkData, settings: &mut Settings) {
    if should_render_compass(ml, settings) {
        let camera = Vector2::new(ml.camera.front[0], ml.camera.front[2]);
        render2d_internal(settings, camera);
    }
}

fn should_render_compass(ml: &MumbleLinkData, settings: &Settings) -> bool {
    if !settings.compass.show {
        return false;
    } else if is_in_loading_screen() {
        return false;
    } else if settings.compass.only_in_wvw {
        return is_wvw_map_id(get_current_map_id(ml));
    } else {
        true
    }
}

pub unsafe fn render2d_dummy(settings: &mut Settings) {
    if settings.compass.show {
        static mut CAMERA_VEC: Vector2<f32> = Vector2::new(0.0, 1.0);

        static ROT: Lazy<Rotation2<f32>> = Lazy::new(|| Rotation2::new(PI * 2.0 / 365.0));

        let r: &Rotation2<f32> = &ROT;
        let c = r * CAMERA_VEC;
        CAMERA_VEC = c;
        render2d_internal(settings, c);
    }
}

const TRANSPARENT_WINDOW_FLAGS: ImGuiWindowFlags = (ImGuiWindowFlags_NoBackground
    | ImGuiWindowFlags_NoInputs
    | ImGuiWindowFlags_NoNav
    | ImGuiWindowFlags_NoDecoration) as ImGuiWindowFlags;

pub unsafe fn render2d_internal(settings: &mut Settings, direction_camera: Vector2<f32>) {
    igPushStyleVarVec2(ImGuiStyleVar_WindowPadding as ImGuiStyleVar, ImVec2::new(0.0, 0.0));
    igPushStyleVarVec2(ImGuiStyleVar_FramePadding as ImGuiStyleVar, ImVec2::new(0.0, 0.0));

    let window_flags: ImGuiWindowFlags = match settings.compass.lock {
        true => TRANSPARENT_WINDOW_FLAGS,
        false => 0,
    };
    if igBegin(c_str!("Compass").as_ptr(), &mut settings.compass.show, window_flags) {
        draw_compass(&settings.compass, direction_camera);
    }
    igEnd();

    igPopStyleVar(2);
}

unsafe fn draw_compass(settings: &SettingsCompass, direction_camera: Vector2<f32>) {
    let mut window_origin = ImVec2::zero();
    igGetCursorScreenPos(&mut window_origin);

    let mut draw_area_start = ImVec2::zero();
    igGetCursorPos(&mut draw_area_start);

    igDummy(ImVec2::new(100.0, 100.0));

    let mut window_size = ImVec2::zero();
    igGetWindowSize(&mut window_size);

    let draw_area_size = ImVec2::new(window_size.x - draw_area_start.x, window_size.y - draw_area_start.y);

    let draw_area_center = Vector2::new(window_origin.x + draw_area_size.x / 2.0, window_origin.y + draw_area_size.y / 2.0);

    // Coordinate system:
    // imgui: [0,0] is top left
    // gw2: [0,0] is center; [0,1] is north

    let length_pin_north = f32::max(f32::min(draw_area_size.x, draw_area_size.y) / 2.0 - 10.0, 0.0);


    // The map center is [0,0].
    // The north is down the y direction.
    // Therefore, the angle of the camera vec is 1/4 of a circle.
    // (Since the angle starts from [+x,0] to [0,+y] to [-x,0] to finally [0,-y])
    let north_offset = PI / 2.0;

    let angle = normalize_angle(get_angle(direction_camera) - north_offset);
    let base_matrix = Isometry2::new(draw_area_center, angle);
    let imgui_invert = Matrix3::new_nonuniform_scaling(&Vector2::new(1.0, -1.0)); // imgui y axis is opposite, so flip around y

    let colors = Colors {
        primary: ImVec4::new(settings.color_primary[0], settings.color_primary[1], settings.color_primary[2], settings.opacity),
        secondary: ImVec4::new(settings.color_secondary[0], settings.color_secondary[1], settings.color_secondary[2], settings.opacity),
    };

    let draw_list = igGetWindowDrawList();

    let matrix: Matrix3<f32> = base_matrix.to_matrix() * Matrix3::new_scaling(length_pin_north) * imgui_invert;
    render_pin(&matrix, 1.0, &colors, draw_list);

    let matrix: Matrix3<f32> = base_matrix.to_matrix() * Rotation2::new(PI / 2.0).to_homogeneous() * Matrix3::new_scaling(length_pin_north) * imgui_invert;
    render_pin(&matrix, 0.6, &colors, draw_list);

    let matrix: Matrix3<f32> = base_matrix.to_matrix() * Rotation2::new(PI / 2.0 * 2.0).to_homogeneous() * Matrix3::new_scaling(length_pin_north) * imgui_invert;
    render_pin(&matrix, 0.6, &colors, draw_list);

    let matrix: Matrix3<f32> = base_matrix.to_matrix() * Rotation2::new(PI / 2.0 * 3.0).to_homogeneous() * Matrix3::new_scaling(length_pin_north) * imgui_invert;
    render_pin(&matrix, 0.6, &colors, draw_list);
}

unsafe fn render_pin(matrix: &Matrix3<f32>, pin_size: f32, colors: &Colors, draw_list: *mut ImDrawList) {
    let compass_center = Point2::new(0.0, 0.0);
    let compass_north = Point2::new(0.0, pin_size);
    let w = 0.15;
    let compass_north_left = Point2::new(-w, w);
    let compass_north_right = Point2::new(w, w);

    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north_left));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_center));
    ImDrawList_PathFillConvex(draw_list, igGetColorU32Vec4(colors.primary));

    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_center));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north_right));
    ImDrawList_PathFillConvex(draw_list, igGetColorU32Vec4(colors.secondary));

    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north_right));
    ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(colors.primary), false, 1.0);
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north_left));
    ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(colors.secondary), false, 1.0);
}

fn get_angle(vec: Vector2<f32>) -> f32 {
    // Normally,
    // vec.y.atan2(vec.x)
    // results in 0 to pi around the y=1 axis
    // and 0 to -pi around the y=-1 axis
    // instead of 0 to 2pi.
    let base = vec.y.atan2(vec.x);

    // So we shift the negative half into the positive
    // by adding a full circle.
    // Then, we normalize to one circle.
    normalize_angle(base)
}

fn normalize_angle(base: f32) -> f32 {
    (base + 2.0 * PI) % (2.0 * PI)
}

fn to_imgui(matrix: &Matrix3<f32>, point: &Point2<f32>) -> ImVec2 {
    let point = matrix * Point3::new(point.x, point.y, 1.0);
    ImVec2::new(point.x, point.y)
}


