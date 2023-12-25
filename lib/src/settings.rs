use imgui_sys::{igBeginMenu, igCheckbox, igEndMenu, igPopID, igPushIDInt, igSeparator, igText};
use c_str_macro::c_str;

pub struct Settings {
    pub show_objectives_overlay: bool,

    pub show_current: bool,
    pub show_red: bool,
    pub show_green: bool,
    pub show_blue: bool,
    pub show_eternal: bool,

    pub compass: SettingsCompass,
}

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
