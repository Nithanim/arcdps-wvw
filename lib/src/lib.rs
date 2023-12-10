use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::transmute;
use std::ops::Deref;
use imgui_sys::*;
use mumblelink_reader::mumble_link::{MumbleLinkDataReader, MumbleLinkReader};
use mumblelink_reader::mumble_link_handler::MumbleLinkHandler;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11;
use crate::api::matchup::Matchup;
use crate::api::objective_definition::ObjectiveDefinition;

use integration::{
    TextureIdType, TextureDataType,
};
use crate::integration::{GfxDevice, setup_mumble_link};
use crate::settings::Settings;

mod integration;

mod api;
mod icons;
mod map_renderer;
mod pos_renderer;
mod data;
mod images;
mod mumble;
pub mod options;
pub mod settings;
mod helpers;


static mut MATCHUP: Option<Matchup> = None;
static mut OBJECTIVES: Option<Vec<ObjectiveDefinition>> = None;
static mut ICONS: Option<HashMap<icons::Icon, ImGuiIcon>> = None;

pub static mut SETTINGS: Settings = Settings {
    show_current: false,
    show_red: false,
    show_green: false,
    show_blue: false,
    show_eternal: false,
};

pub(crate) static mut MUMBLE_LINK: Option<MumbleLinkHandler> = None;

#[cfg(not(windows))]
pub fn nithanim_setup(device: GfxDevice, textures: &mut imgui_glium_renderer::imgui::Textures<imgui_glium_renderer::Texture>) {
    let mut f = |x: imgui_glium_renderer::Texture| textures.insert(x);
    nithanim_setup_internal(device, &mut f);
}

pub(crate) fn nithanim_setup_internal<F>(device: GfxDevice, imgui_converter: &mut F)
    where
        F: FnMut(TextureDataType) -> TextureIdType {
    //imgui_sys::igSetCurrentContext()
    //imgui_sys::igSetAllocatorFunctions()

    data::init();

    let matchup: Matchup = serde_json::from_str(include_str!("../resources/cache/matchup.json")).unwrap();
    let objectives: Vec<ObjectiveDefinition> = serde_json::from_str(include_str!("../resources/cache/objectives.json")).unwrap();
    unsafe {
        MATCHUP = Some(matchup);
        OBJECTIVES = Some(objectives);

        let mut map: HashMap<icons::Icon, ImGuiIcon> = HashMap::new();
        for icon in [icons::Icon::ObjectiveCastle,
            icons::Icon::ObjectiveKeep,
            icons::Icon::ObjectiveTower,
            icons::Icon::ObjectiveCamp,
            icons::Icon::ObjectiveSentry, ] {
            let result = integration::load_icon(icon, device, imgui_converter);
            map.insert(icon, result.unwrap());
        }
        ICONS = Some(map);

        setup_mumble_link();
    }
}


#[no_mangle]
pub extern "C" fn nithanim_ui() {
    let lock_result = data::DATA.lock();
    let mutex = lock_result.unwrap();
    let data = mutex.as_ref();

    unsafe {
        map_renderer::render(
            (&OBJECTIVES.as_ref()).unwrap(),
            (&ICONS.as_ref()).unwrap(),
            data,
            &SETTINGS);

        pos_renderer::render();
    }
}

pub struct ImGuiIcon {
    #[cfg(windows)]
    texture: Direct3D11::ID3D11ShaderResourceView,
    #[cfg(not(windows))]
    texture: imgui_glium_renderer::imgui::TextureId,
    size: ImVec2,
}

impl ImGuiIcon {
    #[cfg(windows)]
    pub fn to_imgui_id(&self) -> ImTextureID {
        unsafe {
            let a: *const c_void = *transmute::<_, &*const c_void>(&self.texture);
            a as ImTextureID
        }
    }
    #[cfg(not(windows))]
    pub fn to_imgui_id(&self) -> ImTextureID {
        self.texture.id() as ImTextureID
    }
}

#[cfg(not(windows))]
impl Into<ImTextureID> for ImGuiIcon {
    fn into(self) -> ImTextureID {
        self.texture.id() as ImTextureID
    }
}





