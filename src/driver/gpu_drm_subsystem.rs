// SPDX-License-Identifier: MIT
// SigmaOS Direct Rendering Manager (DRM) and Kernel Mode Setting (KMS) Driver Subsystem
// Inspired by Linux DRM (`/dev/dri/card0`, `/dev/dri/renderD128`) and FreeBSD `drm-kmod`

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

// ============================================================================
// Linux / BSD DRM Ioctl Constants
// ============================================================================

pub const DRM_IOCTL_VERSION: u32 = 0xC0406400;
pub const DRM_IOCTL_MODE_GETRESOURCES: u32 = 0xC01064A0;
pub const DRM_IOCTL_MODE_CREATE_DUMB: u32 = 0xC02064B2;
pub const DRM_IOCTL_MODE_MAP_DUMB: u32 = 0xC01064B3;
pub const DRM_IOCTL_MODE_DESTROY_DUMB: u32 = 0xC00464B4;
pub const DRM_IOCTL_MODE_ATOMIC_COMMIT: u32 = 0xC03864C8;
pub const DRM_IOCTL_GEM_CLOSE: u32 = 0x40086409;

/// DRM Device Node Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmNodeType {
    PrimaryCard, // `/dev/dri/card0` - Modesetting + Display Output
    RenderNode,  // `/dev/dri/renderD128` - Unprivileged Headless Compute / GPGPU / Vulkan / Offscreen
}

/// GEM (Graphics Execution Manager) Buffer Object
#[derive(Debug, Clone)]
pub struct GemBufferObject {
    pub handle: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub pitch: u32,
    pub size_bytes: usize,
    pub offset: u64,
    pub ref_count: u32,
}

/// DRM CRTC (Cathode Ray Tube Controller) Display Pipeline
#[derive(Debug, Clone)]
pub struct CrtcPipeline {
    pub crtc_id: u32,
    pub active_fb_handle: Option<u32>,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub is_enabled: bool,
}

/// DRM Connector (HDMI / DisplayPort / eDP)
#[derive(Debug, Clone)]
pub struct DrmConnector {
    pub connector_id: u32,
    pub name: String,
    pub is_connected: bool,
    pub encoder_id: u32,
}

/// Atomic KMS Property Commit Request
#[derive(Debug, Clone)]
pub struct AtomicProperty {
    pub object_id: u32,
    pub property_id: u32,
    pub value: u64,
}

/// Atomic KMS State Commit Package
#[derive(Debug, Clone)]
pub struct AtomicKmsCommitState {
    pub flags: u32,
    pub properties: Vec<AtomicProperty>,
    pub fence_fd: Option<i32>,
}

/// Direct Rendering Manager (DRM) & Kernel Mode Setting (KMS) Core Subsystem Engine
pub struct DrmKmsSubsystemEngine {
    pub next_handle: u32,
    pub gem_buffers: HashMap<u32, GemBufferObject>,
    pub crtcs: Vec<CrtcPipeline>,
    pub connectors: Vec<DrmConnector>,
    pub active_commit: Option<AtomicKmsCommitState>,
}

