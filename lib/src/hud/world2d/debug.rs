use std::ffi::CString;
use c_str_macro::c_str;
use imgui_sys::{igBegin, igEnd, igText};
use mumblelink_reader::mumble_link::MumbleLinkData;
use crate::settings::Settings;

pub fn render_debug(settings: &mut Settings, ml: &MumbleLinkData) {
    unsafe {
        if settings.debug {
            if igBegin(c_str!("WvW Debug").as_ptr(), &mut settings.debug, 0) {
                let inch_to_meter = 0.0254;
                let meter_to_inch = 1.0 / inch_to_meter;

                let avatar_pos = ml.avatar.position;
                let pos = CString::new(format!("Pos: {:>5.2},{:>5.2},{:>5.2}", avatar_pos[0] * meter_to_inch, avatar_pos[1] * meter_to_inch, avatar_pos[2] * meter_to_inch)).unwrap();
                igText(pos.as_ptr());
            }
            igEnd();
        }
    }
}