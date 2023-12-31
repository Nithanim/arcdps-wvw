pub mod icon_loader;

use std::ffi::{c_char, c_void};
use std::mem;
use std::ptr::{null, null_mut};
use imgui_sys::{igSetAllocatorFunctions, igSetCurrentContext, ImGuiContext, ImVec2};
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use windows::Win32::Graphics::Dxgi::{DXGI_SWAP_CHAIN_DESC, IDXGISwapChain};
use windows::Win32::Foundation::{HMODULE, HWND, LPARAM, WPARAM};

use mumblelink_reader::mumble_link::{MumbleLinkDataReader, MumbleLinkReader};
use mumblelink_reader::mumble_link_handler::MumbleLinkHandler;

use crate::integration::arcdps::*;

pub use icon_loader::load_icon;
use crate::hud::screen::set_screen_size;
use crate::{IS_GAME, MUMBLE_LINK};

pub type GfxDevice = *const ID3D11Device;

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
    d3dversion: u32) -> *const c_void {
    if (d3dversion != 11) {
        eprintln!("Only DX11 supported!");
        return null();
    }
    IS_GAME = true;

    igSetCurrentContext(imguictx);
    igSetAllocatorFunctions(Some(std::mem::transmute(mallocfn)), Some(std::mem::transmute(freefn)), null_mut());

    init_dxgi(id3dptr);

    crate::setup(D3D11_DEVICE.as_ref().unwrap(), &mut |x| ());

    mod_init as *const c_void
}

pub static mut DXGI_SWAP_CHAIN: Option<IDXGISwapChain> = None;
pub static mut D3D11_DEVICE: Option<ID3D11Device> = None;

unsafe fn init_dxgi(device: *const c_void) {
    let swap_chain: &IDXGISwapChain = std::mem::transmute(&device);

    let mut a: DXGI_SWAP_CHAIN_DESC = mem::zeroed();
    swap_chain.GetDesc(&mut a);
    set_screen_size(ImVec2::new(a.BufferDesc.Width as f32, a.BufferDesc.Height as f32));

    DXGI_SWAP_CHAIN = Some(swap_chain.clone());
    let dev: Result<ID3D11Device, _> = swap_chain.GetDevice();

    if dev.is_err() {
        panic!("Unable to get Device from DXGI context!");
    }

    D3D11_DEVICE = Some(dev.unwrap());
}

unsafe extern "system" fn mod_init() -> *const arcdps_exports {
    crate::settings::read_from_file();

    return &ARC_EXPORTS_STATIC;
}

#[no_mangle]
pub extern "system" fn get_release_addr() -> *const c_void {
    mod_release as *const c_void
}

#[no_mangle]
pub unsafe extern "system" fn mod_release() -> *mut c_void {
    crate::settings::write_to_file();
    crate::teardown();
    // winapi::um::wincon::FreeConsole();
    null_mut()
}

pub unsafe extern "C" fn mod_wnd(hWnd: HWND, uMsg: u32, wParam: WPARAM, lParam: LPARAM) -> usize {
    const WM_SIZE: i32 = 0x0005;

    if uMsg == WM_SIZE as u32 {
        let width = lParam.0 & 0xFFFF;
        let height = (lParam.0 >> 16) & 0xFFFF;
        set_screen_size(ImVec2::new(width as f32, height as f32));
    }

    return uMsg as usize;
}

pub unsafe fn setup_mumble_link() {
    let result1 = MumbleLinkHandler::new();
    if result1.is_err() {
        eprintln!("Unable to setup mumble link: {}", result1.err().unwrap())
    } else {
        MUMBLE_LINK = result1.ok();
    }
}

