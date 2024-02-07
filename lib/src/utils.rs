use mumblelink_reader::mumble_link::{MumbleLinkData, Vector3D};
use nalgebra::{Point3, Vector3};

pub fn drop_static_mut_option<T>(reference: &mut Option<T>) {
    let old_option = core::mem::replace(reference, None);
    if let Some(old) = old_option {
        drop(old);
    }
}

pub fn swap_static_mut_option<T>(reference: &mut Option<T>, new: Option<T>) {
    let old_option = core::mem::replace(reference, new);
    if let Some(old) = old_option {
        drop(old);
    }
}

pub fn is_wvw_map_id(map_id: u32) -> bool {
    match map_id {
        38 | 94 | 95 | 96 => true,
        _ => false
    }
}

const INCH_TO_METER: f32 = 0.0254;
const METER_TO_INCH: f32 = 1.0 / INCH_TO_METER;

pub fn get_mumble_link_camera_vec(ml: &MumbleLinkData) -> Vector3<f32> {
    let vec = ml.camera.front;

    Vector3::new(vec[0] * METER_TO_INCH, vec[1] * METER_TO_INCH, -vec[2] * METER_TO_INCH)
}

pub fn get_mumble_link_camera_position(ml: &MumbleLinkData) -> Point3<f32> {
    get_point_from_mumble_link(ml.camera.position)
}

pub fn get_mumble_link_avatar_position(ml: &MumbleLinkData) -> Point3<f32> {
    get_point_from_mumble_link(ml.avatar.position)
}

pub fn get_mumble_link_up() -> Vector3<f32> {
    Vector3::new(0.0, 1.0, 0.0)
}

fn get_point_from_mumble_link(p: Vector3D) -> Point3<f32> {
    Point3::new(p[0] * METER_TO_INCH, p[1] * METER_TO_INCH, -p[2] * METER_TO_INCH)
}


