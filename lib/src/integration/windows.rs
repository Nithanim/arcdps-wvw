use std::ffi::{c_char, c_void};
use std::mem::size_of;
use std::ptr::{null, null_mut};
use c_str_macro::c_str;
use imgui_sys::{igSetAllocatorFunctions, igSetCurrentContext, ImGuiContext};
use winapi::ctypes::__uint32;
use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::d3d11;
//use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::HANDLE;

use crate::integration::arcdps::*;

static mut filelog: *mut c_void = null_mut();

#[no_mangle]
pub unsafe extern "C" fn get_init_addr(
    arcversion: *mut c_char,
    imguictx: *mut c_void,
    id3dptr: *mut c_void,
    arcdll: HANDLE,
    mallocfn: *mut c_void,
    freefn: *mut c_void,
    d3dversion: __uint32) {
    println!("DEVVV: get_init_addr called!");
    igSetCurrentContext(imguictx as *mut ImGuiContext);
    igSetAllocatorFunctions(Some(std::mem::transmute(mallocfn)), Some(std::mem::transmute(freefn)), null_mut());

    crate::nithanim_setup_internal(id3dptr as *const d3d11::ID3D11Device, &mut |x| ());
}

#[no_mangle]
pub extern "C" fn get_release_addr() -> *mut c_void {
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn mod_release() -> *mut c_void {
    //FreeConsole();
    null_mut()
}


#[no_mangle]
pub unsafe extern "C" fn mod_init() -> *mut arcdps_exports {
    return &mut arc_exports_static;
}
