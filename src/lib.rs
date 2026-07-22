#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]
// SigmaOS Library
// Core library for SigmaOS operating system

pub mod accessibility;
pub mod audio;
pub mod automation;
pub mod boot;
pub mod community;
pub mod compatibility;
pub mod customization;
pub mod dashboard;
pub mod debugger;
pub mod desktop;
pub mod device;
pub mod docs;
pub mod driver;
pub mod drivers;
pub mod ecosystem;
pub mod education;
pub mod filesystem;
pub mod finance;
pub mod governance;
pub mod graphics;
pub mod iso;
pub mod kernel;
pub mod legal;
pub mod media;
pub mod memory;
pub mod ml;
pub mod network;
pub mod observability;
pub mod orchestration;
pub mod package;
pub mod phase_l_plans;
pub mod pillars;
pub mod productivity;
pub mod resilience;
pub mod scheduler;
pub mod security;
pub mod shell;
pub mod sigpkg;
pub mod storage;
pub mod support;
pub mod system;
pub mod tools;
pub mod tracing;
pub mod unimplemented_features;
pub mod virtualization;


#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout =
        Layout::from_size_align(size, 8).unwrap_or_else(|_| Layout::from_size_align(8, 8).unwrap());
    std_alloc(layout)
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn free(_ptr: *mut u8) {
    // No-op deallocation in host test environment to avoid layout-tracking complexity.
}
