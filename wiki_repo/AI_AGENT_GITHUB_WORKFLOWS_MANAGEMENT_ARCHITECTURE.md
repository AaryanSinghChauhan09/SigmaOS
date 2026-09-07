# AI Agent GitHub Actions Workflows Management Architecture

## Executive Overview

SigmaOS maintains an extensive suite of automated GitHub Actions workflows located in `.github/workflows/`. Inspired by continuous integration, reproducible builds, security auditing, and deployment pipelines across major Linux and BSD distributions (Arch Linux, Debian, Fedora, Gentoo, Alpine, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, NixOS, Void, Qubes, Tails, Clear Linux, Solus, Slackware, Illumos, and Haiku), these workflows automate code testing, security auditing, Pages publishing, PR governance, and release artifact signing.

This document serves as the architectural reference for AI coding agents creating, inspecting, or maintaining GitHub Actions workflows in SigmaOS.

---

## Workflow Suite Architecture & Functional Categories

```
                                +-----------------------------------+
                                |    Git Commit / Pull Request      |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |     .github/workflows/ Suite      |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | Distro Matrix CI      |   | Security & Audit  |   | Pages & Automation    |
            | arch-, debian-, fedora|   | cargo-audit, SAST |   | pages auto deploy     |
            | freebsd-, openbsd-ci  |   | cosign, sbom scan |   | wiki-sync, pr-checks  |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |    Automated Verification Pass    |
                                +-----------------------------------+
```

### Functional Categories

1. **Distro Parity Matrix Workflows**:
   - **Linux Distros**: Arch Linux PKGBUILD/AUR (`arch-aur-pkgbuild-ci.yml`), Fedora RPM/ostree (`fedora-crypto-policies-rpm-ostree-ci.yml`), Debian sbuild (`debian-sbuild-reproducible-ci.yml`), Alpine musl (`alpine-musl-apk-security-ci.yml`), Gentoo ebuild (`gentoo-portage-ebuild-ci.yml`), Void xbps/runit (`void-runit-supervision-ci.yml`), NixOS flake store (`nixos-flake-store-gc-ci.yml`), Clear Linux stateless (`clear-linux-swupd-stateless-ci.yml`), Slackware slackpkg (`slackware-slackpkg-pkgtool-ci.yml`), Solus eopkg, Qubes Xen isolation (`qubes-xen-isolation-security-ci.yml`), Tails amnesic privacy (`tails-amnesic-privacy-security-ci.yml`).
   - **BSD & Alternative Distros**: FreeBSD Jail/ZFS (`freebsd-jail-zfs-bootenv-ci.yml`), OpenBSD pledge/unveil (`openbsd-pf-pledge-security-ci.yml`), NetBSD rump kernel (`netbsd-rump-kernel-ci.yml`), DragonFly HAMMER2 (`dragonfly-hammer2-pfs-ci.yml`), HardenedBSD PaX (`hardenedbsd-secadm-pax-ci.yml`), Illumos DTrace/Zones (`illumos-dtrace-zfs-zones-ci.yml`), Haiku packagefs (`haiku-packagefs-hpkg-ci.yml`).

2. **Security & Reproducible Build Pipelines**:
   - `security-audit.yml`: Cargo security advisory checking (`cargo audit`).
   - `reproducible-sbom-cosign.yml`: SPDX/CycloneDX SBOM generation and Cosign artifact signing.
   - `sast-fuzzing-semgrep.yml`: Static analysis and fuzz testing.

3. **Pages & Governance Workflows**:
   - `sovereign-distro-pages-auto-deploy.yml`: GitHub Pages documentation publishing.
   - `wiki-sync.yml`: Dual-repository wiki synchronization between `wiki/` and `wiki_repo/`.
   - `pr_fast_checks.yml` / `05_Automation_PR_Governance.yml`: Automated PR labeling and branch name verification.

---

## Zero-Drift Guardrails

AI agents modifying or adding workflows must follow these YAML rules:
- Toolchain setup MUST use `dtolnay/rust-toolchain@v1` with `with: toolchain: stable`.
- Run commands MUST execute native test scripts (`./run_sigma_tests.sh`) to guarantee sandbox parity.
- Indentation MUST strictly adhere to 2-space YAML formatting.

---

## Related Architectural References
- `.github/workflows/` - Master workflows directory.
- `docs/AI_AGENT_GITHUB_WIKI_MANAGEMENT_ARCHITECTURE.md` - Wiki deployment architecture.
