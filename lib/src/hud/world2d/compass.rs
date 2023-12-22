use std::f32::consts::PI;
use c_str_macro::c_str;
use imgui_sys::{igBegin, igDummy, igEnd, igGetColorU32Vec4, igGetCursorScreenPos, igGetWindowDrawList, igGetWindowSize, igPopStyleVar, igPushStyleVarVec2, ImDrawList_PathLineTo, ImDrawList_PathStroke, ImGuiStyleVar, ImGuiStyleVar_FramePadding, ImGuiStyleVar_WindowPadding, ImVec2, ImVec4};
use mumblelink_reader::mumble_link::MumbleLinkData;
use nalgebra::{Isometry2, Matrix3, Point2, Point3, Rotation2, Vector2};
use once_cell::sync::Lazy;
use crate::settings::Settings;

pub unsafe fn render2d(ml: &MumbleLinkData, settings: &mut Settings) {
    let camera = Vector2::new(ml.camera.front[0], ml.camera.front[2]);
    println!("LookDir: [{}, {}]", camera.x, camera.y);
    render2d_internal(settings, camera);
}

pub unsafe fn render2d_dummy(settings: &mut Settings) {
    static mut CAMERA_VEC: Vector2<f32> = Vector2::new(0.0, 1.0);

    static ROT: Lazy<Rotation2<f32>> = Lazy::new(|| Rotation2::new(PI * 2.0 / 365.0));

    let r: &Rotation2<f32> = &ROT;
    let c = r * CAMERA_VEC;
    CAMERA_VEC = c;
    render2d_internal(settings, c);
}

pub unsafe fn render2d_internal(settings: &mut Settings, direction_camera: Vector2<f32>) {
    if settings.show_compass {
        igPushStyleVarVec2(ImGuiStyleVar_WindowPadding as ImGuiStyleVar, ImVec2::new(0.0, 0.0));
        igPushStyleVarVec2(ImGuiStyleVar_FramePadding as ImGuiStyleVar, ImVec2::new(0.0, 0.0));

        if igBegin(c_str!("Compass").as_ptr(), &mut settings.show_compass, 0) {
            draw_compass(direction_camera);
        }
        igEnd();

        igPopStyleVar(2);
    }
}

unsafe fn draw_compass(direction_camera: Vector2<f32>) {
    let mut window_origin = ImVec2::zero();
    igGetCursorScreenPos(&mut window_origin);

    igDummy(ImVec2::new(200.0, 200.0));

    let mut window_size = ImVec2::zero();
    igGetWindowSize(&mut window_size);

    let window_center = Vector2::new(window_origin.x + window_size.x / 2.0, window_origin.y + window_size.y / 2.0);

    // Coordinate system:
    // imgui: [0,0] is top left
    // gw2: [0,0] is center; [0,1] is north


    // The map center is [0,0].
    // The north is down the y direction.
    // Therefore, the angle of the camera vec is 1/4 of a circle.
    // (Since the angle starts from [+x,0] to [0,+y] to [-x,0] to finally [0,-y])
    let north_offset = PI / 2.0;

    let angle = normalize_angle(get_angle(direction_camera) - north_offset);
    let matrix = Isometry2::new(window_center, angle);


    let imgui_invert = Matrix3::new_nonuniform_scaling(&Vector2::new(1.0, -1.0)); // imgui y axis is opposite, so flip around y
    let matrix: Matrix3<f32> = matrix.to_matrix() * imgui_invert;


    let compass_center = Point2::new(0.0, 0.0);
    let compass_north = Point2::new(0.0, 50.0);
    let compass_north_left = Point2::new(-10.0, 0.0);
    let compass_north_right = Point2::new(10.0, 0.0);


    let draw_list = igGetWindowDrawList();
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_center));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north_left));
    ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(ImVec4::new(0.0, 0.0, 1.0, 1.0)), false, 6.0);
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_center));
    ImDrawList_PathLineTo(draw_list, to_imgui(&matrix, &compass_north_right));
    ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(ImVec4::new(0.0, 0.0, 1.0, 1.0)), false, 6.0);
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


