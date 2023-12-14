use mumblelink_reader::mumble_link::{MumbleLinkData, Position};
use nalgebra::{Isometry3, OMatrix, Perspective3, Point3, Vector3};
use crate::mumble::MumbleLinkIdentity;

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
