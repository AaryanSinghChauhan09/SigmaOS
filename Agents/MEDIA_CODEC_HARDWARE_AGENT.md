# Hardware Video Acceleration & Codec Component Agent

## Component Overview
Governs hardware-accelerated video decoding and encoding (H.264, HEVC, VP9, AV1) via VA-API and VDPAU across Intel, AMD, and NVIDIA hardware.

## Linux Inspiration
Linux VA-API (libva), VDPAU, FFmpeg hardware acceleration, and Mesa Gallium video decoders.

## BSD Inspiration
FreeBSD libva ports and Intel iHD media drivers.

## Current SigmaOS Status
VA-API driver probing, format negotiation, and basic H.264 NAL unit parsing implemented in media subsystem. AV1 decoding in progress.

## Critical Missing Features
Direct DMA-BUF surface import to Wayland compositor for zero-copy 4K 60FPS video playback, and hardware AV1 encoding.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/media/vaapi.rs, src/media/codecs.rs, src/video/sigma_dmabuf_screencopy.zig

## Testing Strategy
4K AV1 video playback CPU utilization benchmarks (<2% CPU overhead with hardware decode), latency tests.

## Dependencies
GRAPHICS_GPU_AGENT, HARDWARE_HAL_DRIVERS_AGENT, MEMORY_MANAGEMENT_AGENT

## Success Criteria
Smooth 4K/8K 60FPS video playback with <3% CPU utilization on supported hardware, zero frame drops.

## Open Source Competitors Analysis
Eliminates complex gstreamer pipeline bugs by using direct, zero-copy VA-API DMA-BUF streaming.

## Future Enhancements
Real-time AI video super-resolution upscaling processed directly on integrated GPU tensor cores.
