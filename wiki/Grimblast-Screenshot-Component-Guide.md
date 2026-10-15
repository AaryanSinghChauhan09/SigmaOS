# Grimblast Screencopy & Screenshot Capture Component Guide

## Component Overview
Governs Wayland screencopy protocol interaction, interactive rectangular region selection (slurp parity), display freeze canvases, and instant clipboard piping inspired by Omarchy Linux Grimblast.

## Linux Inspiration
grimblast shell wrapper, grim image grabber, slurp region selector, and wl-clipboard.

## BSD Inspiration
OpenBSD scrot/xwd capture tools, FreeBSD graphics capture utilities.

## Current SigmaOS Status
Area coordinate cropping and screenshot commands implemented in GrimblastScreenshotEngine and Zig sigma_grimblast_fast_capture.zig.

## Critical Missing Features
Direct in-memory libpng compression pipeline without temporary disk file creation, and built-in text OCR extraction.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_media_tools.rs, src/desktop/sigma_grimblast_fast_capture.zig, src/ipc/pipe.rs

## Testing Strategy
High-resolution 4K full-screen capture latency test (<30ms from hotkey to clipboard), area boundary clamp validation.

## Dependencies
WAYLAND_COMPOSITOR_AGENT, CLIPBOARD_MANAGER_AGENT, MEMORY_MANAGEMENT_AGENT

## Success Criteria
Sub-30ms capture-to-clipboard latency, pixel-perfect region crop, zero UI stutter during screen freeze.

## Open Source Competitors Analysis
Replaces fragile shell scripts chaining four separate CLI binaries with a single unified, memory-safe native Rust/Zig engine.

## Future Enhancements
Integrated AI OCR extracting text from captured screen rectangles directly into the system clipboard.
