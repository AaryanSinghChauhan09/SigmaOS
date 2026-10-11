// SigmaOS Direct Rendering Manager (DRM) & Kernel Mode Setting (KMS) Atomic Engine
// Coordinates tear-free atomic display commits, CRTC page flips, and plane overlays.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

pub const DRM_MODE_ATOMIC_TEST_ONLY: u32 = 0x0100;
pub const DRM_MODE_ATOMIC_NONBLOCK: u32 = 0x0200;
pub const DRM_MODE_ATOMIC_ALLOW_MODESET: u32 = 0x0400;

/// Video display resolution & timings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayMode {
    pub hdisplay: u16,
    pub vdisplay: u16,
    pub vrefresh: u32,
    pub pixel_clock_khz: u32,
}

/// DRM Display Plane (Primary, Overlay, Cursor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneType {
    Primary,
    Overlay,
    Cursor,
}

/// Atomic State Property Value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomicProperty {
    pub object_id: u32,
    pub property_id: u32,
    pub value: u64,
}

/// Atomic Commit Transaction Request
#[derive(Debug, Clone, Default)]
pub struct DrmAtomicRequest {
    pub properties: Vec<AtomicProperty>,
    pub flags: u32,
}

impl DrmAtomicRequest {
    pub fn new(flags: u32) -> Self {
        Self {
            properties: Vec::new(),
            flags,
        }
    }

    pub fn add_property(&mut self, object_id: u32, property_id: u32, value: u64) {
        self.properties.push(AtomicProperty {
            object_id,
            property_id,
            value,
        });
    }
}

/// Active Display CRTC Hardware Controller
#[derive(Debug, Clone)]
pub struct DrmCrtc {
    pub id: u32,
    pub is_enabled: bool,
    pub current_mode: DisplayMode,
    pub active_fb_id: u32,
    pub vblank_counter: u64,
}

/// DRM/KMS Atomic Modesetting Driver Subsystem
#[derive(Debug, Clone)]
pub struct DrmKmsAtomicDriver {
    pub crtcs: Vec<DrmCrtc>,
    pub current_fb_id: u32,
    pub atomic_commits_count: u64,
}

impl DrmKmsAtomicDriver {
    pub fn new() -> Self {
        let default_mode = DisplayMode {
            hdisplay: 1920,
            vdisplay: 1080,
            vrefresh: 60,
            pixel_clock_khz: 148500,
        };

        let crtc0 = DrmCrtc {
            id: 1,
            is_enabled: true,
            current_mode: default_mode,
            active_fb_id: 100,
            vblank_counter: 0,
        };

        Self {
            crtcs: alloc::vec![crtc0],
            current_fb_id: 100,
            atomic_commits_count: 0,
        }
    }

    /// Perform atomic test or live hardware commit
    pub fn atomic_commit(&mut self, req: &DrmAtomicRequest) -> Result<(), &'static str> {
        let is_test_only = (req.flags & DRM_MODE_ATOMIC_TEST_ONLY) != 0;

        // Validate all requested properties
        for prop in &req.properties {
            if prop.object_id == 0 {
                return Err("EINVAL: Invalid DRM object ID in atomic request");
            }
        }

        if is_test_only {
            return Ok(()); // Dry-run validation succeeded
        }

        // Apply changes atomically across CRTCs
        for prop in &req.properties {
            for crtc in &mut self.crtcs {
                if crtc.id == prop.object_id {
                    // Update active framebuffer ID on page flip
                    crtc.active_fb_id = prop.value as u32;
                    crtc.vblank_counter += 1;
                }
            }
        }

        self.atomic_commits_count += 1;
        Ok(())
    }
}

impl Default for DrmKmsAtomicDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drm_kms_atomic_commit() {
        let mut drm = DrmKmsAtomicDriver::new();
        assert_eq!(drm.crtcs[0].active_fb_id, 100);

        // Prepare page flip to framebuffer 101
        let mut req = DrmAtomicRequest::new(DRM_MODE_ATOMIC_NONBLOCK);
        req.add_property(1, 42, 101);

        assert!(drm.atomic_commit(&req).is_ok());
        assert_eq!(drm.crtcs[0].active_fb_id, 101);
        assert_eq!(drm.crtcs[0].vblank_counter, 1);
        assert_eq!(drm.atomic_commits_count, 1);
    }
}
