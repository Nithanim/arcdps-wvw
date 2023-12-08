pub mod icon_loader;

use std::ffi::{c_char, c_void};
use std::mem;
use std::ptr::{null_mut};
use imgui_sys::{igSetAllocatorFunctions, igSetCurrentContext, ImGuiContext};
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use windows::Win32::Graphics::Dxgi::{DXGI_SWAP_CHAIN_DESC, IDXGISwapChain};
use windows::Win32::Foundation::HMODULE;

use crate::integration::arcdps::*;

pub use icon_loader::load_icon;

pub type TextureDataType = ();
pub type TextureIdType = ();

static mut filelog: *mut c_void = null_mut();

#[no_mangle]
pub unsafe extern "system" fn get_init_addr(
    arcversion: *const c_char,
    imguictx: *mut ImGuiContext,
    id3dptr: *mut c_void,
    arcdll: HMODULE,
    mallocfn: *const c_void,
    freefn: *const c_void,
    d3dversion: u32) -> unsafe extern "system" fn() -> *const arcdps_exports {
    igSetCurrentContext(imguictx);
    igSetAllocatorFunctions(Some(std::mem::transmute(mallocfn)), Some(std::mem::transmute(freefn)), null_mut());

    init_dxgi(id3dptr);

    crate::nithanim_setup_internal(D3D11_DEVICE.as_ref().unwrap(), &mut |x| ());

    mod_init
}

pub static mut DXGI_SWAP_CHAIN: Option<IDXGISwapChain> = None;
pub static mut D3D11_DEVICE: Option<ID3D11Device> = None;

unsafe fn init_dxgi(device: *const c_void) {
    let swap_chain: &IDXGISwapChain = std::mem::transmute(&device);

    let mut a: DXGI_SWAP_CHAIN_DESC = mem::zeroed();
    swap_chain.GetDesc(&mut a);
    println!("SIZE: {}x{}", a.BufferDesc.Width, a.BufferDesc.Height);

    DXGI_SWAP_CHAIN = Some(swap_chain.clone());
    let dev: Result<ID3D11Device, _> = swap_chain.GetDevice();

    if dev.is_err() {
        panic!("Unable to get Device from DXGI context!");
    }

    D3D11_DEVICE = Some(dev.unwrap());
}

unsafe extern "system" fn mod_init() -> *const arcdps_exports {
    return &ARC_EXPORTS_STATIC;
}

#[no_mangle]
pub extern "system" fn get_release_addr() -> *const c_void {
    mod_release as *const c_void
}

#[no_mangle]
pub unsafe extern "system" fn mod_release() -> *mut c_void {
    // winapi::um::wincon::FreeConsole();
    null_mut()
}


