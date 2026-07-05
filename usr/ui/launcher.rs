/// SigmaOS: usr/ui/launcher.rs
/// App Drawer and AI Semantic Search Menu.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;
type SigmaBool  = bool;

pub const MAX_SEARCH_LEN: SigmaUsize = 64;
pub const MAX_RESULTS: SigmaUsize = 8;

pub struct LauncherState {
    pub is_visible: SigmaBool,
    pub search_buffer: [u8; MAX_SEARCH_LEN],
    pub search_len: SigmaUsize,
    pub selected_index: SigmaU32,
    pub result_ids: [SigmaU32; MAX_RESULTS],
    pub result_count: SigmaUsize,
}

static mut LAUNCHER: LauncherState = LauncherState {
    is_visible: false,
    search_buffer: [0; MAX_SEARCH_LEN],
    search_len: 0,
    selected_index: 0,
    result_ids: [0; MAX_RESULTS],
    result_count: 0,
};

extern "C" {
    fn ai_submit_task(caller: SigmaU32, prio: u8, prompt: *const u8, len: SigmaUsize) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn launcher_toggle() {
    LAUNCHER.is_visible = !LAUNCHER.is_visible;
    if !LAUNCHER.is_visible {
        LAUNCHER.search_len = 0;
        LAUNCHER.result_count = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn launcher_type_char(c: u8) {
    if LAUNCHER.is_visible && LAUNCHER.search_len < MAX_SEARCH_LEN {
        LAUNCHER.search_buffer[LAUNCHER.search_len] = c;
        LAUNCHER.search_len += 1;
        
        // Trigger background AI task for semantic app prediction based on search string
        // Interactive priority (2)
        ai_submit_task(0, 2, LAUNCHER.search_buffer.as_ptr(), LAUNCHER.search_len);
    }
}