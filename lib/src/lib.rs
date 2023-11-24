use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::transmute;
use std::ops::Deref;
use imgui_sys::*;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11;
use crate::api::matchup::Matchup;
use crate::api::owner::Faction;
use crate::api::owner::Faction::{BLUE, GREEN, RED};
use crate::api::world_map_type::WorldMapType;
use crate::api::objective_definition::ObjectiveDefinition;

mod integration;

#[cfg(windows)]
type GfxDevice = *const Direct3D11::ID3D11Device;
#[cfg(not(windows))]
type GfxDevice<'a> = &'a glium::Display;

mod api;
mod icons;
mod map_renderer;

static mut MATCHUP: Option<Matchup> = None;
static mut OBJECTIVES: Option<Vec<ObjectiveDefinition>> = None;
static mut ICONS: Option<HashMap<icons::Icon, ImGuiIcon>> = None;


#[cfg(not(windows))]
pub fn nithanim_setup(device: GfxDevice, textures: &mut imgui_glium_renderer::imgui::Textures<imgui_glium_renderer::Texture>) {
    let mut f = |x: imgui_glium_renderer::Texture| textures.insert(x);
    nithanim_setup_internal(device, &mut f);
}


use integration::{
    TextureIdType, TextureDataType
};

pub(crate) fn nithanim_setup_internal<F>(device: GfxDevice, imgui_converter: &mut F)
    where
        F: FnMut(TextureDataType) -> TextureIdType {
    //imgui_sys::igSetCurrentContext()
    //imgui_sys::igSetAllocatorFunctions()

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
    }
}

fn get_wold_map_type(faction: Faction) -> WorldMapType {
    return match faction {
        RED => WorldMapType::RED,
        GREEN => WorldMapType::GREEN,
        BLUE => WorldMapType::BLUE,
    };
}

fn get_home_world_faction(home_world: i32) -> Option<Faction> {
    let worlds = unsafe { &MATCHUP.as_ref().unwrap().all_worlds };

    return if worlds.red.iter().find(|&&w| w == home_world).is_some() {
        Some(RED)
    } else if worlds.green.iter().find(|&&w| w == home_world).is_some() {
        Some(GREEN)
    } else if worlds.blue.iter().find(|&&w| w == home_world).is_some() {
        Some(BLUE)
    } else {
        None
    };
}

#[no_mangle]
pub extern "C" fn nithanim_ui() {
    unsafe {
        map_renderer::render_map((&OBJECTIVES.as_ref()).unwrap(), (&ICONS.as_ref()).unwrap(), (&MATCHUP.as_ref()).unwrap());
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





