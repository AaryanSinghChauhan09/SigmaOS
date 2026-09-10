# AI Agent Gentoo Portage & Ebuild Management Directives

## Executive Overview
This document specifies maintenance rules for AI agents developing and maintaining Gentoo Portage `ebuild` compilation, `USE` flag optimization, and subslot ABI rebuild solver modules in SigmaOS.

---

## Core Components
1. **Portage USE Flag Governor**: `PortageUseFlagGovernor` in `src/distro/distro_inspiration_engine.rs`.
2. **Subslot & EAPI Solver**: `GentooPortageSubslotAndUseExpandEngine` and `GentooPortageEapiSlotOperatorEngine` in `src/package/bsd_linux_package_innovations.rs`.
3. **Per-Package Env Overrides**: `PortagePackageEnvEngine` in `src/package/bsd_linux_package_innovations.rs`.

---

## AI Agent Verification Protocol
Agents updating Gentoo Portage layers must run:
```bash
rustc --test src/package/bsd_linux_package_innovations.rs --edition=2021 -o build/package_innovations_test && ./build/package_innovations_test
./run_sigma_tests.sh
```
