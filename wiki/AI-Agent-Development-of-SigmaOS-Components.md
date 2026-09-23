# AI Agent Development of SigmaOS Components

This page documents the guidelines, architecture, and practices for AI agents developing, maintaining, and enhancing components of SigmaOS.

## Overview

SigmaOS leverages AI agents (such as Jules, Ori, Hermes, and OpenClaw) to continuously develop, audit, refactor, and verify system components across kernel, driver, userland, init system, package management, security, and desktop environment boundaries.

## Core Architectural Directives

1. **Zero-Dependency Core & Memory Safety**:
   - All core components in `src/` must prioritize pure Rust implementations with minimal or zero external unsafe C dependencies.
   - Reduced reliance on C/C++ libraries through native Rust reimplementations.

2. **Linux & BSD Feature Parity & Superiority**:
   - Universal package management (`SIGPKG`) supporting 89+ package formats (Debian, RPM, Pacman, Alpine, Gentoo, Void, FreeBSD, OpenBSD, NetBSD, Nix, Guix, Flatpak, Snap, AppImage, Mobile, Windows, Language packages).
   - Multi-OS security isolation combining Linux LSM/Landlock, FreeBSD Capsicum, OpenBSD Pledge/Unveil, and Qubes OS domain isolation.
   - Advanced storage & filesystem engine integrating OpenZFS ARC, Btrfs Snapper, DragonFly HAMMER2, and OSTree transactional commits.

3. **Spaced CLI Router & Developer Workflows**:
   - Flat executable namespace (`bin/omarchy-*`) mapped to spaced user commands (`omarchy theme set`, `omarchy network band`).
   - Header comment metadata parsing (`# omarchy:group`, `# omarchy:name`, `# omarchy:summary`, `# omarchy:hidden`, `# omarchy:args`).
   - Developer Tmux IDE layout generators (`tdl`, `tds`, `tdlm`, `tsl`).

4. **Idempotent Atomic Migrations**:
   - One-time repair scripts in `migrations/*.sh` tracked per-user under `~/.local/state/omarchy/migrations/`.
   - File mode `0644`, strict `bash -euo pipefail` safety, and non-zero exit halting to guarantee system state consistency.

5. **Self-Synchronization & Documentation Transfer**:
   - When a component or feature from an `.md` specification is fully implemented and verified via `./run_sigma_tests.sh`, its documentation is synchronized to `wiki/` and tracked in `src/sovereign_wiki_master_engine.rs`.

## Development & Verification Workflow

1. **Exploration & Planning**: Analyze requirements, check existing implementations, and set execution plan.
2. **Implementation & Verification**: Modify source code, avoiding direct artifact editing. Confirm every change using read-only inspection tools.
3. **Automated Testing**: Run `./run_sigma_tests.sh` to ensure workspace-wide test runners pass cleanly.
4. **Pre-Commit Checks**: Execute pre-commit instructions, review code changes, and submit via Pull Requests.
