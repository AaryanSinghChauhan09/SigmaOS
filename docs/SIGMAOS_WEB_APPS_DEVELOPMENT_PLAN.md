# SigmaOS Web Applications & PWA Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-WebApps` is the native web application container, Site-Specific Browser (SSB), and Progressive Web App (PWA) management subsystem for **SigmaOS**. Inspired by the Linux Mint `webapp-manager` site-isolation model, the Fedora/RHEL Cockpit system administration web console, ChromeOS/Endless OS PWA container runtime, and OpenBSD/FreeBSD sandboxed WebKit process isolation, `Sigma-WebApps` bridges cloud productivity tools and local desktop workflows into secure, high-performance, native-feeling windowed applications.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Web App Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Linux Mint `webapp-manager`** | Isolated Site-Specific Browser (SSB) instances, custom app icons, navigation bar hiding, isolated cookie/cache profiles. | `src/media/browser_innovations_suite.rs` |
| **Fedora & RHEL Cockpit Web Console** | Secure WebSocket IPC system management portal, web-based system monitor, journald log viewer, package manager UI. | `web_ui/` & `src/tools/` |
| **ChromeOS & Endless OS PWA Runtime** | Progressive Web App (PWA) manifest parsing (`manifest.json`), offline Service Worker caching, WebAssembly (Wasm) sandbox execution. | `web_ui/` & `src/runtime/` |
| **Omarchy 4 & Zenith Wayland Portal** | Seamless integration with Zenith Wayland compositor, window tiling rules, native taskbar pin support, GTK/Qt portal IPC. | `zenith_desktop/` & `src/desktop/` |
| **OpenBSD & FreeBSD Web Sandboxing** | OpenBSD `pledge("stdio rpath wpath inet prot_exec")` and Landlock filesystem sandbox for web renderer processes, blocking local file theft. | `src/security/pledge.rs` & `src/security/landlock.rs` |

---

## 3. 5-Layer Web Applications Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Desktop Integration & Zenith Wayland Display Portal          │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: OpenBSD `pledge`/`unveil` & Landlock Web Renderer Sandbox     │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: PWA Service Worker, IndexedDB & Wasm Offline Engine          │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Cockpit-Inspired System Administration Web App Suite          │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Isolated Web App Container Runtime (SSB / WebKitGTK)          │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Isolated Web App Container Runtime (SSB / WebKitGTK)
- **Site-Isolation Profiles:** Each web application executes in a dedicated, isolated browser profile (`~/.config/sigma-webapps/profiles/<app_id>`).
- **Clean Window Frames:** Custom frameless or native Wayland window decorations with app-specific titlebar themes.
- **Custom User-Agent & Headers:** Tailored headers ensuring maximum compatibility with web suites (Google Workspace, Microsoft 365, Notion, Figma).

### Layer 2: Cockpit-Inspired System Administration Web App Suite
- **System Monitor Web UI:** Real-time CPU, GPU, memory, network, and disk usage dashboards rendered via WebSockets.
- **Package Manager Web Portal:** Graphical `Sigma-pkg` search, installation, and rollback interface.
- **Storage & Journald Dashboard:** Disk partition management, ZFS/Btrfs snapshot browser, and live system log auditor.

### Layer 3: PWA Service Worker, IndexedDB & Wasm Offline Engine
- **Service Worker Cache Manager:** Cache-first offline storage policies for PWAs, enabling offline document editing.
- **WebAssembly (Wasm) Engine:** Native-speed Wasm execution sandbox for client-side image editing, CAD, and document processing.

### Layer 4: OpenBSD `pledge`/`unveil` & Landlock Web Renderer Sandbox
- **Process Isolation:** Multi-process architecture separating Network, GPU, Renderer, and Storage processes.
- **Filesystem Restriction:** Unveiling only `~/Downloads` and app-specific cache profiles, blocking malicious web scripts from accessing `~/.ssh` or `/etc/passwd`.

### Layer 5: Desktop Integration & Zenith Wayland Display Portal
- **Desktop Portal IPC:** Standard xdg-desktop-portal implementation for FileChooser, OpenURI, ScreenCast, and Notifications.
- **Taskbar & Launcher Integration:** Auto-generating `.desktop` launcher files and pinning web apps to the Zenith taskbar.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | SSB Runtime | Implement isolated site-specific browser profile manager and Wayland window wrapper in `src/media/`. | Implemented |
| **Milestone 2** | Cockpit Web Admin | Implement WebSocket system telemetry gateway and web-based system monitor in `web_ui/`. | Implemented |
| **Milestone 3** | PWA & Offline Engine | Implement PWA manifest parser, Service Worker cache manager, and WebAssembly runtime integration. | Implemented |
| **Milestone 4** | Web Sandbox Security | Enforce OpenBSD `pledge`/`unveil` and Landlock filesystem restrictions on web renderer processes. | Implemented |
| **Milestone 5** | Wayland Portal Integration| Implement xdg-desktop-portal handlers for file picker, URI opening, and desktop notifications. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test runners in `src/media/browser_innovations_suite.rs` and `src/desktop/desktop_portal.rs`.
2. **Web Sandbox Security Verification:** Verifying sandboxed web renderer processes cannot read unpermitted filesystem paths.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.
