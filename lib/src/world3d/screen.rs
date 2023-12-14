use imgui_sys::ImVec2;

static mut SCREEN_SIZE: ImVec2 = ImVec2 {
    x: 1.0,
    y: 1.0,
};

pub fn get_screen_size() -> ImVec2 {
    unsafe { SCREEN_SIZE.clone() }
}

pub fn set_screen_size(screen_size: ImVec2) {
    unsafe { SCREEN_SIZE = screen_size }
}

