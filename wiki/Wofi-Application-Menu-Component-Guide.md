# Wofi Application Menu & Dmenu Provider Component Guide

## Component Overview
Provides a highly customizable, CSS3-styleable application launcher, window switcher, and dmenu script runner optimized for Wayland compositors inspired by Omarchy Linux Wofi.

## Linux Inspiration
wofi GTK/Wayland menu, rofi-wayland fork, and fuzzel minimal launcher.

## BSD Inspiration
OpenBSD dmenu simplicity, FreeBSD wofi ports.

## Current SigmaOS Status
Application menu categorization and dmenu stdin/stdout pipe mode functional in desktop toolkit. CSS parser active.

## Critical Missing Features
Fuzzy matching score normalization across mixed desktop entries and terminal shell history, and sub-pixel scrolling.

## Implementation Priority
MEDIUM (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_media_tools.rs, src/desktop/panel.rs, src/desktop/zenith.rs

## Testing Strategy
Dmenu piping benchmark with 50,000 text lines, CSS style reload latency tests.

## Dependencies
WAYLAND_COMPOSITOR_AGENT, IPC_AGENT, DESKTOP_ENVIRONMENT_AGENT

## Success Criteria
Sub-2ms menu popup latency, silky-smooth keyboard navigation, zero dropped keystrokes during rapid typing.

## Open Source Competitors Analysis
Provides complete styling freedom without the memory overhead of Electron-based desktop menus.

## Future Enhancements
Context-aware search prioritizing recently edited files and active browser tabs.
