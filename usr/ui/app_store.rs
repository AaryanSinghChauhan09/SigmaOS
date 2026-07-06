/// SigmaOS: usr/ui/app_store.rs
/// Front-end UI for the SigPkg Package Manager.
/// Displays Sovereign applications available for install.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;

pub const MAX_STORE_APPS: SigmaUsize = 32;

#[derive(Copy, Clone)]
pub struct StoreApp {
    pub pkg_id: SigmaU32,
    pub name: [u8; 16],
    pub desc: [u8; 64],
    pub is_installed: bool,
    pub size_kb: SigmaU32,
}

impl StoreApp {
    pub const fn empty() -> Self {
        StoreApp {
            pkg_id: 0,
            name: [0; 16],
            desc: [0; 64],
            is_installed: false,
            size_kb: 0,
        }
    }
}

static mut STORE_CATALOG: [StoreApp; MAX_STORE_APPS] = [StoreApp::empty(); MAX_STORE_APPS];
static mut STORE_COUNT: SigmaUsize = 0;

extern "C" {
    fn sigpkg_install(pkg_id: SigmaU32) -> SigmaI32;
    fn sigpkg_remove(pkg_id: SigmaU32) -> SigmaI32;
}

#[no_mangle]
pub unsafe extern "C" fn app_store_init() -> SigmaI32 {
    // Hardcode some defaults for the stub
    if STORE_COUNT == 0 {
        let mut app1 = StoreApp::empty();
        app1.pkg_id = 101;
        app1.size_kb = 4096;
        // Mock name copying
        let name = b"SigmaCalc\0";
        core::ptr::copy_nonoverlapping(name.as_ptr(), app1.name.as_mut_ptr(), name.len());
        STORE_CATALOG[0] = app1;
        STORE_COUNT += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn app_store_click_install(index: SigmaUsize) -> SigmaI32 {
    if index < STORE_COUNT {
        let pkg_id = STORE_CATALOG[index].pkg_id;
        
        // Trigger backend sigpkg via extern C IPC call
        let rc = sigpkg_install(pkg_id);
        if rc == 0 {
            STORE_CATALOG[index].is_installed = true;
        }
        return rc;
    }
    -4 // ENOENT
}