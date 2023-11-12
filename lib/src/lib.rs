use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::iter::Map;
use std::ops::Deref;
use imgui_sys::*;
#[cfg(windows)]
use winapi::{
    shared::{
        dxgiformat::DXGI_FORMAT_B8G8R8A8_UNORM,
        dxgitype::DXGI_SAMPLE_DESC,
    },
    um::d3d11,
    um::d3dcommon,
};
use crate::api::matchup::Matchup;
use crate::api::owner::Faction;
use crate::api::owner::Faction::{BLUE, GREEN, RED};
use crate::api::world_map_type::WorldMapType;
use std::rc::Rc;
use crate::api::objective_definition;
use crate::api::objective_definition::ObjectiveDefinition;

#[cfg(windows)]
type GfxDevice = d3d11::ID3D11Device;
#[cfg(not(windows))]
type GfxDevice<'a> = &'a glium::Display;

/*
use winapi::{
    shared::{dxgi, dxgiformat, dxgitype, minwindef::TRUE, windef::HWND, winerror},
    um::{d3d11, d3d11_1, d3d11sdklayers, d3dcommon},
};*/


mod api;
mod icons;

static mut MATCHUP: Option<Matchup> = None;
static mut OBJECTIVES: Option<Vec<ObjectiveDefinition>> = None;
static mut ICONS: Option<HashMap<icons::Icon, ImGuiIcon>> = None;


#[cfg(not(windows))]
pub fn nithanim_setup(device: GfxDevice, textures: &mut imgui_glium_renderer::imgui::Textures<imgui_glium_renderer::Texture>) {
    let mut f = |x: imgui_glium_renderer::Texture| textures.insert(x);
    nithanim_setup_internal(device, &mut f);
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn nithanim_setup(device: GfxDevice) {
    //imgui_sys::igSetCurrentContext()
    //imgui_sys::igSetAllocatorFunctions()

    nithanim_setup_internal(device, ());
}


fn nithanim_setup_internal<F>(device: GfxDevice, imgui_converter: &mut F)
    where
        F: FnMut(imgui_glium_renderer::Texture) -> imgui_glium_renderer::imgui::TextureId {
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
        igBegin(CString::new("WvW").unwrap().as_ptr(), &mut true, 0);
        igText(CString::new("HELLO").unwrap().as_ptr());
        igButton(CString::new("gfgdfg").unwrap().as_ptr(), ImVec2::new(200f32, 15f32));

        let z = (&OBJECTIVES.as_ref()).unwrap();
        let single_map_objectives: Vec<&ObjectiveDefinition> = z.iter()
            .filter(|&f| f.map_type == "Center")
            .filter(|&f| match &f.type_ {
                objective_definition::Type::CAMP | objective_definition::Type::TOWER | objective_definition::Type::KEEP | objective_definition::Type::CASTLE => true,
                _default => false
            })
            .collect();


        let mut pos = ImVec2::zero();
        igGetCursorScreenPos(&mut pos);
        //let draw_list = igGetWindowDrawList();
        let mut draw_area = ImVec2::zero();
        let available = igGetContentRegionAvail(&mut draw_area);

        let u = (&ICONS.as_ref()).unwrap();
        let uv0 = ImVec2::new(0.0, 0.0);
        let uv1 = ImVec2::new(1.0, 1.0);
        let tint_color = ImVec4::new(1.0, 1.0, 1.0, 1.0);
        let border_color = ImVec4::new(0.0, 0.0, 0.0, 0.0);

        for objective in single_map_objectives {
            let ic = match &objective.type_ {
                objective_definition::Type::CAMP => Some(&icons::Icon::ObjectiveCamp),
                objective_definition::Type::TOWER  => Some(&icons::Icon::ObjectiveTower),
                 objective_definition::Type::KEEP =>  Some(&icons::Icon::ObjectiveKeep),
                objective_definition::Type::CASTLE =>  Some(&icons::Icon::ObjectiveCastle),
                _default => None,
            }.map(|m|u.get(m).unwrap());

            if ic.is_some() {
                igImage(
                    ic.unwrap().to_imgui_id(),
                    ic.unwrap().size, uv0,
                    uv1,
                    tint_color,
                    border_color,
                );
            }
        }


        //igDummy(ImVec2::new(200f32, 200f32));
        igEnd();
    }
}

#[cfg(not(windows))]
unsafe fn load_icon<F>(icon: icons::Icon, device: GfxDevice, imgui_converter: &mut F) -> Result<ImGuiIcon, String>
    where
        F: FnMut(imgui_glium_renderer::Texture) -> imgui_glium_renderer::imgui::TextureId {
    let icon_data = icon.value();
    let bytes: &[u8] = icon_data.bytes.deref();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba(Vec::from(bytes), (icon_data.size.x as u32, icon_data.size.y as u32));
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
        size: icon.value().size,
        texture: imgui_converter(texture),
    })
}


#[cfg(windows)]
unsafe fn load_icon(icon: icons::Icon, device: GfxDevice, f: ()) -> Result<ImGuiIcon, String> {
    let bytes: &[u8] = icon.value().bytes.deref();


    // https://github.com/knoxfighter/arcdps-extension/blob/ef878f37307ff4bc95289623389a6e01521d7a12/Icon.cpp#L213C28-L213C28
    let desc = d3d11::D3D11_TEXTURE2D_DESC {
        Width: 32,
        Height: 32,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: d3d11::D3D11_USAGE_DEFAULT,
        BindFlags: d3d11::D3D11_BIND_SHADER_RESOURCE,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };

    let sub_resource = d3d11::D3D11_SUBRESOURCE_DATA {
        pSysMem: bytes.as_ptr() as *const c_void,
        SysMemPitch: desc.Width * 4,
        SysMemSlicePitch: 0,
    };


    let mut pTexture: *mut d3d11::ID3D11Texture2D = ptr::null_mut();
    let create_texture2dres = device.CreateTexture2D(&desc, &sub_resource, &mut pTexture);

    if !create_texture2dres {
        panic!("Error creating 2d texture: ");
    }

    let mut srvDescTexture: d3d11::D3D11_SHADER_RESOURCE_VIEW_DESC_u = std::mem::zeroed();
    *(srvDescTexture.Texture2D_mut()) = d3d11::D3D11_TEX2D_SRV {
        MipLevels: desc.MipLevels, // Assuming `desc` is available and contains MipLevels
        MostDetailedMip: 0,
        ..*srvDescTexture.Texture2D_mut()
    };

    let srvDesc = d3d11::D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        ViewDimension: d3dcommon::D3D11_SRV_DIMENSION_TEXTURE2D,
        u: srvDescTexture,
    };

    let mut d11texture: *mut d3d11::ID3D11ShaderResourceView = ptr::null_mut();
    device.CreateShaderResourceView(pTexture as *mut d3d11::ID3D11Resource, &srvDesc, &mut d11texture);

    pTexture.Release();

    return Ok(ImGuiIcon {
        size: icon.value().size,
        texture: d11texture,
    });
}

pub struct ImGuiIcon {
    #[cfg(windows)]
    texture: *mut d3d11::ID3D11ShaderResourceView,
    #[cfg(not(windows))]
    texture: imgui_glium_renderer::imgui::TextureId,
    size: ImVec2,
}

impl ImGuiIcon {
    #[cfg(windows)]
    pub fn to_imgui_id(&self) -> ImTextureID {
        self.texture as ImTextureID
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

#[cfg(windows)]
impl Into<ImTextureID> for ImGuiIcon {
    fn into(self) -> ImTextureID {
        self.texture as ImTextureID
    }
}




