# ❄️ Declarative Package Management & Hermetic Builds

SigmaOS supports NixOS-inspired declarative configuration, hermetic content-addressed stores (CAS), and reproducible build environments.

---

## 1. Nix Flake & DSL Expression Engine (`src/sigpkg/nix_dsl.rs`)

* **Nix Expressions (`NixExpr`):** Native Rust AST evaluator for Nix derivation specifications (`NixDerivationSpec`).
* **Flake Lockfile Verification (`NixFlakeLockfile`):** Verifies input commit hashes and Merkle tree roots against `flake.lock` to guarantee zero-drift builds.
* **Hermetic CAS Store (`NixOsHermeticCasStore`):** Content-Addressed Store (`/nix/store/<hash>-<name>`) ensuring reproducible build closures without external system leaks.

---

## 2. NixOS-Style Profiles & Atomic Generation Switching (`src/sigpkg/universal_oop_system.rs`)

* **Sovereign Profile Manager (`SovereignProfileManager`):** Manages userland profiles with atomic generation symlink pointers (`/nix/var/nix/profiles/per-user/<user>/profile-<gen>`).
* **Atomic Rollback:** Reverts profile symlink pointers back to any historical generation instantly without re-downloading packages.

---

## 3. Portage USE Flags & Dynamic Dependencies (`src/sigpkg/portage.rs` & `src/sigpkg/gentoo_use_flags.rs`)

* **USE Flag Manager (`UseFlagManager`):** Configures compile-time features (`ssl`, `wayland`, `dbus`, `X`).
* **Dynamic Dependency Solver (`GentooEbuildUseFlagSolver`):** Dynamically expands or shrinks required package dependencies based on active USE flag profiles.
