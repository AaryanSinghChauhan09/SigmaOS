# OSS Absorption: ReactOS & Wine — Win32 API Compatibility

> **Status**: 🔄 Active | **Source Projects**: ReactOS, Wine | **Target Shard**: `SigmaOS Win32 Subsystem`

---

## 1. Executive Summary

While SigmaOS is built on a Linux-compatible Sovereign Lattice, industrial users often rely on legacy Windows applications (CAD tools, proprietary PLC software, specialized data analysis). 

SigmaOS absorbs the **Win32 API compatibility layers** pioneered by ReactOS (kernel-level compatibility) and Wine/Proton (userspace API translation), integrating them into an isolated `sigma-win32` execution shard.

---

## 2. Key Features Absorbed

### 2.1 The `sigma-win32` Subsystem (Proton-inspired)

SigmaOS runs Windows applications natively, translating Win32/DirectX calls to POSIX/Vulkan calls at zero overhead.

```bash
# Launch a Windows executable seamlessly
$ sigma run legacy-app.exe
Σ [WIN32] Initializing Win32 Subsystem (Proton-compatible)...
  Mapping C:\ to /home/user/.wine/drive_c/
  Translating DirectX 11 → Vulkan (DXVK)
  Application 'legacy-app.exe' running (PID 890)
```

### 2.2 Sandboxing Win32

A major advantage of SigmaOS is that Win32 applications run fully sandboxed. They cannot see the native filesystem or other running applications.

```rust
// userland/win32/sandbox.rs
// SPDX-License-Identifier: MIT

pub fn spawn_win32(exe: &Path) -> Result<()> {
    let mut config = SandboxConfig::flatpak_default("win32-compat");
    
    // Restrict access to virtual C: drive only
    config.rw_mounts = vec![ PathBuf::from("/home/user/.sigma/win32_prefix/") ];
    
    // Drop all capabilities
    config.seccomp = SeccompProfile::Strict;

    BubblewrapBuilder::new()
        .config(&config)
        .exec(&["wine", exe.to_str().unwrap()])
        .spawn()
}
```

---

## 3. Architecture

```
┌────────────────────────────────────────────────────────────────┐
│               SIGMA-WIN32 COMPATIBILITY LAYER                  │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Windows Application (.exe)                               │  │
│  └──────────────────────┬───────────────────────────────────┘  │
│                         │ Win32 API calls                      │
│  ┌──────────────────────▼───────────────────────────────────┐  │
│  │ WINE / PROTON TRANSLATION LAYER                          │  │
│  │ kernel32.dll, user32.dll, gdi32.dll (reimplemented)      │  │
│  └──────┬──────────────────────────────────┬────────────────┘  │
│         │ POSIX syscalls                   │ DirectX           │
│  ┌──────▼──────────────────────────┐ ┌─────▼────────────────┐  │
│  │ SIGMA KERNEL (Linux compat)     │ │ DXVK (Vulkan)        │  │
│  └─────────────────────────────────┘ └──────────────────────┘  │
└────────────────────────────────────────────────────────────────┘
```

---

## 4. References & Standards

- WineHQ — `winehq.org` (LGPL-2.1)
- ReactOS — `reactos.org` (GPL-2.0)
- Valve Proton / DXVK — `github.com/ValveSoftware/Proton`
