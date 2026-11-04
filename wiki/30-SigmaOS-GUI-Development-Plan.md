# SigmaOS GUI Development Plan

## Scope and operating rules
This document defines the comprehensive architecture and development plan for the SigmaOS GUI subsystem.
It details the Wayland protocol model, DRM/KMS atomic modesetting pipeline, freedesktop desktop interfaces,
and WCAG 2.2 accessibility guidance for window management, compositing, and user experience.

## Design and Acceptance Rules
1. **Wayland Client Buffer Isolation**: Strict validation of SHM/DMABUF buffer mappings, ownership tracking, and damage regions.
2. **DRM/KMS Atomic Modesetting**: Transactional state validation for CRTCs, planes, connectors, and output configurations.
3. **Assistive Technology Bridge**: Semantic trees (role/name/value) exposed via accessible IPC compatible with screen readers and AT-SPI.
4. **Typed Portal Access**: Sandboxed permission mediation for file chooser dialogs, screen recording, and system settings.

## Proposed 19-PR Sequence Roadmap
1. `PR 01`: `AuditBaseline` - Define GUI scope, session architecture, and support status
2. `PR 02`: `AuditBaseline` - Establish DRM/KMS capability probes and surface protocols
3. `PR 03`: `BootableDesktopSlice` - Connect DRM atomic modeset pipeline to framebuffer discovery
4. `PR 04`: `BootableDesktopSlice` - Implement Wayland display compositor loop and client socket listener
5. `PR 05`: `BootableDesktopSlice` - Add SHM buffer sharing and zero-copy damage tracking
6. `PR 06`: `BootableDesktopSlice` - Implement XDG shell surface roles (toplevel, popup) and window lifecycle
7. `PR 07`: `BootableDesktopSlice` - Integrate input routing for keyboard, pointer, and touch gestures
8. `PR 08`: `BootableDesktopSlice` - Establish default fallback cursor theme and visual viewport renderer
9. `PR 09`: `UsableCoreUx` - Implement desktop shell panel with status indicators and app launcher
10. `PR 10`: `UsableCoreUx` - Add notification daemon surface with bubble stacking and actions
11. `PR 11`: `UsableCoreUx` - Add window manager decoration framing, titling, and resize borders
12. `PR 12`: `UsableCoreUx` - Support workspace tiling, floating layers, and fullscreen transitions
13. `PR 13`: `SecurityAndAccessibility` - Implement client surface sandboxing and isolated buffer sharing
14. `PR 14`: `SecurityAndAccessibility` - Implement AT-SPI semantic node tree bridge and accessibility focus
15. `PR 15`: `SecurityAndAccessibility` - Implement typed portal access mediator for file chooser and screen capture
16. `PR 16`: `SecurityAndAccessibility` - Add WCAG 2.2 high-contrast themes and dynamic text scaling
17. `PR 17`: `HardwareAndReleaseReadiness` - Add multi-monitor layout configuration and hotplug event handling
18. `PR 18`: `HardwareAndReleaseReadiness` - Integrate GPU hardware acceleration via Mesa/Virgl3D/Vulkan
19. `PR 19`: `HardwareAndReleaseReadiness` - Publish GUI developer guide, compositor reference, and maturity matrix

## Periodic Maintenance PR Cadences
- **Weekly**: Monitor Wayland protocol extensions and dependency security advisories.
- **Bi-weekly**: Run automated layout rendering fuzzers and input event fuzz tests.
- **Monthly**: Review WCAG 2.2 accessibility conformance and AT-SPI bridge performance.
- **Quarterly**: Benchmark multi-monitor atomic modeset latency and GPU frame pacing.

## Component Rules
- Every graphical component must implement graceful fallback when hardware acceleration is unavailable.
- Client surface crashes must not crash the compositor master loop.
- No direct unvalidated pointer dereferences across client-compositor memory boundaries.

## PR Review and Merge Gates
- All PRs must compile cleanly in `no_std` environments with `alloc`.
- Integration and regression test suites must pass 100% deterministically.
- Documentation and specification contracts must remain synchronized.

## Rollout and Exit Criteria
- Compositor launches reliably from bootloader handover into graphical user session.
- Seamless multi-monitor configuration with zero screen tearing via DRM/KMS atomic commits.
- Full parity with modern desktop environments while maintaining lightweight kernel footprint.
