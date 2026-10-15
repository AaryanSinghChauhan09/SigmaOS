# MangoHud Gaming Performance Overlay Component Guide

## Component Overview
Provides hardware-accelerated Vulkan and OpenGL in-game performance overlays displaying real-time FPS, frame timing graphs, GPU/CPU temperatures, VRAM usage, and GameMode governor controls inspired by Omarchy Linux.

## Linux Inspiration
MangoHud Vulkan overlay layer, Feral Interactive GameMode CPU governor switcher, and DXVK HUD.

## BSD Inspiration
FreeBSD Linuxulator gaming configurations, OpenBSD hw.sensors telemetry.

## Current SigmaOS Status
Config generator, FPS limiter logic, and hardware sensor telemetry tracking implemented in OmarchyMangoHudGamingEngine.

## Critical Missing Features
Direct Vulkan overlay layer hook (`VK_LAYER_MANGOHUD`), frame-pacing variance analysis, and automated GPU clock locking.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/distro/omarchy_linux_apex_engine.rs, src/distro/omarchy_gaming_enhanced.rs, src/graphics/drm_kms.rs

## Testing Strategy
Vulkan 1.3 frame pacing benchmarks under high render load, FPS limiter accuracy tests (stable 60/120/144 FPS).

## Dependencies
GRAPHICS_GPU_AGENT, SCHEDULER_AGENT, POWER_MANAGEMENT_AGENT

## Success Criteria
Zero frame-time stutter introduced by overlay rendering, accurate sub-millisecond frame pacing telemetry.

## Open Source Competitors Analysis
Integrates GameMode and MangoHud into a unified Rust engine, eliminating daemon inter-process communication latency.

## Future Enhancements
Autonomous frame-generation and dynamic resolution scaling governor adjusting game settings to maintain target FPS.
