# Distros Above SigmaOS: Architectural Synthesis

This specification details how SigmaOS structurally synthesizes and absorbs the defining value propositions of major Linux distributions into a single, cohesive operating system model.

---

## 🎨 Layout & Modular Synthesis Layer

SigmaOS does not copy external distribution code; instead, it implements cleanroom sovereign versions of their functionality mapping to system layers:

```
┌────────────────────────────────────────────────────────┐
│                   Zenith Desktop UI                    │
│   (Absorbs Zorin Appearance & Pantheon Workspaces)    │
├────────────────────────────────────────────────────────┤
│             sigpkg Package Management                  │
│   (Absorbs NixOS Immutability & Arch rolling release)  │
├────────────────────────────────────────────────────────┤
│                  SigmaOS Kernel                        │
│   (Absorbs Clear Linux SIMD & Debian LTS stability)    │
└────────────────────────────────────────────────────────┘
```

---

## 💾 Subsystem Absorption Map

| Distribution | Core Value Proposition | SigmaOS Sovereign Implementation |
| :--- | :--- | :--- |
| **Arch Linux** | Full customizability / rolling releases | Modular compile-time profile flags in `sigma.toml` |
| **NixOS** | Declarative system configuration | Read-only boot structures with transient `/sigma/store` CAS |
| **Clear Linux** | Highly-optimized performance loops | AVX-512 memory copy engines and EEVDF autotuners |
| **Zorin OS** | Intuitive desktop layouts | Sovereign Layout Engine with customizable desktop grids |
| **RescueZilla** | Sector-level cloning and backups | `sigma-recover` sector streaming encrypted backup files |

---

## ⚙️ Dynamic Profile Configuration

The system uses profiles to match the desired deployment type:
- **`sigma-core` (Arch-like minimal)**: Minimal runtime footprint, direct hardware console, no GUI.
- **`sigma-desktop` (Zorin-like UI)**: Active Zenith compositor, audio, standard input/output drivers.
- **`sigma-cloud` (NixOS-like immutable)**: Containers active, read-only system partitions, declarative GitOps.
