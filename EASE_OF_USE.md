# Ease of Use & Extreme Modularisation

SigmaOS shatters the myth that a secure, bare-metal microkernel must be difficult to use. By combining **Sovereign Extensions (`s-ext`)**, **Application Sandboxing (`app_sandbox.c`)**, and **Declarative Manifests (`manifest_parser.c`)**, the system achieves unparalleled Developer Experience (DX) and User Experience (UX).

Located in `modules/ext/plugins/` and `modules/tools/cli/`.

## Competitive Advantages (USPs)

### 1. Zero-Configuration Sandboxing
- **Standard OS**: On Linux, to properly isolate an application, a user must configure complex `Docker` containers, `chroot` jails, or `AppArmor` profiles.
- **SigmaOS USP**: The kernel's Application Sandbox handles everything invisibly. When a user double-clicks an app, the OS automatically generates an isolated VFS namespace and negotiates a strictly enforced Memory Contract on the app's behalf. If the app tries to exceed its memory quota or reach out to the network without permission, the Sovereign Watchdog terminates it instantly. The user does *nothing*.

### 2. Manifest-Driven Automations
- **Standard OS**: Apps install background daemons or registry keys to launch at startup, define hotkeys, or change the UI theme, slowly bloating the system over time.
- **SigmaOS USP**: Third-party applications ship with a simple, declarative JSON manifest. When launched, the kernel's Manifest Parser reads it and dynamically wires the OS *around* the app. 
    - Need a global hotkey? The kernel temporarily registers it in Ring-0 and instantly unregisters it when the app closes. 
    - Prefer a dark theme? The Zenith UI Compositor shifts its palette temporarily while the app is focused.
    No permanent system bloat is ever created.

### 3. Ultimate Plugin Modularisation
- **Standard OS**: Customising the OS kernel requires deep C knowledge and recompilation.
- **SigmaOS USP**: With `s-ext`, any authorized application can inject logic directly into the UI render loop or the AI Scheduler's reward function. This turns the entire operating system into a modular, programmable playground, without sacrificing the security of the underlying capability architecture.
