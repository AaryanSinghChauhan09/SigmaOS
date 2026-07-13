# Security, Branch & Package Summary

> **Status**: 🔄 Active | **Scope**: `Repository Maintenance & Cleanup`

## 1. Executive Summary

To reduce repository bloat, minimize redundancy, and maintain a clear source of truth, SigmaOS is consolidating packages, merging overlapping branches, and ensuring all conceptual documentation resides solely in the GitHub Wiki.

This document serves as the master tracking page for these efforts.

---

## 2. Package Consolidation: `SigmaPkg`

SigmaOS is actively absorbing the behavior of various Linux package managers (`apt`, `pacman`, `dnf`, `nix`) into a single, unified compatibility layer known as **SigmaPkg**. 

Instead of duplicating scripts across the main repository for parsing `.deb`, `.rpm`, or `.pkg.tar.zst` files, `SigmaPkg` exposes a unified API.

| Target Manager | Status | SigmaPkg Subsystem | Notes |
| :--- | :--- | :--- | :--- |
| **APT** (Debian) | 🔄 In Progress | `sigmapkg-deb` | Emulates `dpkg` unpacking and `apt` dependency resolution. |
| **Pacman** (Arch) | 🔄 In Progress | `sigmapkg-alpm` | Supports `PKGBUILD` scripts and Arch repository parity. |
| **DNF** (Fedora) | 📋 Planned | `sigmapkg-rpm` | Handles `.rpm` formats and SELinux contexts. |
| **Nix** (NixOS) | ✅ Merged | `sigmapkg-nix` | Implements functional, content-addressed derivation store. |

**Action Taken**: All standalone bash scripts scattered throughout the repo for package management are being deprecated and merged into the `sigma-pkg` Rust binary.

---

## 3. Branch Cleanup & Consolidation

To maintain a clean Git history, feature branches are actively audited, tested, and merged into `main`. Once merged, the origin branches are deleted to prevent repository clutter.

### Recent Branch Audits
- `feat/sigma-fs-prototype` → **Merged** into `main`. Branch deleted.
- `feat/sigma-media-ai` → **Merged** into `main`. Branch deleted.
- `chore/docs-cleanup` → **Active**. Currently tracking migration of `.md` files to Wiki.

*Note: For the definitive list of merged Pull Requests, please see the GitHub repository PR tab. We enforce a linear commit history on `main`.*

---

## 4. Security Policy Synchronization

Security documentation is critical and must not diverge. 
- The `SECURITY.md` in the main repository is the **Single Source of Truth** for reporting vulnerabilities. 
- It contains our PGP public keys and security embargo policies.
- The Wiki only references `SECURITY.md` to prevent duplication.

*If you discover a vulnerability, please consult the `SECURITY.md` file in the root of the SigmaOS GitHub repository.*
