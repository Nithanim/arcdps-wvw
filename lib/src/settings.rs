pub struct Settings {
    pub show_current: bool,
    pub show_red: bool,
    pub show_green: bool,
    pub show_blue: bool,
    pub show_eternal: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_current: false,
            show_red: false,
            show_green: false,
            show_blue: false,
            show_eternal: false,
        }
    }
}