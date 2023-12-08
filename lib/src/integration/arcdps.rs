use std::ffi::{c_char, c_void, CStr};
use std::ptr::{null, null_mut};
use crate::options::render_options;
use crate::SETTINGS;

static mut filelog: *mut c_void = null_mut();


const EXT_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"WvW Display\0") };
const EXT_VERSION: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"0.1\0") };

pub static mut ARC_EXPORTS_STATIC: arcdps_exports = arcdps_exports {
    sig: 0xF3D01609, // If extension should not be loaded, set to 0
    imguivers: 18000,
    size: std::mem::size_of::<arcdps_exports>(),
    out_name: EXT_NAME.as_ptr(),
    out_build: EXT_VERSION.as_ptr(),
    combat: null(), // mod_combat as *const c_void,
    wnd_nofilter: null(), //mod_wnd as *const c_void,
    imgui: mod_imgui as *const c_void,
    options_end: options_end as *const c_void,
    combat_local: null(),
    wnd_filter: null(),
    options_windows: null(),
};

pub unsafe extern "C" fn options_end() -> usize {
    render_options(&mut SETTINGS);
    return 0;
}

pub unsafe extern "C" fn options_windows(windowname: *const char) -> usize {
    0
}


#[no_mangle]
pub unsafe extern "C" fn mod_combat(ev: *mut cbtevent, src: *mut ag, dst: *mut ag, skillname: *mut c_char, id: u64, revision: u64) -> usize {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn mod_imgui(pNotCharSelectionOrLoading: u32, pHideIfCombatOrOoc: u32) -> usize
{
    crate::nithanim_ui();
    if pNotCharSelectionOrLoading == 0 || pHideIfCombatOrOoc != 0
    {
        return 0;
    }
    return 0;
}



// TODO quick-fix to dummy because on the linux-tester, we never call it
// but this will explode on a windows-test tool!
#[cfg(not(windows))]
pub unsafe extern "C" fn mod_wnd() -> usize {
    panic!("NOT IMPLEMENTED")
}

#[repr(C)]
pub struct arcdps_exports {
    pub size: usize,
    /* size of exports table */
    pub sig: u32,
    /* pick a number between 0 and uint32_t max that isn't used by other modules */
    pub imguivers: u32,
    /* set this to IMGUI_VERSION_NUM. if you don't use imgui, 18000 (as of 2021-02-02) */
    pub out_name: *const c_char,
    /* name string */
    pub out_build: *const c_char,
    /* build string */
    pub wnd_nofilter: *const c_void,
    /* wndproc callback, fn(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam), return assigned to umsg */
    pub combat: *const c_void,
    /* combat event callback, fn(cbtevent* ev, ag* src, ag* dst, char* skillname, uint64_t id, uint64_t revision) */
    pub imgui: *const c_void,
    /* ::present callback, before imgui::render, fn(uint32_t not_charsel_or_loading, uint32_t hide_if_combat_or_ooc) */
    pub options_end: *const c_void,
    /* ::present callback, appending to the end of options window in arcdps, fn() */
    pub combat_local: *const c_void,
    /* combat event callback like area but from chat log, fn(cbtevent* ev, ag* src, ag* dst, char* skillname, uint64_t id, uint64_t revision) */
    pub wnd_filter: *const c_void,
    /* wndproc callback like wnd_nofilter above, input filered using modifiers */
    pub options_windows: *const c_void,
    /* called once per 'window' option checkbox, with null at the end, non-zero return disables arcdps drawing that checkbox, fn(char* windowname) */
}

#[repr(C)]
pub struct ag {
    pub name: *mut c_char,
    /* agent name. may be null. valid only at time of event. utf8 */
    pub id: usize,
    /* agent unique identifier */
    pub prof: u32,
    /* profession at time of event. refer to evtc notes for identification */
    pub elite: u32,
    /* elite spec at time of event. refer to evtc notes for identification */
    pub self_: u32,
    /* 1 if self, 0 if not */
    pub team: u16,
    /* sep21+ */
}

#[repr(C)]
pub struct cbtevent {
    pub time: u64,
    pub src_agent: u64,
    pub dst_agent: u64,
    pub value: u32,
    pub buff_dmg: u32,
    pub overstack_value: u32,
    pub skillid: u32,
    pub src_instid: u16,
    pub dst_instid: u16,
    pub src_master_instid: u16,
    pub dst_master_instid: u16,
    pub iff: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buffremove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_offcycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}
