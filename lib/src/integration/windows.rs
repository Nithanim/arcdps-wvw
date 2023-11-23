use std::ffi::{c_char, c_void};
use std::ptr::{null_mut};
use imgui_sys::{igSetAllocatorFunctions, igSetCurrentContext, ImGuiContext};
use winapi::ctypes::__uint32;
use winapi::shared::minwindef::{HMODULE};
use winapi::um::d3d11;
//use winapi::um::wincon::FreeConsole;

use crate::integration::arcdps::*;

static mut filelog: *mut c_void = null_mut();

#[no_mangle]
pub unsafe extern "system" fn get_init_addr(
    arcversion: *const c_char,
    imguictx: *mut ImGuiContext,
    id3dptr: *mut c_void,
    arcdll: HMODULE,
    mallocfn: *const c_void,
    freefn: *const c_void,
    d3dversion: __uint32) -> unsafe extern "system" fn() -> *const arcdps_exports {
    igSetCurrentContext(imguictx);
    igSetAllocatorFunctions(Some(std::mem::transmute(mallocfn)), Some(std::mem::transmute(freefn)), null_mut());

    eprintln!("WE ARE HERE, BOYS!");

    crate::nithanim_setup_internal(id3dptr as *const d3d11::ID3D11Device, &mut |x| ());

    mod_init
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


