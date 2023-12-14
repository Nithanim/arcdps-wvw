use std::ops::Deref;
use std::rc::Rc;
use imgui_sys::ImVec2;
use crate::{icons, ImGuiIcon};

pub type GfxDevice<'a> = &'a glium::Display;

pub type TextureDataType = imgui_glium_renderer::Texture;
pub type TextureIdType = imgui_glium_renderer::imgui::TextureId;


pub unsafe fn setup_mumble_link() {}

pub unsafe fn load_icon<F>(icon: icons::Icon, device: GfxDevice, imgui_converter: &mut F) -> Result<ImGuiIcon, String>
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

// DUMMY FOR WINDOWS
pub unsafe extern "C" fn mod_wnd() -> usize {
    return 1 as usize
}