impl DrmKmsSubsystemEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            next_handle: 1,
            gem_buffers: HashMap::new(),
            crtcs: Vec::new(),
            connectors: Vec::new(),
            active_commit: None,
        };

        // Initialize default display hardware pipelines
        engine.crtcs.push(CrtcPipeline {
            crtc_id: 100,
            active_fb_handle: None,
            width: 1920,
            height: 1080,
            refresh_rate: 60,
            is_enabled: true,
        });

        engine.connectors.push(DrmConnector {
            connector_id: 200,
            name: "DisplayPort-1".to_string(),
            is_connected: true,
            encoder_id: 300,
        });

        engine
    }

    /// Handles DRM character device `/dev/dri/card0` and `/dev/dri/renderD128` ioctl dispatches
    pub fn handle_drm_ioctl(
        &mut self,
        node_type: DrmNodeType,
        ioctl_cmd: u32,
        width: u32,
        height: u32,
        bpp: u32,
    ) -> Result<u32, &'static str> {
        match ioctl_cmd {
            DRM_IOCTL_VERSION => Ok(0x0100_0000), // Version 1.0.0
            DRM_IOCTL_MODE_GETRESOURCES => {
                if node_type == DrmNodeType::RenderNode {
                    return Err("Render node does not support mode resources");
                }
                Ok(self.crtcs.len() as u32)
            }
            DRM_IOCTL_MODE_CREATE_DUMB => {
                let pitch = width * (bpp / 8);
                let size_bytes = (pitch * height) as usize;
                let handle = self.next_handle;
                self.next_handle += 1;

                let offset = (handle as u64) * 0x0010_0000; // 1 MB alignment
                let buf = GemBufferObject {
                    handle,
                    width,
                    height,
                    bpp,
                    pitch,
                    size_bytes,
                    offset,
                    ref_count: 1,
                };

                self.gem_buffers.insert(handle, buf);
                Ok(handle)
            }
            DRM_IOCTL_MODE_MAP_DUMB => {
                let buf = self
                    .gem_buffers
                    .get(&width) // Here `width` parameter acts as `handle`
                    .ok_or("Invalid GEM handle")?;
                Ok((buf.offset & 0xFFFF_FFFF) as u32)
            }
            DRM_IOCTL_GEM_CLOSE => {
                self.gem_buffers.remove(&width);
                Ok(0)
            }
            DRM_IOCTL_MODE_ATOMIC_COMMIT => {
                if node_type == DrmNodeType::RenderNode {
                    return Err("Atomic commit not permitted on render node");
                }
                if let Some(crtc) = self.crtcs.first_mut() {
                    crtc.active_fb_handle = Some(width);
                }
                Ok(0)
            }
            _ => Err("Unsupported DRM ioctl command"),
        }
    }

    /// Submit Atomic KMS Commit State across all display CRTC pipelines
    pub fn commit_atomic_kms_state(&mut self, commit: AtomicKmsCommitState) -> Result<(), &'static str> {
        self.active_commit = Some(commit);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drm_kms_subsystem_create_and_map_dumb_buffer() {
        let mut drm = DrmKmsSubsystemEngine::new();

        let handle = drm
            .handle_drm_ioctl(DrmNodeType::PrimaryCard, DRM_IOCTL_MODE_CREATE_DUMB, 1920, 1080, 32)
            .unwrap();
        assert_eq!(handle, 1);

        let buf = drm.gem_buffers.get(&handle).unwrap();
        assert_eq!(buf.width, 1920);
        assert_eq!(buf.height, 1080);
        assert_eq!(buf.pitch, 1920 * 4);

        let offset = drm
            .handle_drm_ioctl(DrmNodeType::PrimaryCard, DRM_IOCTL_MODE_MAP_DUMB, handle, 0, 0)
            .unwrap();
        assert_eq!(offset, 0x0010_0000);
    }

    #[test]
    fn test_drm_render_node_isolation() {
        let mut drm = DrmKmsSubsystemEngine::new();

        let res = drm.handle_drm_ioctl(
            DrmNodeType::RenderNode,
            DRM_IOCTL_MODE_GETRESOURCES,
            0,
            0,
            0,
        );
        assert!(res.is_err());

        let handle = drm
            .handle_drm_ioctl(DrmNodeType::RenderNode, DRM_IOCTL_MODE_CREATE_DUMB, 512, 512, 32)
            .unwrap();
        assert_eq!(handle, 1);
    }

    #[test]
    fn test_drm_atomic_kms_commit() {
        let mut drm = DrmKmsSubsystemEngine::new();

        let commit = AtomicKmsCommitState {
            flags: 0x1,
            properties: vec![AtomicProperty {
                object_id: 100,
                property_id: 1,
                value: 1,
            }],
            fence_fd: Some(10),
        };

        drm.commit_atomic_kms_state(commit).unwrap();
        assert!(drm.active_commit.is_some());
    }
}
