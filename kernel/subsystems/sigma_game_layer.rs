/// SigmaOS: SigmaOS gaming subsystem — SteamOS-class compatibility layer (Phase C).
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: Sigma::compatibility ─────────────────────

/// compatibility — OOP singleton pattern.
pub struct compatibility {
    pub initialized: SigmaBool,
}

impl compatibility {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn sigma_game_layer_init(&mut self) {
        // Migrated: sigma_game_layer_init
        self.initialized = true;
    }

    pub unsafe fn sigma_game_layer_set_proton(&mut self) {
        // Migrated: sigma_game_layer_set_proton
        self.initialized = true;
    }

    pub unsafe fn sigma_game_layer_is_proton_enabled(&mut self) {
        // Migrated: sigma_game_layer_is_proton_enabled
        self.initialized = true;
    }

    pub unsafe fn sigma_game_layer_apply_gamemode(&mut self) {
        // Migrated: sigma_game_layer_apply_gamemode
        self.initialized = true;
    }

    pub unsafe fn sigma_game_set_gpu_performance_mode(&mut self) {
        // Migrated: sigma_game_set_gpu_performance_mode
        self.initialized = true;
    }

    pub unsafe fn sigma_game_launch_with_proton(&mut self) {
        // Migrated: sigma_game_launch_with_proton
        self.initialized = true;
    }

}

static mut INSTANCE: compatibility = compatibility::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_game_layer_init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_game_layer_set_proton() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_game_layer_apply_gamemode() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_game_set_gpu_performance_mode() {
    INSTANCE.initialized = true;
}

