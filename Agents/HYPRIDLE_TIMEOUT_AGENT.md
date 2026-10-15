# Hypridle Dynamic Idle Timeout Governor Component Agent

## Component Overview
Governs multi-stage system idle event detection, display brightness dimming, Wayland lock invocation, DPMS display power-down, and automatic system suspension inspired by Omarchy Linux Hypridle.

## Linux Inspiration
hypridle Wayland idle listener daemon, swayidle, and systemd-logind idle hints.

## BSD Inspiration
OpenBSD apm idle sleep triggers, FreeBSD devd ACPI power management events.

## Current SigmaOS Status
Multi-stage idle timeout evaluation implemented in HypridleTimeoutGovernor. Brightnessctl dimming and DPMS off command templates active.

## Critical Missing Features
Wayland ext-idle-notify-v1 protocol server integration, media playback active inhibitor detection, and smooth gamma fade-to-black.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_hypr_ecosystem.rs, src/power/acpi.rs, src/display/compositor.rs

## Testing Strategy
Time-accelerated synthetic idle event benchmarks, media player inhibitor bypass verification, resume latency tests.

## Dependencies
WAYLAND_COMPOSITOR_AGENT, POWER_MANAGEMENT_AGENT, TIMEKEEPING_AGENT

## Success Criteria
Sub-10ms idle trigger dispatch, zero false-positive locks while watching video or listening to audio.

## Open Source Competitors Analysis
Replaces fragile C++ idle daemons with a memory-safe, typed Rust governor communicating directly with the compositor.

## Future Enhancements
Webcam-assisted presence detection pausing idle timeouts when the user is looking directly at the screen.
