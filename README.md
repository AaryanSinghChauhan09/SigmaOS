# SigmaOS

Modular, memory-safe Linux desktop OS built from independent component
repositories. This repo is the integration root: workspace build orchestration,
core submodules, and the component registry.

Architecture and design documentation live in
[SigmaOS-docs](https://github.com/AaryanSinghChauhan09/SigmaOS-docs).

## Component repositories

### Phase 2 — Core (7)

- SigmaOS-kernel
- SigmaOS-filesystem
- SigmaOS-network
- SigmaOS-security
- SigmaOS-system
- SigmaOS-desktop
- SigmaOS-packages

### Phase 3 — Desktop layer (8)

- SigmaOS-compositor
- SigmaOS-window-manager
- SigmaOS-terminal
- SigmaOS-file-manager
- SigmaOS-text-editor
- SigmaOS-theme-engine
- SigmaOS-settings
- SigmaOS-app-framework

### Phase 4 — Components (146)

Full list of 146 component repositories:
[PHASE4-MANIFEST.md](.agents/tasks/phase4-components/PHASE4-MANIFEST.md)

### Docs (1)

- SigmaOS-docs

## Layout

- `COMPONENTS.toml` — registry of all components
- `.gitmodules` — seven core submodules
- `src_modules/` — submodule checkouts
- `Cargo.toml` — Rust workspace root
