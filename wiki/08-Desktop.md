# Desktop

**Capability state: Prototype.** SigmaOS has no verified boot-to-graphical-session path. This page is the canonical home for desktop status, UX references, validation, and the desktop roadmap. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository includes desktop-related configuration and UI models, including a command-palette/launcher model. These are not an integrated desktop session: there is no verified compositor-to-display path, login flow, window management runtime, or supported desktop application set. Proposed names, shortcuts, settings, and shell commands in older plans are not available SigmaOS commands.

The launcher model supports in-memory filtering/ranking and has standalone tests. It does not launch applications or connect to a running desktop. See [Software Store and Command Palette](Software-Store-and-Command-Palette.md) for its specific status.

## Design references

- **Omarchy:** keyboard-first navigation, visible shortcuts, coherent defaults, and quick access to common developer workflows.
- **Linux Mint:** clear first-session setup, familiar controls, accessible defaults, and recoverable user-facing flows.
- **Arch Linux:** explicit configuration and concise, task-focused documentation.

These are design references, not SigmaOS capability claims.

## Roadmap

1. Establish a real display, input, and session path on the supported QEMU profile.
2. Add a keyboard-operable launcher connected to actual installed applications; report unavailable apps clearly.
3. Build first-session setup for display, input, networking, help, and recovery.
4. Add accessible window management, settings, notifications, and power controls only as backed services become available.
5. Validate clean install, keyboard-only use, screen scaling, session failure, and recovery before calling the desktop supported.

**Completion evidence:** a clean installation reaches a usable graphical session, core workflows work with keyboard and pointer, accessibility behavior is checked, and failure/recovery steps are reproducible.

## Related pages

- [Boot and installation](01-Installation.md)
- [Packaging and updates](09-Packaging.md)
- [Audio and graphics](Audio-and-Graphics.md)
- [Software Store and Command Palette](Software-Store-and-Command-Palette.md)
