# Universal Linux & BSD Distribution Parity Roadmap — AI Agent Directives

## Overview
This document specifies the future development roadmap and autonomous agent execution directives for absorbing and unifying capabilities from top Linux and BSD distributions into SigmaOS.

## Distribution Parity Matrix

### Linux Distribution Parity
- **Arch Linux:** Pacman package database synchronization, `PKGBUILD` stanza parsing (`PkgbuildMetadataParser`), and ALPM transaction hooks (`PacmanHooksEngine`).
- **Debian Linux:** `dpkg-divert` file redirection (`DpkgDivertEngine`), `dpkg-trigger` deferred processing (`DpkgTriggersEngine`), `debconf` preseed answer engine, and `deb-control` stanza parsing (`DebControlParser`).
- **Fedora / RHEL:** Active Directory / FreeIPA realm joining (`FedoraSssdFreeIpaEngine`), SSSD authentication, RPM `.spec` parsing (`FedoraRpmSpecParser`), and RPM-OSTree atomic rollback (`FedoraRpmostreeAtomicEngine`).
- **Alpine Linux:** `apk` package solver, `musl` libc compatibility bridge, and OpenRC runlevel init scripts.
- **Gentoo Linux:** Portage USE flags engine, `ebuild` emerge dependency solver, and toolchain optimization flags.
- **Void Linux:** `xbps` package transaction solver and `runit` service supervision daemon.
- **NixOS:** Declarative Nix flake state management and atomic system generation rollback.

### BSD Distribution Parity
- **FreeBSD:** VNET network virtualization, Capsicum capability sandbox (`cap_rights_limit`), and `bsd-pkg` package translation.
- **OpenBSD:** Pledge syscall sandboxing (`pledge`), unveil filesystem visibility (`unveil`), and KARL kernel address space layout randomization.
- **NetBSD:** Rump kernel component virtualization and `pkgsrc` package manager compatibility.

## Agent Quality Directives
- **Zero-Allocation Hot Paths:** Use bounds-checked byte slice window scans (`windows(N)`) for path and input validation without temporary heap `String` allocations.
- **Verification:** Every distribution engine must include standalone `#[test]` unit test functions executable via `rustc --test`.
- **PR Format Submissions:** Every distro compatibility module must expose a `*PrProposal` struct formatted for GitHub PR submission.
