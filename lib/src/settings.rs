use std::io::ErrorKind;
use imgui_sys::{igBeginMenu, igCheckbox, igEndMenu, igPopID, igPushIDInt, igSeparator, igText};
use c_str_macro::c_str;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub show_objectives_overlay: bool,

    pub show_current: bool,
    pub show_red: bool,
    pub show_green: bool,
    pub show_blue: bool,
    pub show_eternal: bool,

    pub compass: SettingsCompass,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SettingsCompass {
    pub show: bool,
    pub lock: bool,
}

pub static mut SETTINGS: Settings = Settings {
    show_objectives_overlay: false,
    show_current: false,
    show_red: false,
    show_green: false,
    show_blue: false,
    show_eternal: false,
    compass: SettingsCompass {
        show: false,
        lock: false,
    },
};

pub fn get_settings<'a>() -> &'a mut Settings {
    unsafe {
        &mut SETTINGS
    }
}

pub unsafe fn render_options() {
    let settings = get_settings();

    igPushIDInt(1);
    igText(c_str!("Compass").as_ptr());
    igCheckbox(c_str!("Show").as_ptr(), &mut settings.compass.show);
    igCheckbox(c_str!("Lock").as_ptr(), &mut settings.compass.lock);
    igPopID();

    igSeparator();

    if igBeginMenu(c_str!("Border windows").as_ptr(), true) {
        igCheckbox(c_str!("Current map").as_ptr(), &mut settings.show_current);
        igCheckbox(c_str!("Eternal battlegrounds").as_ptr(), &mut settings.show_eternal);
        igCheckbox(c_str!("Red border").as_ptr(), &mut settings.show_red);
        igCheckbox(c_str!("Green border").as_ptr(), &mut settings.show_green);
        igCheckbox(c_str!("Blue border").as_ptr(), &mut settings.show_blue);
        igEndMenu();
    }

    igSeparator();

    igPushIDInt(3);
    igText(c_str!("Overlay").as_ptr());
    igCheckbox(c_str!("Show").as_ptr(), &mut settings.show_objectives_overlay);
    igPopID();
}

pub fn read_from_file() {
    let read = std::fs::read_to_string("addons\\arcdps\\arcdps_wvw.json");

    if read.is_ok() {
        let deserializd = serde_json::from_str::<Settings>(read.unwrap().as_str());

        match deserializd {
            Ok(settings) => unsafe { SETTINGS = settings }
            Err(error) => { eprintln!("Unable to deserialize arcdps_wvw settings: {}", error) }
        }
    } else {
        let error = read.unwrap_err();
        if error.kind() != ErrorKind::NotFound {
            eprintln!("Unable to read arcdps_wvw settings: {}", error)
        }
    }
}

pub fn write_to_file() {
    let serialized = serde_json::to_string(get_settings());

    if serialized.is_err() {
        eprintln!("Unable to serialize arcdps_wvw settings: {}", serialized.unwrap_err())
    } else {
        let write_result = std::fs::write("addons\\arcdps\\arcdps_wvw.json", serialized.unwrap());
        if write_result.is_err() {
            eprintln!("Unable to write arcdps_wvw settings: {}", write_result.unwrap_err())
        }
    }
}
