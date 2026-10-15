# Wlogout Session Management & Power Menu Component Guide

## Component Overview
Provides a sleek Wayland session menu overlay offering lockscreen, logout, suspend, hibernate, reboot, and shutdown actions with smooth background blur and keyboard shortcuts inspired by Omarchy Linux.

## Linux Inspiration
wlogout GTK/Wayland logout menu, swaylock-effects screen locker, and systemd logind power management.

## BSD Inspiration
OpenBSD apm power control, FreeBSD shutdown and acpiconf commands.

## Current SigmaOS Status
Action dispatcher for lock/logout/suspend/reboot/shutdown implemented in OmarchyWlogoutSessionEngine. Blurred backdrop rendering supported.

## Critical Missing Features
Full PAM-authenticated lockscreen fallback, biometric fingerprint resume unlock, and session state persistence.

## Implementation Priority
MEDIUM (Phase 1)

## Key Files to Create/Improve
src/distro/omarchy_linux_apex_engine.rs, src/desktop/panel.rs, src/power/acpi.rs

## Testing Strategy
Power menu invocation latency tests (<15ms response), clean session termination and daemon shutdown verification.

## Dependencies
INIT_SUPERVISOR_AGENT, SECURITY_AGENT, POWER_MANAGEMENT_AGENT

## Success Criteria
Instantaneous session menu rendering, zero orphaned processes upon logout, reliable suspend and resume cycles.

## Open Source Competitors Analysis
Replaces heavy C/GTK wlogout with a lightweight safe Rust engine that hooks directly into the SigmaOS supervisor.

## Future Enhancements
Emergency lock mode that immediately purges memory encryption keys from CPU registers upon suspicious physical tampering.
