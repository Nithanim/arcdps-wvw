use std::ffi::CString;
use std::time::SystemTime;
use c_str_macro::c_str;
use imgui_sys::{igBegin, igEnd, igText};
use mumblelink_reader::mumble_link::MumbleLinkData;
use time::OffsetDateTime;
use crate::data::{get_shared_data, SharedData};
use crate::is_in_loading_screen;
use crate::settings::Settings;

pub fn render_debug(settings: &mut Settings, ml: Option<&MumbleLinkData>) {
    unsafe {
        if settings.debug {
            if igBegin(c_str!("WvW Debug").as_ptr(), &mut settings.debug, 0) {
                add_game_state();
                add_player_pos(ml);
                add_data_insight(settings);
            }
            igEnd();
        }
    }
}

unsafe fn add_data_insight(settings: &Settings) {
    let world_id_str = CString::new(format!("WorldId: {}", settings.world_id)).unwrap();
    igText(world_id_str.as_ptr());

    let shared_data_age_string = match get_shared_data() {
        None => String::from("Data age: ?"),
        Some(data) => {
            let duration = data.timestamp.elapsed();
            let seconds = duration.as_secs() % 60;
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = (duration.as_secs() / 60) / 60;
            format!("Data age: {:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
        }
    };
    let age_str = CString::new(shared_data_age_string).unwrap();
    igText(age_str.as_ptr());


    let curr_time_string = CString::new(OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or(String::from("?"))).unwrap();
    igText(curr_time_string.as_ptr());
}

unsafe fn add_game_state() {
    let pos = CString::new(format!("Loading/char selection: {}", is_in_loading_screen())).unwrap();
    igText(pos.as_ptr());
}

unsafe fn add_player_pos(ml: Option<&MumbleLinkData>) {
    if ml.is_some() {
        let inch_to_meter = 0.0254;
        let meter_to_inch = 1.0 / inch_to_meter;

        let avatar_pos = ml.unwrap().avatar.position;
        let pos = CString::new(format!("Pos: {:>5.2},{:>5.2},{:>5.2}", avatar_pos[0] * meter_to_inch, avatar_pos[1] * meter_to_inch, avatar_pos[2] * meter_to_inch)).unwrap();
        igText(pos.as_ptr());
    } else {
        let pos = CString::new(format!("Pos: {:>5}  ,{:>5}  ,{:>5}  ", "?", "?", "?")).unwrap();
        igText(pos.as_ptr());
    }
}