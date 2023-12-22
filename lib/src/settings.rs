use imgui_sys::{igBeginMenu, igCheckbox, igEndMenu};
use c_str_macro::c_str;

pub struct Settings {
    pub show_objectives_overlay: bool,

    pub show_current: bool,
    pub show_red: bool,
    pub show_green: bool,
    pub show_blue: bool,
    pub show_eternal: bool,

    pub show_compass: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_objectives_overlay: false,
            show_current: false,
            show_red: false,
            show_green: false,
            show_blue: false,
            show_eternal: false,
            show_compass: false,
        }
    }
}

pub unsafe fn render_options(settings: &mut Settings) {
    igCheckbox(c_str!("Overlay").as_ptr(), &mut settings.show_objectives_overlay);
    if igBeginMenu(c_str!("Border windows").as_ptr(), true) {
        igCheckbox(c_str!("Current map").as_ptr(), &mut settings.show_current);
        igCheckbox(c_str!("Eternal battlegrounds").as_ptr(), &mut settings.show_eternal);
        igCheckbox(c_str!("Red border").as_ptr(), &mut settings.show_red);
        igCheckbox(c_str!("Green border").as_ptr(), &mut settings.show_green);
        igCheckbox(c_str!("Blue border").as_ptr(), &mut settings.show_blue);
        igEndMenu();
    }
    igCheckbox(c_str!("Compass").as_ptr(), &mut settings.show_compass);
}
