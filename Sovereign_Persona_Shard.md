# Sovereign Persona Shard

**Parity:** macOS Configuration Profiles · Android Work Profiles · Linux PAM  
**Location:** `kernel/modules/core/SovereignPersonaShard.c`  
**Standard:** Zenith Industrial Sovereignty v1.0

---

## Overview

The Sovereign Persona Shard provides native, zero-dependency multi-user personalisation for SigmaOS. Each Persona is a full silicon context — encompassing a unique `UID`, capability bitmask, aesthetic theme, and ANSI shell prompt — that can be atomically switched without process restart. This absorbs the USPs of macOS Configuration Profiles, Android Work Profiles, and Linux PAM.

---

## Architecture

```
Sovereign Persona Matrix (up to 8 concurrent contexts)
  ├── Zenith_Admin   — Full capability ring (0xFFFFFFFF) | Obsidian theme
  ├── Citizen_Dev    — Developer sandbox ring            | Aurora theme
  └── Guest_Secure   — Read-only minimal ring            | Frost theme

Context Switch Engine
  └── Atomic deactivation of previous context → activation of target
      — No process restart required
      — Capability mask enforced at dispatch time
```

---

## CLI Reference — `sigma-persona`

| Sub-command | Action |
|---|---|
| `sigma-persona create <name> <theme> <uid> <cap_mask>` | Create a new silicon persona context |
| `sigma-persona switch <name>` | Atomically switch the active persona |
| `sigma-persona audit` | Display all personas with UID, theme, cap-mask, and state |

---

## Built-in Personas

| Name | UID | Theme | Cap Mask |
|---|---|---|---|
| `Zenith_Admin` | 0 | Obsidian | `0xFFFFFFFF` |
| `Citizen_Dev` | 1000 | Aurora | `0x0FFF0000` |
| `Guest_Secure` | 9999 | Frost | `0x00000001` |

---

## Design Philosophy

- **Zero External Dependency**: No PAM modules, no dbus, no uid-map userspace tools.
- **Atomic Context Switching**: Single-instruction deactivate/activate cycle.
- **Capability Federation**: Each persona's accessible shard commands are governed by its bitmask.
- **Aesthetic Sovereignty**: Per-persona ANSI themes are applied at the silicon shell layer.

---

## Synchronization State

`GLOBAL MESH ACTIVE` — Synchronized with `AaryanSinghChauhan09/SigmaOS`.
