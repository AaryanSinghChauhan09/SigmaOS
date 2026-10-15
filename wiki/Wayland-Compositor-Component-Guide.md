# Wayland Compositor & Display Server Component Guide

## Component Overview
Coordinates the Zenith desktop display server, Wayland protocol multiplexing, hardware plane composition (DRM/KMS), and GPU damage tracking.

## Linux Inspiration
wlroots library, Sway compositor, Hyprland dynamic tiling and animations, Weston reference compositor.

## BSD Inspiration
FreeBSD DRM kernel modules, OpenBSD xenocara and wsdisplay infrastructure.

## Current SigmaOS Status
Basic framebuffer rendering, software mouse cursor, and Wayland surface protocol headers implemented. Hardware DRM/KMS scanout needs finishing.

## Critical Missing Features
Direct DRM/KMS atomic modesetting, DMA-BUF zero-copy surface sharing, fractional scaling, dynamic window tiling layout engine, and tear-free VSync.

## Implementation Priority
HIGH (Phase 2)

## Key Files to Create/Improve
src/display/wayland.rs, src/display/drm.rs, src/display/compositor.rs, src/display/damage.rs

## Testing Strategy
Wayland protocol conformance suite, 144Hz tear-free VSync display test, multi-monitor hotplug verification.

## Dependencies
GRAPHICS_GPU_AGENT, INPUT_ACCESSIBILITY_AGENT, MEMORY_MANAGEMENT_AGENT

## Success Criteria
Rock-solid 120/144 FPS rendering without dropped frames, sub-5ms input-to-display latency, zero compositor crash crashes.

## Open Source Competitors Analysis
Replaces heavy X11/Xorg and bulky Mutter/KWin with an ultra-compact Rust-native Wayland compositor consuming <15MB RAM.

## Future Enhancements
Direct HDR10+ and Dolby Vision color pipeline with ICC profile hardware LUT mapping.
