# AI Agent Future Development Roadmap: Arch Linux Parity & Gap Closure

This specification details the strategic roadmap for AI Agent workflows targeting 100% functional parity between SigmaOS and Arch Linux distribution primitives.

---

## 1. Executive Summary & Vision

SigmaOS absorbs key operational innovations from Arch Linux (Simplicity, Modernity, Pragmatism, User Centrality) while replacing legacy C/C++ memory-unsafe components with zero-dependency, `#![no_std]` Rust primitives.

Key Target Subsystems:
* **ALPM (Arch Linux Package Management)** & `pacman` database transaction safety.
* **AUR (Arch User Repository)** RPC API & PKGBUILD parsing/sandboxing.
* **`makepkg` & `namcap`** build isolation and package linting.
* **`archinstall`** declarative guided installer parity.
* **`reflector`** dynamic mirrorlist ranking and PGP keyring web-of-trust verification.

---

## 2. Core Subsystem Architectural Blueprint

### 2.1 ALPM Database & Transaction Hook Runner
* **Database Format**: Local sync database (`/var/lib/pacman/local`) converted into content-addressed BTreeMap entries.
* **Transaction Hooks**: Pre-transaction and post-transaction hook execution ordering with priority queues (`PreTransaction`, `PostTransaction`).
* **Dependency Resolution**: Directed Acyclic Graph (DAG) solver for circular dependency resolution and orphan package cleanup (`pacman -Qtdq`).

### 2.2 AUR RPC Client & PKGBUILD Parser
* **AUR v5 RPC API**: HTTPS JSON query/search client with caching.
* **PKGBUILD Parser**: Safe AST evaluation for `pkgname`, `pkgver`, `pkgrel`, `arch`, `depends`, `makedepends`, `source`, `sha256sums`, and `build()` / `package()` functions.
* **Sandboxed Build Environment**: `chroot` and Landlock-isolated container execution during `makepkg` execution.

### 2.3 Reflector Mirrorlist & Keyring Verification
* **Reflector Engine**: Benchmark download latency, HTTPS availability, and completion sync status across global mirrors.
* **Pacman Keyring Engine**: Ed25519 and PGP signature verification for `.pkg.tar.zst` release tarballs with Web-of-Trust level enforcement (`Full`, `Marginal`, `Never`).

---

## 3. Implementation Roadmap Milestones

| Milestone | Target Quarter | Focus Area | Functional Objectives |
| :--- | :--- | :--- | :--- |
| **Phase 1** | Q4 2026 | ALPM & PKGBUILD | AST parser for PKGBUILD, ALPM hook transaction runner |
| **Phase 2** | Q1 2027 | AUR & Chroot | AUR v5 RPC search, `chroot` isolated `makepkg` builds |
| **Phase 3** | Q2 2027 | Keyring & Reflector | PGP/Ed25519 signature verification, Reflector mirror ranking |
| **Phase 4** | Q3 2027 | Archinstall Parity | Declarative JSON configuration guided installer wizard |

---

## 4. AI Agent Subsystem Autonomous Workflow

```
[AUR / Official Sync Repo]
         │
         ▼
[PKGBUILD AST Parser]
         │
         ▼
[Landlock Build Sandbox] ──(makepkg)──► [.pkg.tar.zst Archive]
                                                │
                                                ▼
[ALPM Hook Runner Engine] ◄──(Signify/PGP)── [Keyring Verifier]
```

AI Agents in SigmaOS continuously monitor package changes, evaluate PKGBUILD security linting with `namcap` rules, and execute sandboxed builds without root privileges.
