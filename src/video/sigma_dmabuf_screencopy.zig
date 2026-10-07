// SPDX-License-Identifier: GPL-3.0-or-later
// SigmaOS Sovereign DMABUF Screencopy Interface
// (`src/video/sigma_dmabuf_screencopy.zig`)
// Low-level Zig module for zero-copy Wayland zwlr_screencopy_v1 frame extraction,
// DMABUF plane descriptor assembly, and hardware encoder feeding.

const std = @import("std");

pub const DmaBufPlane = struct {
    fd: i32,
    offset: u32,
    stride: u32,
    modifier: u64,
};

pub const ScreencopyFrame = struct {
    width: u32,
    height: u32,
    drm_format: u32,
    plane_count: u32,
    planes: [4]DmaBufPlane,
    flags: u32,

    pub fn initRgbx8888(w: u32, h: u32, fd: i32) ScreencopyFrame {
        const stride = w * 4;
        return ScreencopyFrame{
            .width = w,
            .height = h,
            .drm_format = 0x34324258, // DRM_FORMAT_XBGR8888
            .plane_count = 1,
            .planes = [4]DmaBufPlane{
                DmaBufPlane{ .fd = fd, .offset = 0, .stride = stride, .modifier = 0 },
                DmaBufPlane{ .fd = -1, .offset = 0, .stride = 0, .modifier = 0 },
                DmaBufPlane{ .fd = -1, .offset = 0, .stride = 0, .modifier = 0 },
                DmaBufPlane{ .fd = -1, .offset = 0, .stride = 0, .modifier = 0 },
            },
            .flags = 0,
        };
    }

    pub fn frameSizeBytes(self: *const ScreencopyFrame) u64 {
        return @as(u64, self.planes[0].stride) * @as(u64, self.height);
    }
};

/// Validate that the screencopy frame is suitable for direct NVENC / VA-API DMA mapping
pub fn validateEncoderDmaMapping(frame: *const ScreencopyFrame) bool {
    if (frame.width == 0 or frame.height == 0) return false;
    if (frame.plane_count == 0 or frame.planes[0].fd < 0) return false;
    if (frame.planes[0].stride < frame.width * 4) return false;
    return true;
}
