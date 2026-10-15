# Night Light & Hardware Gamma Calibration Component Guide

## Component Overview
Governs smooth color temperature transitions (1000K to 10000K), solar twilight calculation, display hardware gamma LUT manipulation, and blue-light reduction inspired by Omarchy Linux Hyprsunset.

## Linux Inspiration
hyprsunset, wlsunset, and redshift gamma controllers.

## BSD Inspiration
OpenBSD wsdisplay color calibration, FreeBSD xgamma.

## Current SigmaOS Status
Color temperature stepping and hyprsunset command generation implemented in HyprsunsetNightLightEngine and Nim sigma_nightlight.nim.

## Critical Missing Features
Direct DRM/KMS hardware gamma LUT programming via libdrm, smooth multi-second ramp interpolation, and per-monitor color curves.

## Implementation Priority
MEDIUM (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_hypr_ecosystem.rs, src/desktop/sigma_nightlight.nim, src/graphics/drm_kms.rs

## Testing Strategy
Astronomical sunrise/sunset time synchronization tests, gamma curve ramp smoothness tests under 144Hz monitors.

## Dependencies
GRAPHICS_GPU_AGENT, LOCATION_GEOCLUE_AGENT, TIMEKEEPING_AGENT

## Success Criteria
Seamless, imperceptible color temperature shifts, zero frame drops during temperature adjustments.

## Open Source Competitors Analysis
Bypasses userspace X11 gamma hacks by writing directly to kernel DRM gamma lookup tables.

## Future Enhancements
Ambient light sensor integration dynamically matching display white point to room ambient lighting.
