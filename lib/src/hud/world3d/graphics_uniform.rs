use std::ops::Div;
use imgui_sys::ImVec2;
use mumblelink_reader::mumble_link::MumbleLinkData;
use nalgebra::{Const, Isometry3, OMatrix, Perspective3, Point2, Point3, U4, Vector3, Vector4};
use crate::mumble::MumbleLinkIdentity;

pub struct GraphicsUniform {
    pub(crate) screen_size: ImVec2,
    pub(crate) view_projection: OMatrix<f32, U4, Const<4>>,
}

impl GraphicsUniform {
    pub fn render<F>(&self, world_coords: Point3<f32>, render_fn: F)
        where
            F: FnOnce(Point2<f32>) {
        let clip_space_coords: Vector4<f32> = self.view_projection * world_coords.to_homogeneous();
        if clip_space_coords.z > 0.0 {
            let normalized_device_coordinates = clip_space_coords.div(clip_space_coords.w);
            let normalized_device_coordinates = Point2::new(normalized_device_coordinates.x, normalized_device_coordinates.y);

            let screen_coords = Point2::new((normalized_device_coordinates.x + 1.0) / 2.0 * self.screen_size.x, (normalized_device_coordinates.y + 1.0) / 2.0 * self.screen_size.y);
            let imgui_coords = Point2::new(screen_coords.x, self.screen_size.y - screen_coords.y);

            render_fn(imgui_coords);
        }
    }
}

fn calc_view_matrix(ml: &MumbleLinkData) -> Isometry3<f32> {
    let camera_pos: Point3<f32> = Point3::new(ml.camera.position[0], ml.camera.position[1], -ml.camera.position[2]);
    let camera_vec: Vector3<f32> = Vector3::new(ml.camera.front[0], ml.camera.front[1], -ml.camera.front[2]);
    let camera_target: Point3<f32> = camera_pos + camera_vec;

    Isometry3::look_at_rh(&camera_pos, &camera_target, &Vector3::y())
}

pub fn get_view_projection_matrix(ml: &MumbleLinkData) -> OMatrix<f32, nalgebra::U4, nalgebra::Const<4>> {
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
