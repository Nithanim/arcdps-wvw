use c_str_macro::c_str;
use imgui_sys::{igBeginMenu, igCheckbox, igEndMenu};
use crate::settings::Settings;

pub unsafe fn render_options(settings: &mut Settings) {
    if igBeginMenu(c_str!("Border windows").as_ptr(), true) {
        igCheckbox(c_str!("Current map").as_ptr(), &mut settings.show_current);
        igCheckbox(c_str!("Eternal battlegrounds").as_ptr(), &mut settings.show_eternal);
        igCheckbox(c_str!("Red border").as_ptr(), &mut settings.show_red);
        igCheckbox(c_str!("Green border").as_ptr(), &mut settings.show_green);
        igCheckbox(c_str!("Blue border").as_ptr(), &mut settings.show_blue);
        igEndMenu();
    }
}
