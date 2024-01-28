use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkDataReader, Position};
use nalgebra::Point3;
use crate::mumble::{GuildwarsContext, MumbleLinkIdentity};


fn to_point(p0: &Position) -> Point3<f32> {
    let v = &p0.position;
    Point3::new(v[0], v[1], v[2])
}


pub(crate) fn get_current_map_id(ml: &MumbleLinkData) -> u32 {
    let gw2context = ml.read_context_into_struct::<GuildwarsContext>();
    return gw2context.map_id;
}

