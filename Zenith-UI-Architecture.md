# Zenith Desktop: UI Architecture & Applications

This document outlines the Phase 3 implementations of the User Interface and Application Ecosystem for SigmaOS, operating entirely within our `no_std`, zero-allocation boundaries.

## 1. Object-Oriented UI Framework (`ui_core.rs`)
To provide a structured layout system without standard library Box or Vec allocations, we implemented a custom trait-based Widget framework.
- **`Widget` Trait**: Requires `draw`, `handle_event`, `set_bounds`, and `get_bounds`.
- **Layout Containers**: Structures like `HBox` act as layout calculators, storing fixed-size arrays of children indices to map to a global UI registry.
- **Event Handling**: Provides a unified enum `EventType` (e.g. `MouseClick`, `KeyPress`) that routes downward through the Widget hierarchy.

## 2. Zenith Compositor & Shell
The primary user experience is governed by the compositor and shell modules.
- **`zenith_desktop.rs`**: The session manager. It boots, loads the personalization profiles, initializes the Window Manager, and spins up the main VSYNC event loop.
- **`dash.rs`**: The floating dock/taskbar positioned at the bottom center of the screen, tracking active and pinned applications.
- **`launcher.rs`**: The application drawer. It integrates directly with the AI Task Engine (`sigma_llm_backend`) to provide semantic search and autocomplete as the user types.

## 3. Application Ecosystem
We converted several application stubs into functional `no_std` logic components.
- **App Store (`app_store.rs`)**: A graphical frontend interacting with the `sigpkg` binary backend via IPC, managing the installation state of Sovereign apps.
- **Sigma Logic (`sigma_logic.rs`)**: A visual node-based automation engine (an OS-level Zapier). It maps system events (like `TriggerFileCreated`) to automated actions (like `ActionRunAiPrompt`), creating a responsive, smart operating system.
- **Sigma DB (`sigma_db.rs`)**: A lightweight, embedded NoSQL Key-Value store using static memory arrays. It provides applications a unified API (`db_put`, `db_get`) to store states, tokens, and preferences locally without a hefty SQL engine.

*Last Updated: July 2026*
