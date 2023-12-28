use std::thread;
use std::time::Duration;
use c_str_macro::c_str;
use imgui_sys::{igBeginMenu, igButton, igEndMenu, igInputInt, igIsItemHovered, igSetTooltip, igText, ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenDisabled, ImGuiInputTextFlags, ImVec2};
use once_cell::sync::OnceCell;
use reqwest::blocking::Client;
use crate::api::world_info::WorldInfo;
use crate::data::http_client::get_http_client;
use crate::settings::Settings;


static SELECTION_DATA: OnceCell<Result<Vec<WorldInfo>, String>> = OnceCell::new();

pub(crate) unsafe fn render_options_world_id(settings: &mut Settings) {
    igInputInt(c_str!("World id").as_ptr(), &mut settings.world_id, 0, 0, 0 as ImGuiInputTextFlags);
    if igIsItemHovered(ImGuiHoveredFlags_AllowWhenDisabled as ImGuiHoveredFlags) {
        igSetTooltip(c_str!("The id of the server you account is on.").as_ptr());
    }

    static mut CLICKED_LOAD_SERVER_SELECTION: bool = false;

    if CLICKED_LOAD_SERVER_SELECTION {
        render_menu(settings);
    }

    if !CLICKED_LOAD_SERVER_SELECTION {
        if render_button_load() {
            CLICKED_LOAD_SERVER_SELECTION = true;
        }
    }
}

unsafe fn render_menu(settings: &mut Settings) {
    if igBeginMenu(c_str!("Select server").as_ptr(), true) {
        let data = SELECTION_DATA.get();
        if data.is_some() {
            let maybe = data.unwrap();

            if maybe.is_ok() {
                maybe.as_ref().unwrap().iter().for_each(|e| {
                    let result = std::ffi::CString::new(e.name.to_owned());
                    let c_string = result.unwrap();
                    let label = c_string.as_ptr();

                    if igButton(label, ImVec2::zero()) {
                        settings.world_id = e.id;
                    }
                })
            } else {
                igText(c_str!("Error loading data!").as_ptr())
            }
        } else {
            igText(c_str!("Loading data...").as_ptr());
        }
        igEndMenu();
    }
}

unsafe fn render_button_load() -> bool {
    if igButton(c_str!("Load server selection").as_ptr(), ImVec2::zero()) {
        thread::spawn(|| {
            let result = match get_http_client() {
                Some(client) => {
                    fetch_worlds(client)
                }
                None => {
                    Err(String::from("No http client available!"))
                }
            };
            SELECTION_DATA.set(result).expect("It should not be possible to trigger loading of worlds multiple times!")
        });
        true
    } else {
        false
    }
}

fn fetch_worlds(client: &Client) -> Result<Vec<WorldInfo>, String> {
    thread::sleep(Duration::new(1, 0));
    let result = client.get("https://api.guildwars2.com/v2/worlds?ids=all").send();
    match result {
        Ok(response) => {
            let a: Result<Vec<WorldInfo>, _> = response.json();
            match a {
                Ok(o) => {
                    let mut z = o;
                    z.sort_by(|a, b| a.name.cmp(&b.name));
                    Ok(z)
                }
                Err(e) => { Err(e.to_string()) }
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}
