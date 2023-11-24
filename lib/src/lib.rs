use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::transmute;
use std::ops::Deref;
#[cfg(not(windows))]
use std::rc::Rc;
use imgui_sys::*;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::{D3D11_SHADER_RESOURCE_VIEW_DESC_0, D3D11_TEX2D_SRV};
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use crate::api::matchup::Matchup;
use crate::api::owner::Faction;
use crate::api::owner::Faction::{BLUE, GREEN, RED};
use crate::api::world_map_type::WorldMapType;
use crate::api::objective_definition::ObjectiveDefinition;

#[cfg(windows)]
mod integration;

#[cfg(windows)]
type GfxDevice = *const Direct3D11::ID3D11Device;
#[cfg(not(windows))]
type GfxDevice<'a> = &'a glium::Display;

/*
use winapi::{
    shared::{dxgi, dxgiformat, dxgitype, minwindef::TRUE, windef::HWND, winerror},
    um::{d3d11, d3d11_1, d3d11sdklayers, d3dcommon},
};*/


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

#[cfg(not(windows))]
type TextureDataType = imgui_glium_renderer::Texture;
#[cfg(windows)]
type TextureDataType = ();
#[cfg(not(windows))]
type TextureIdType = imgui_glium_renderer::imgui::TextureId;
#[cfg(windows)]
type TextureIdType = ();

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
            let result = load_icon(icon, device, imgui_converter);
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

#[cfg(not(windows))]
unsafe fn load_icon<F>(icon: icons::Icon, device: GfxDevice, imgui_converter: &mut F) -> Result<ImGuiIcon, String>
    where
        F: FnMut(imgui_glium_renderer::Texture) -> imgui_glium_renderer::imgui::TextureId {
    let icon_data = icon.value();
    let bytes: &[u8] = icon_data.bytes.deref();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba(Vec::from(bytes), (icon_data.size.w, icon_data.size.h));
    let gl_texture = glium::Texture2d::new(device, raw_image).unwrap();

    let texture = imgui_glium_renderer::Texture {
        texture: Rc::new(gl_texture),
        sampler: glium::uniforms::SamplerBehavior {
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
            minify_filter: glium::uniforms::MinifySamplerFilter::Linear,
            ..Default::default()
        },
    };

    Ok(ImGuiIcon {
        size: ImVec2::new(icon.value().size.w as f32, icon.value().size.h as f32),
        texture: imgui_converter(texture),
    })
}


#[cfg(windows)]
unsafe fn load_icon<F>(icon: icons::Icon, device: *const Direct3D11::ID3D11Device, f: &mut F) -> Result<ImGuiIcon, String>
    where
        F: FnMut(TextureDataType) -> TextureIdType {
    let device = device.as_ref().unwrap();
    let bytes: &[u8] = icon.value().bytes.deref();

    let format = DXGI_FORMAT_R8G8B8A8_UNORM;

    // https://github.com/knoxfighter/arcdps-extension/blob/ef878f37307ff4bc95289623389a6e01521d7a12/Icon.cpp#L213C28-L213C28
    let desc = Direct3D11::D3D11_TEXTURE2D_DESC {
        Width: icon.value().size.w,
        Height: icon.value().size.h,
        MipLevels: 1,
        ArraySize: 1,
        Format: format,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: Direct3D11::D3D11_USAGE_IMMUTABLE,
        BindFlags: Direct3D11::D3D11_BIND_SHADER_RESOURCE,
        ..Default::default()
    };

    let sub_resource = Direct3D11::D3D11_SUBRESOURCE_DATA {
        pSysMem: bytes.as_ptr() as *const c_void,
        SysMemPitch: desc.Width * 4,
        SysMemSlicePitch: 0,
    };


    let mut pTexture: Option<Direct3D11::ID3D11Texture2D> = None;
    let create_texture2dres = device.CreateTexture2D(
        &desc,
        Some(&sub_resource),
        Some(&mut pTexture),
    );

    if create_texture2dres.is_err() {
        panic!("Error creating 2d texture!");
    }
    if pTexture.is_none() {
        panic!("WTF1??");
    }

    let srvDescTexture = Direct3D11::D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: format,
        ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
        Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
            Texture2D: D3D11_TEX2D_SRV {
                MipLevels: 1,
                MostDetailedMip: 0,
            }
        },
    };

    let pTexture = pTexture.unwrap();

    let mut d11texture: Option<Direct3D11::ID3D11ShaderResourceView> = None;
    let v = device.CreateShaderResourceView(&pTexture, Some(&srvDescTexture), Some(&mut d11texture));

    if v.is_err() {
        panic!("Error creating 2d texture!");
    }
    if d11texture.is_none() {
        panic!("WTF4??????");
    }

    //pTexture.as_ref().unwrap().Release();

    return Ok(ImGuiIcon {
        size: ImVec2::new(icon.value().size.w as f32, icon.value().size.h as f32),
        texture: d11texture.unwrap(),
    });
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





