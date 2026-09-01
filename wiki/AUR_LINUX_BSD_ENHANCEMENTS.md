# SigmaOS AUR Subsystem: Linux & BSD Parity Specification

## Overview
SigmaOS enhances the traditional Arch User Repository (AUR) model by integrating best-of-breed paradigms from BSD distributions (FreeBSD Ports, OpenBSD security isolation) and Linux distributions (Gentoo Portage USE flags, Arch `namcap` linting, FreeBSD `poudriere` clean builds).

---

## Key Subsystem Architecture

### 1. Isolated Build Sandboxing (`AurBuildSandbox`)
- **Parity Inspirations**: FreeBSD `poudriere` clean chroot building, OpenBSD `pledge()` and `unveil()` system call & file path restrictions.
- **Security Protections**:
  - Restricts path access to temporary build directories (`/tmp/build`, `/var/cache/sigma_pkg`).
  - Limits pledged system calls (`stdio`, `rpath`, `wpath`, `cpath`, `exec`).
  - Enforces resource limits (RAM caps, CPU limits) and default offline network isolation.

---

### 2. Package FLAVORS & USE Flags (`AurPackageOptionsEngine`)
- **Parity Inspirations**: FreeBSD Ports `FLAVORS` (e.g. `py311`, `qt6`), Gentoo Portage `USE` flags (e.g. `+ssl`, `+lto`, `-wayland`).
- **Features**:
  - Modular package flavor switching without package duplication.
  - Granular `USE` flag toggle evaluating dynamic `--with-*` configure parameters.

---

### 3. Namcap Quality & Vulnerability Linter (`NamcapSecurityAuditor`)
- **Parity Inspirations**: Arch Linux `namcap` package linter, FreeBSD `portlint`.
- **Lint Rules**:
  - Detects missing mandatory variables (`pkgname`, `pkgver`).
  - Rejects insecure HTTP transfers (`curl` without TLS).
  - Flags dangerous privilege escalation patterns (`sudo`, `doas`) in unprivileged build scripts.

---

### 4. Overlays & Trusted User Pipeline (`AurOverlayManager` & `AurTrustedUserPipeline`)
- **Parity Inspirations**: Gentoo `layman`/overlays multi-repo management, Arch Linux Trusted User (TU) voting & promotion.
- **Workflow**:
  - Supports dynamic registration of user-maintained package overlays.
  - Allows Trusted Users to vote and automatically promote high-popularity AUR packages into official binary repositories (`core`, `extra`, `community`).
