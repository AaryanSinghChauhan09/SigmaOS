# AI Agent Universal Package Management Guide

## Overview
This wiki guide details Universal Package Management protocols for AI coding agents operating on SigmaOS. It covers foreign package format parsing (.deb/apt, .rpm/dnf, PKGBUILD/pacman, ebuild/portage, apk, xbps, hpkg, snap, flatpak, nix), `SigPkgUniversalBridgeEngine`, `UniversalDependencyMapper` canonicalization, `UniversalScriptletConverter` hook transpilation, `UniversalSandboxCapabilityMatrix` permission mapping, and `UniversalPmCommandDispatcher` CLI command translation.

## Key Packaging Principles
1. **Universal Adapter**: Foreign package manifests (.deb, .rpm, PKGBUILD, ebuild, apk, hpkg) are parsed natively and translated into canonical `Sigma-pkg` models.
2. **Dependency Canonicalization**: Distro-specific dependency names (`openssl-devel`, `libssl-dev`, `security/openssl`) map to canonical `openssl`.
3. **Sandbox Mapping**: Snap plugs and Flatpak finish-args map directly to native SigmaOS Capability permissions.

## Absorption Example (`src/sigpkg/universal_adapter.rs`)
```rust
let mut bridge = SigPkgUniversalBridgeEngine::new();
let pkg = bridge.absorb_and_register("nginx.deb", control_bytes)?;
```

## Related Documents
- `docs/AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENT_DATA_OPERATION_MANAGEMENT.md`
