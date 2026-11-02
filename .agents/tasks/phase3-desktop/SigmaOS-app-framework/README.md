# SigmaOS-app-framework

Shared library for SigmaOS desktop applications. Provides widgets, layout primitives, theming integration, accessibility support, and IPC.

## Upstream Inspiration

- [gtk4-rs](https://gtk-rs.org/) — GTK4 Rust bindings
- [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/) — GNOME adaptive widgets
- [tauri](https://tauri.app/) — app framework with IPC model
- [iced](https://iced.rs/) — Elm-inspired Rust GUI

## Purpose

- **Widgets**: button, label, entry, listbox, dialog with consistent API
- **Layout**: box layout, grid layout, responsive breakpoints
- **Theming**: integrates with SigmaOS-theme-engine
- **Accessibility**: ARIA-equivalent roles, screen reader support
- **IPC**: zbus D-Bus + custom IPC for inter-app communication
- **Async event loop**: GLib main loop integration

## Build Instructions

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev

cargo build --release
cargo test
```

## API Overview

- `Widget` trait — base for all UI components
- `LayoutBox` / `LayoutGrid` — composable layout containers
- `IpcChannel` — send/receive typed messages over D-Bus
- `AppContext` — top-level application state with theme + IPC
