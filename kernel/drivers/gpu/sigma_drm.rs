/// SigmaOS: sigma_drm module
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.
/// Inspired by: Linux DRM subsystem, Mesa 3D

#![no_std]
#![allow(dead_code)]

// ── Kernel Primitive Types ─────────────────────────────────────────────────────
type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
const MAX_CONNECTORS: SigmaUsize = 8;
const MAX_MODES: SigmaUsize = 32;
const MAX_GEM_OBJECTS: SigmaUsize = 256;

// ── DRM Mode ───────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode {
    pub hdisplay: SigmaU64,
    pub vdisplay: SigmaU64,
    pub vrefresh: SigmaU64,
    pub clock: SigmaU64,
    pub flags: SigmaU64,
    pub name: [u8; 32],
}

// ── DRM Connector ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_connector {
    pub id: SigmaU64,
    pub type: SigmaU64,
    pub status: SigmaU64,
    pub num_modes: SigmaU64,
    pub preferred: SigmaU64,
    pub active_fb: SigmaU64,
}

// ── GEM Object ─────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem_object {
    pub handle: SigmaU64,
    pub gpu_pa: SigmaU64,
    pub cpu_va: SigmaU64,
    pub size: SigmaU64,
    pub refcount: SigmaU64,
    pub in_use: SigmaBool,
    pub domain: SigmaU64,
}

// ── Sigma Framebuffer ───────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigma_framebuffer {
    pub handle: SigmaU64,
    pub width: SigmaU64,
    pub height: SigmaU64,
    pub stride: SigmaU64,
    pub format: SigmaU64,
}

// ── DRM Device ─────────────────────────────────────────────────────────────
pub struct DrmDevice {
    pub connectors: [drm_connector; MAX_CONNECTORS],
    pub connector_count: SigmaUsize,
    pub gem_objects: [gem_object; MAX_GEM_OBJECTS],
    pub gem_count: SigmaUsize,
    pub next_handle: SigmaU64,
    pub initialized: SigmaBool,
}

impl DrmDevice {
    pub const fn new() -> Self {
        Self {
            connectors: [drm_connector {
                id: 0, type: 0, status: 0, num_modes: 0, preferred: 0, active_fb: 0
            }; MAX_CONNECTORS],
            connector_count: 0,
            gem_objects: [gem_object {
                handle: 0, gpu_pa: 0, cpu_va: 0, size: 0, refcount: 0, in_use: false, domain: 0
            }; MAX_GEM_OBJECTS],
            gem_count: 0,
            next_handle: 1,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> SigmaI32 {
        self.initialized = true;
        0
    }

    pub fn create_gem_object(
        &mut self,
        size: SigmaU64,
        domain: SigmaU64,
    ) -> Option<SigmaU64> {
        if self.gem_count >= MAX_GEM_OBJECTS {
            return None;
        }

        let idx = self.gem_count;
        let handle = self.next_handle;
        self.next_handle += 1;

        self.gem_objects[idx].handle = handle;
        self.gem_objects[idx].size = size;
        self.gem_objects[idx].domain = domain;
        self.gem_objects[idx].refcount = 1;
        self.gem_objects[idx].in_use = true;
        self.gem_count += 1;

        Some(handle)
    }

    pub fn destroy_gem_object(&mut self, handle: SigmaU64) -> SigmaI32 {
        for i in 0..self.gem_count {
            if self.gem_objects[i].handle == handle {
                self.gem_objects[i].refcount -= 1;
                if self.gem_objects[i].refcount == 0 {
                    self.gem_objects[i].in_use = false;
                }
                return 0;
            }
        }
        -1
    }
}

static mut G_DRM: DrmDevice = DrmDevice::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_drm_init() -> SigmaI32 {
    G_DRM.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_drm_create_gem_object(size: SigmaU64, domain: SigmaU64) -> SigmaU64 {
    match G_DRM.create_gem_object(size, domain) {
        Some(handle) => handle,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_drm_destroy_gem_object(handle: SigmaU64) -> SigmaI32 {
    G_DRM.destroy_gem_object(handle)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_drm_connector_count() -> SigmaU32 {
    G_DRM.connector_count as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_drm_gem_count() -> SigmaU32 {
    G_DRM.gem_count as SigmaU32
}



