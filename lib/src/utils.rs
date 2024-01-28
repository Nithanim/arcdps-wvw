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

pub fn get_mumble_link_camera_vec(ml: &MumbleLinkData) -> Vector3<f32> {
    let point = mumble_link_to_map_coordinates(ml.camera.front);
    Vector3::new(point.x, point.y, point.z)
}

pub fn get_mumble_link_avatar_position(ml: &MumbleLinkData) -> Point3<f32> {
    mumble_link_to_map_coordinates(ml.avatar.position)
}

pub fn mumble_link_to_map_coordinates(vec: Vector3D) -> Point3<f32> {
    let inch_to_meter = 0.0254;
    let meter_to_inch = 1.0 / inch_to_meter;

    Point3::new(vec[0] * meter_to_inch, vec[1] * meter_to_inch, -vec[2] * meter_to_inch)
}

