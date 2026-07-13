# OSS Absorption: Flatpak & Flathub Application Distribution

> **Status**: 🔄 Active | **Source Project**: Flatpak 1.15 + Flathub | **Target Shard**: `SigmaOS Application Sandbox Layer`

---

## 1. Executive Summary

Flatpak is the standard desktop application sandboxing format for Linux, providing:
- **Bubblewrap** (`bwrap`) kernel namespace sandboxing per application
- **Portal** architecture for safe cross-sandbox communication (file access, screen sharing, notifications)
- **OCI-style layered runtimes** shared between applications to reduce disk usage
- **Flathub** as the world's largest curated Linux application registry

SigmaOS implements `sigma-flatpak` — a Flatpak-compatible layer using the `sigma-sandbox` isolation primitives and `sigma-portals` for app–OS communication, allowing SigmaOS to natively run all 2,000+ Flathub applications while maintaining the Sovereign Lattice's stronger security guarantees.

---

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                 SIGMA APPLICATION SANDBOX LAYER                 │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                  SIGMA PORTALS                          │    │
│  │  FileChooser │ Screenshot │ Notifications │ WebBrowser  │    │
│  │  Camera      │ Location   │ Print         │ OpenURI     │    │
│  └────────────────────────┬────────────────────────────────┘    │
│                           │ D-Portal protocol (DBus-free)       │
│  ┌────────────────────────▼────────────────────────────────┐    │
│  │               sigma-sandbox (bwrap-compatible)          │    │
│  │  Kernel namespaces: PID, NET, MNT, UTS, IPC, USER       │    │
│  │  Seccomp filter: allowlist of 60 syscalls               │    │
│  │  Wayland socket forwarding (no X11)                     │    │
│  │  PipeWire audio socket forwarding                       │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐    │
│  │  Firefox    │  │  GIMP       │  │  LibreOffice        │    │
│  │  (isolated) │  │  (isolated) │  │  (isolated)         │    │
│  └─────────────┘  └─────────────┘  └─────────────────────┘    │
│                 Shared Runtime: GNOME Platform 46               │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Features

### 3.1 Bubblewrap-Compatible Sandboxing (`sigma-sandbox`)

```rust
// userland/sandbox/bwrap.rs
// SPDX-License-Identifier: MIT

pub struct SandboxConfig {
    pub allow_network: bool,
    pub allow_display: bool,        // Wayland only
    pub allow_audio:   bool,        // PipeWire socket only
    pub ro_mounts:     Vec<PathBuf>, // Read-only bind mounts into sandbox
    pub rw_mounts:     Vec<PathBuf>, // Read-write (e.g., ~/.var/app/<id>/)
    pub seccomp:       SeccompProfile,
    pub env:           HashMap<String, String>,
}

impl SandboxConfig {
    /// Create a Flatpak-equivalent sandbox for this application
    pub fn flatpak_default(app_id: &str) -> Self {
        Self {
            allow_network: true,
            allow_display: true,
            allow_audio:   true,
            ro_mounts: vec![
                PathBuf::from("/usr"),
                PathBuf::from("/etc/ld.so.cache"),
            ],
            rw_mounts: vec![
                PathBuf::from(format!("/home/user/.var/app/{app_id}")),
            ],
            seccomp: SeccompProfile::FlatpakCompat,
            env: Default::default(),
        }
    }

    pub fn spawn(&self, exec: &[&str]) -> Result<Child> {
        BubblewrapBuilder::new()
            .config(self)
            .exec(exec)
            .spawn()
    }
}
```

### 3.2 Portal System (`sigma-portals`)

Portals allow sandboxed apps to safely access system resources through user-visible prompts:

```bash
# When a sandboxed app calls xdg-open or file picker:
# → sigma-portals shows a native dialog
# → User grants or denies
# → Result passed back to sandboxed app

# Portal permissions are persisted per-app:
$ sigma portals list firefox
Σ [INFO] firefox — Portal Permissions:
  FileChooser    : ✅ Allowed (user-approved)
  Notifications  : ✅ Allowed
  WebBrowser     : ✅ Default browser
  Camera         : ❌ Denied
  Location       : ❌ Denied
  Screenshot     : 🔔 Ask each time

# Revoke a portal permission
$ sigma portals revoke firefox Notifications
Σ [SUCCESS] Revoked Notifications for firefox
```

### 3.3 Shared Runtimes (OCI Layer Cache)

Multiple apps share a single runtime OCI layer (e.g., GNOME Platform 46), saving disk space:

```bash
$ sigma app list-runtimes
Σ [INFO] Installed Runtimes:
  sigma.runtime.gnome.46      (980MB) — used by: Firefox, GIMP, Inkscape
  sigma.runtime.kde.6.2       (840MB) — used by: Kdenlive, Krita
  sigma.runtime.freedesktop.24 (420MB) — used by: VLC, gThumb

$ sigma app install --from flathub org.mozilla.firefox
Σ [PKG] Installing Firefox from Flathub...
  Runtime: sigma.runtime.gnome.46 (already installed)
  App delta: 87MB (runtime shared — not re-downloaded)
  Σ Done in 12s
```

### 3.4 Flathub Integration

```bash
$ sigma app search "video editor"
Σ [SEARCH] Flathub results:
  kdenlive       (org.kde.kdenlive)    ⭐4.8  — Professional video editor
  Shotcut        (org.shotcut.Shotcut) ⭐4.5  — Cross-platform video editor
  OpenShot       (org.openshot.OpenShot) ⭐4.2

$ sigma app install org.kde.kdenlive
$ sigma app uninstall org.kde.kdenlive
$ sigma app update                     # Update all installed apps
```

---

## 4. Security Enhancements Over Upstream Flatpak

| Feature | Upstream Flatpak | SigmaOS Enhancement |
|:--------|:----------------|:--------------------|
| Sandboxing | bubblewrap (bwrap) | sigma-sandbox (Rust, audited) |
| D-Bus | Full D-Bus session bus | sigma-ipc portal bridge (no D-Bus) |
| Seccomp policy | 200 allowed syscalls | 60 allowed syscalls (strictest) |
| Network isolation | None | Per-app firewall rules via sigma-net |
| Filesystem | ~/.var/app/<id> | Content-addressed storage (tamper-evident) |

---

## 5. References & Standards

- Flatpak — `flatpak.org` (LGPL-2.1)
- Bubblewrap — `github.com/containers/bubblewrap` (LGPL-2.0)
- XDG Desktop Portals — `flatpak.github.io/xdg-desktop-portal` (LGPL-2.1)
- Flathub — `flathub.org`
