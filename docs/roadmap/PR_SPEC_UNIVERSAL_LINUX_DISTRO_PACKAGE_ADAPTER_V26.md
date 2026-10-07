# Pull Request Specification: Universal Linux Distro Package Adapter Suite V26

## Overview
This Pull Request Specification defines the **Universal Linux Distro Package Adapter Suite V26** (`src/package/sovereign_distro_package_advancements_v26.rs`), extending SigmaOS's universal package system to seamlessly adapt to, inspect, classify, sandbox, and transpilation-install packages from all major Linux and BSD distributions into native `SigmaPkg` format.

## Architecture & Subsystems V26

### 1. Universal Format Inspector and Classifier V26 (`UniversalFormatInspectorAndClassifierV26`)
- Performs zero-copy inspection of foreign package files across 36+ package formats (`.deb`, `.rpm`, `.apk`, `.pkg.tar.xz`, `.pkg.tar.zst`, `.xbps`, `.eopkg`, `.nixpkg`, `.portage`, `.flatpak`, `.snap`, `.AppImage`, `.air`, `.bottle`, `.ipa`, `.ports`, `.pkg`, `.aab`, `.app`, `.hap`, `.PiSi`, `.lzm`, `.pup`, `.pet`, `.zypper`, `.guix`, `.moss`, `.hpkg`, etc.).
- Automatically classifies signature attestation kinds (`OpenBsdSignify`, `PqcKyberDilithium`, `GpgOpenPgp`, `X509Certificate`, `ApkV2V3Signature`, `Unsigned`).
- Extracts build flags (`-O3 -march=x86-64-v3 -fstack-protector-strong`) and hashes raw payload into SHA-256 integrity digests.

### 2. Universal Cross-Distro Capability Governor V26 (`UniversalCrossDistroCapabilityGovernorV26`)
- Maps distro-specific dependency names (`libssl-dev`, `openssl-devel`, `security/openssl`, `libc6`, `glibc`, `musl`, `zlib1g-dev`, `python3-dev`) to canonical sovereign dependency names (`sovereign-openssl`, `sovereign-libc`, `sovereign-zlib`, `sovereign-python`).
- Generates granular sandboxing policies per format including OpenBSD `pledge`/`unveil`, Linux `landlock`, FreeBSD `capsicum`, and macOS `app-sandbox` entitlements.

### 3. UDF Scriptlet Sandbox Engine V26 (`UdfScriptletSandboxEngineV26`)
- User-Defined Function (UDF) hook engine executing lifecycle scriptlets (`PreInstall`, `PostInstall`, `PreRemove`, `PostRemove`, `PreTranspile`).
- Maintains an audit execution log for pre/post build scriptlet sandboxing.

### 4. Universal Multi-Format Transpiler and Execution Engine V26 (`UniversalMultiFormatTranspilerAndExecutionEngineV26`)
- Transpiles foreign package manifests into native `UnifiedPackage` in `SigmaPkg` format.
- Transactional checkpoint creation and state rollback mechanism.

### 5. Universal Foreign PM CLI Router V26 (`UniversalPmCliRouterV26`)
- Parses foreign PM CLI invocations (`apt`, `pacman`, `dnf`, `apk`, `pkg`, `xbps`, `emerge`, `nix`, etc.) and routes them to native SigmaPkg execution structures.

## Verification & Testing
- Integrated unit tests in `src/package/sovereign_distro_package_advancements_v26.rs` verified via `cargo check --lib` and `./run_sigma_tests.sh`.
