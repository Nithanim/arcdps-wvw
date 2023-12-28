mod world_id;

use std::io::ErrorKind;
use imgui_sys::{igBeginMenu, igButton, igCheckbox, igColorEdit3, igEndMenu, igInputInt, igIsItemHovered, igPopID, igPushIDInt, igSeparator, igSetTooltip, igSliderFloat, igText, ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenDisabled, ImGuiInputTextFlags, ImGuiInputTextFlags_, ImGuiInputTextFlags_ReadOnly, ImVec2};
use c_str_macro::c_str;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub world_id: i32,
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
    pub color_primary: [f32; 3],
    pub color_secondary: [f32; 3],
    pub opacity: f32,
}

pub static mut SETTINGS: Settings = Settings {
    world_id: 0,
    show_objectives_overlay: false,
    show_current: false,
    show_red: false,
    show_green: false,
    show_blue: false,
    show_eternal: false,
    compass: SettingsCompass {
        show: false,
        lock: false,
        color_primary: [1.0, 1.0, 1.0],
        color_secondary: [0.0, 0.0, 0.0],
        opacity: 1.0,
    },
};

pub fn get_settings<'a>() -> &'a mut Settings {
    unsafe {
        &mut SETTINGS
    }
}

pub unsafe fn render_options() {
    let settings = get_settings();

    igPushIDInt(3);
    igText(c_str!("General").as_ptr());
    world_id::render_options_world_id(settings);
    igPopID();

    igSeparator();

    igPushIDInt(1);
    igText(c_str!("Compass").as_ptr());
    igCheckbox(c_str!("Show").as_ptr(), &mut settings.compass.show);
    igCheckbox(c_str!("Lock").as_ptr(), &mut settings.compass.lock);
    igColorEdit3(c_str!("Primary color").as_ptr(), settings.compass.color_primary.as_mut_ptr(), 0);
    igColorEdit3(c_str!("Secondary color").as_ptr(), settings.compass.color_secondary.as_mut_ptr(), 0);
    igSliderFloat(c_str!("Opacity").as_ptr(), &mut settings.compass.opacity, 0.0, 1.0, c_str!( "%.3f").as_ptr(), 0);
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
