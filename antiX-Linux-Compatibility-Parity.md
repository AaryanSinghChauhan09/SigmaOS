# antiX Linux Compatibility & Lightweight Init Parity in SigmaOS

## Overview

SigmaOS incorporates a clean-room compatibility subsystem modeled after **antiX Linux**, designed specifically for resource-constrained environments, ultra-fast boot sequences, and non-systemd init alternatives (`SysVinit`, `runit`).

***

## Key Modules

*   [`src/compatibility/antix.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/compatibility/antix.rs): antiX service manager, live USB persistence simulator, and lightweight package manager wrappers.
*   [`src/init/sigma_init.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/init/sigma_init.rs): Native zero-dependency init capable of microsecond process dispatch.

***

## Features

| Feature | antiX Concept | SigmaOS Native Implementation |
|---------|---------------|-------------------------------|
| **Live Persistence** | `antiX-live-persistence` | OverlayFS / B-Tree snapshotting in RAM with atomic flash sync |
| **Lightweight Init** | `runit` / `SysVinit` | Native async dependency graph without dbus/systemd daemon bloat |
| **Low-Memory Desktop** | `Rox-IceWM` / `Fluxbox` | Pure-Rust minimal desktop session consuming under 32MB RAM |
| **CLI Package Tools** | `cli-apt-i` | Native CLI package wizard in [`src/sigpkg/`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/) |

***

## Architecture

    Bootloader (sigma_boot_efi)
           │
           ▼
    [antiX Lightweight Init Engine] ──> Starts essential daemons with < 10MB RAM footprint
           │
           ▼
    [Live Persistence Controller] ────> Sets up ephemeral writable root overlay
           │
           ▼
    [Fast Userspace Session] ─────────> Spawns Zenith-Lite / antiX terminal workspace
