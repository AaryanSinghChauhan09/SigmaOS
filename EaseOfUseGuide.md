
# Ease of Use Guide: Simplicity in Sovereignty


SigmaOS balances power with an intuitive, friction-free experience for both developers and end-users.


## 1. Onboarding Wizard

New users are greeted by the **Sovereign Onboarding Wizard**, which guides them through initial lattice configuration, theme selection, and system hardening.


## 2. Centralized Control Center

The **Zenith Control Center** (`web_ui/scripts/modules/control_center.js`) provides a unified interface to monitor and manage all 33 suites. You can toggle shards, view vitals, and adjust system domains from a single hub.


## 3. Atomic Updates

Updates to the Sovereign Lattice are atomic. By using the **Declarative State Parser**, you can rollback to previous stable versions of the lattice state instantly if an update fails.


## 5. Sovereign App Store

Discover and install new shards via the **Zenith App Store** (`web_ui/scripts/modules/zenith_app_store.js`). This graphical interface for **SigmaPKG** allows you to expand the system's capabilities with a single click.


## 6. Auto-Tiling (Pop!_OS Style)

Maintain a clean and productive workspace with the **Window Tiler** (`web_ui/scripts/modules/window_tiler.js`). The system automatically tiles active windows across the canvas, ensuring that you can monitor multiple 33-suite shards simultaneously without overlapping.


## 7. Scriptable Extensions

Users can extend system functionality without touching C/ASM core logic. The **Lua Scripting Bridge** allows for lightweight, user-defined automation and UI tweaks.
