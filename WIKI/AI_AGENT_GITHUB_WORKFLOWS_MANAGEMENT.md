# AI Agent GitHub Actions Workflows Management Guide

## Overview
This wiki guide details GitHub Actions Workflows Management protocols for AI coding agents operating on SigmaOS. It covers over 60+ automated workflows in `.github/workflows/` inspired by Arch Linux, Debian, Fedora, Gentoo, Alpine, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, NixOS, Void, Qubes, Tails, Clear Linux, Solus, Slackware, Illumos, and Haiku.

## Key Principles
1. **Multi-Distro Matrix**: CI matrix workflows verify package format bridges (.deb, .rpm, PKGBUILD, ebuild, apk, hpkg, nix).
2. **Reproducible Build Signing**: Workflows generate SPDX/CycloneDX SBOMs and sign release artifacts via Cosign.
3. **Rust Toolchain Standard**: All workflows configure Rust via `dtolnay/rust-toolchain@v1` with `toolchain: stable`.

## Sample Workflow (`.github/workflows/sample.yml`)
```yaml
name: Sample CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@v1
        with:
          toolchain: stable
      - run: ./run_sigma_tests.sh
```

## Related Documents
- `docs/AI_AGENT_GITHUB_WORKFLOWS_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_GITHUB_WORKFLOWS_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENT_GITHUB_WIKI_MANAGEMENT.md`
