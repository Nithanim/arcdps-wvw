use std::ffi::c_void;
use std::ops::Deref;
use imgui_sys::*;
use windows::Win32::Graphics::Direct3D11;
use windows::Win32::Graphics::Direct3D11::{D3D11_SHADER_RESOURCE_VIEW_DESC_0, D3D11_TEX2D_SRV};
use windows::Win32::Graphics::Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM;
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use crate::{icons, ImGuiIcon, TextureDataType, TextureIdType};

pub unsafe fn load_icon<F>(icon: icons::Icon, device: *const Direct3D11::ID3D11Device, f: &mut F) -> Result<ImGuiIcon, String>
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


    let mut texture: Option<Direct3D11::ID3D11Texture2D> = None;
    let create_texture2dres = device.CreateTexture2D(
        &desc,
        Some(&sub_resource),
        Some(&mut texture),
    );

    if create_texture2dres.is_err() {
        panic!("Error creating 2d texture!");
    }
    if texture.is_none() {
        panic!("WTF1??");
    }

    let srv_desc_texture = Direct3D11::D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: format,
        ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
        Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
            Texture2D: D3D11_TEX2D_SRV {
                MipLevels: 1,
                MostDetailedMip: 0,
            }
        },
    };

    let texture = texture.unwrap();

    let mut d11texture: Option<Direct3D11::ID3D11ShaderResourceView> = None;
    let v = device.CreateShaderResourceView(&texture, Some(&srv_desc_texture), Some(&mut d11texture));

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
