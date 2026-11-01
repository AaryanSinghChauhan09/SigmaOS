# SigmaOS — Sovereign AI-Native Operating System

> **Current Status:** v0.1 Alpha — Active Development
> **Maintainer:** [AaryanSinghChauhan09](https://github.com/AaryanSinghChauhan09)
> **Architecture:** Multi-repo, 149 components
> **Primary Language:** Rust (kernel, system, desktop), C (firmware/drivers), Python (tooling), Shell (scripts)

---

## What is SigmaOS?

SigmaOS is a **sovereign, AI-native operating system** built from the ground up to give users complete ownership of their computing environment. It is not a respin, not a fork, and not a wrapper — it is a clean-slate, principle-driven Linux distribution inspired by the composability of **Linux Mint**, the philosophy of **Omarchy**, and the sovereignty ethos of modern open-source distros.

SigmaOS answers a single question: *What would a modern operating system look like if it were designed today, for users who value freedom, transparency, and AI-augmented workflows?*

### Core Goals

| Goal | Description |
|------|-------------|
| **Sovereign** | No telemetry, no forced cloud, no locked-in vendor. The user owns the OS. |
| **AI-Native** | AI is a first-class citizen — not bolted on, but integrated into the desktop, shell, and app layer. |
| **Modular** | 149 independent component repositories. Swap, patch, or replace any layer without rebuilding the whole system. |
| **Secure by Default** | Least-privilege processes, signed updates, verified boot, memory-safe kernel modules in Rust. |
| **User Freedom** | FOSS-only stack (practical licensing where necessary). No binary blobs without user consent. |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                    APP LAYER                        │
│   sigma-browser · sigma-files · sigma-terminal      │
│   sigma-ai-assistant · sigma-settings · sigma-store │
├─────────────────────────────────────────────────────┤
│                  DESKTOP LAYER                      │
│   sigma-shell · sigma-wm · sigma-panel · sigma-dm   │
│   sigma-theme · sigma-notifications · sigma-search  │
├─────────────────────────────────────────────────────┤
│                  SYSTEM LAYER                       │
│   sigma-init · sigma-pkg · sigma-update · sigma-net │
│   sigma-audio · sigma-input · sigma-display         │
├─────────────────────────────────────────────────────┤
│                  KERNEL LAYER                       │
│   sigma-kernel · sigma-drivers · sigma-fs           │
│   sigma-mm · sigma-sched · sigma-ipc · sigma-sec    │
├─────────────────────────────────────────────────────┤
│                 FIRMWARE / BOOT                     │
│   sigma-bootloader · sigma-uefi · sigma-initrd      │
└─────────────────────────────────────────────────────┘
```

---

## Quick Links

| Section | Description |
|---------|-------------|
| [Architecture](Architecture.md) | Full system architecture, ABI contracts, IPC model |
| [Principles](Principles.md) | All 90+ design principles, definitions, and SigmaOS application |
| [Component Map](Component-Map.md) | All 149 component repositories with metadata |
| [Roadmap](Roadmap.md) | 10-phase implementation plan with status |
| [Contributing](Contributing.md) | How to contribute, branch model, PR process |
| [Distro Comparison](Distro-Comparison.md) | SigmaOS vs Linux Mint, Omarchy, Arch, Ubuntu, Fedora |
| [Gap Analysis](Gap-Analysis.md) | Current gaps vs targets across all domains |
| [Branch Policy](Branch-Policy.md) | Branch naming, merge rules, cleanup policy |
| [Release Process](Release-Process.md) | Versioning, RC process, ISO signing, announcements |
| [Security Policy](Security-Policy.md) | Threat model, CVE process, responsible disclosure |
| [Hardware Support](Hardware-Support.md) | Supported hardware matrix, GPU/Wi-Fi/audio |

---

## Current State — v0.1 Alpha

SigmaOS v0.1 Alpha represents the **product definition and architectural foundation** phase. At this stage:

- All 149 component repositories have been defined and structured
- Core principles are codified and govern all design decisions
- Architecture documents, ABI contracts, and IPC models are drafted
- The bootloader integration path (GRUB2/systemd-boot) is chosen
- Rust kernel module scaffolding is in place
- No stable ISO is available yet — this is a pre-release engineering phase

### What Works
- Repository structure and naming convention
- Build system scaffolding (Cargo workspace + Makefile)
- Kernel configuration baseline (Linux 6.x with Rust modules enabled)
- Principle documentation and governance model

### What Doesn't Work Yet
- No installable ISO
- No working desktop environment
- No package manager integration
- AI assistant is specification-only

---

## Phase Roadmap (Summary)

| Phase | Name | Status |
|-------|------|--------|
| 1 | Product Definition | ✅ In Progress |
| 2 | Bootable Base | 🔲 Planned |
| 3 | Installer | 🔲 Planned |
| 4 | Desktop | 🔲 Planned |
| 5 | Default App Bundle | 🔲 Planned |
| 6 | Secure Updates | 🔲 Planned |
| 7 | Recovery / Rollback | 🔲 Planned |
| 8 | Hardware Validation | 🔲 Planned |
| 9 | QA and RC | 🔲 Planned |
| 10 | Stable Release | 🔲 Planned |

See [Roadmap](Roadmap.md) for full phase details, tasks, and deliverables.

---

## Design Philosophy

SigmaOS is built on **over 90 named principles** — from classic software engineering (SOLID, DRY, KISS) to OS-specific disciplines (Memory Discipline, Interrupt Handling, Direct Device Control) to community values (Meritocracy, Respect, User Freedom). Every design decision traces back to at least one principle.

See [Principles](Principles.md) for the full catalog.

---

## Get Involved

- **Discussions:** GitHub Discussions on the main repository
- **Issues:** File bugs and feature requests on the relevant component repository
- **Contributing:** Read [Contributing.md](Contributing.md) before opening a PR
- **Governance:** SigmaOS uses a meritocratic maintainer model — see [Contributing.md](Contributing.md)

---

*SigmaOS is free and open-source software. See each component repository for its specific license.*
