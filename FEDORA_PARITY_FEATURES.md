# Fedora Linux Parity Features in SigmaOS

SigmaOS incorporates native, zero-dependency Rust implementations inspired by Red Hat and Fedora Linux core innovations. This document details the architectural components and sub-systems implemented in `src/compatibility/fedora.rs`, `src/sigpkg/fedora_rpm_engine.rs`, `src/distro/fedora_parity.rs`, and related modules.

***

## Architecture Overview

SigmaOS implements safe Rust native equivalents for Fedora's foundational tooling across package management, security policies, image deployment, build systems, and community infrastructure.

    +-----------------------------------------------------------------------+
    |                      Fedora Parity Layer in SigmaOS                    |
    +------------------------------------+----------------------------------+
    | Component                          | Implementation File              |
    +------------------------------------+----------------------------------+
    | DnfPackageResolver & Dnf5Engine    | `src/compatibility/fedora.rs`    |
    | FedoraSilverblueRpmOstreeEngine    | `src/compatibility/fedora.rs`    |
    | SovereignSeLinuxEngine             | `src/compatibility/fedora.rs`    |
    | SovereignFirewalldManager          | `src/compatibility/fedora.rs`    |
    | FedoraCryptoPoliciesEngine         | `src/compatibility/fedora.rs`    |
    | FedoraToolbxContainerEngine        | `src/compatibility/fedora.rs`    |
    | FedoraCockpitWebConsoleEngine      | `src/compatibility/fedora.rs`    |
    | AnacondaInstaller & KickstartGen   | `src/compatibility/fedora.rs`    |
    | BodhiUpdateTriage & KojiTaskRunner | `src/compatibility/fedora.rs`    |
    | FedoraTheNewHotnessEngine          | `src/distro/fedora_new_hotness.rs` |
    | FedoraMirrorManager2Engine         | `src/sigpkg/fedora_rpm_engine.rs` |
    +------------------------------------+----------------------------------+

***

## Implemented Subsystems & Features

### 1. Package Management (`DNF` & `DNF5` Parity)

*   **`DnfPackageResolver`**: Recursive dependency resolution with circular dependency detection, GPG signature verification, and repository metadata syncing.
*   **`FedoraDnf5PackageEngine`**: Microdnf & Libdnf5 plugin architecture and transaction handling.
*   **`FedoraDnfHistoryRollbackEngine`**: Transaction history tracking with point-in-time $O(1)$ undo and rollback delta calculation.
*   **`FedoraOfflineUpdateEngine`**: `systemd-offline-update` parity for staging updates and triggering execution upon reboot.

### 2. Atomic OS & Image Delivery (`rpm-ostree` & `Silverblue` Parity)

*   **`FedoraSilverblueRpmOstreeEngine`**:
    *   Atomic commit staging and deployment.
    *   RPM package layering (`overlay_layer_package`).
    *   Deployment pinning (`pin_deployment`).
    *   Stream rebasing (`rebase_stream`).
    *   Instant rollback capability.

### 3. Security & Governance (`SELinux` & `crypto-policies`)

*   **`SovereignSeLinuxEngine` / `SeLinuxEngine`**:
    *   Mandatory Access Control (MAC) enforcement and permissive modes.
    *   Security context parsing (`user:role:type:sensitivity`).
    *   Domain transition rules and policy authorization.
*   **`FedoraCryptoPoliciesEngine`**:
    *   System-wide cryptographic profiles (`Default`, `Legacy`, `Future`, `Fips`, `Custom`).
    *   Custom sub-profile enablement (`SHA1`, `PQC` quantum-resistance).
    *   Cipher suite and RSA key length validation.

### 4. Containerization & Developer Environments (`Toolbx` & `Flatpak`)

*   **`FedoraToolbxContainerEngine`**: Interactive OCI container management with automatic host bind-mounts (`/home`, `/dev`, `/run/host`), environment injection, and command execution (`run_command`).
*   **`FedoraFlatpakSandboxManager`**: `bwrap` namespace sandboxing with fine-grained XDG Desktop Portal access control.

### 5. Administration & Network Control (`Cockpit` & `firewalld`)

*   **`SovereignCockpitConsole` / `FedoraCockpitWebConsoleEngine`**: Real-time web administration bridge with JSON telemetry streaming and service state control.
*   **`SovereignFirewalldManager`**: Dynamic network security zones (`public`, `trusted`, `work`) with interface mapping and port filtering.

### 6. Build Systems & Release Infrastructure (`Koji`, `Bodhi`, `Anitya`, `MirrorManager2`)

*   **`KojiBuildServer` & `FedoraKojiTaskRunner`**: Multi-arch build task queueing (`x86_64`, `aarch64`, `riscv64`) and release tagging.
*   **`BodhiUpdateTriage`**: Karma accumulation, Greenwave CI testing gates, critical-path testing durations, and `updateinfo.xml` generation.
*   **`FedoraTheNewHotnessEngine`**: Upstream release monitoring (Anitya mapping) and automated fedmsg event broadcasting.
*   **`FedoraMirrorManager2Engine`**: GeoIP, BGP ASN, and bandwidth-weighted mirror selection with Metalink XML generation.

***

## Verification & Testing

All Fedora parity components are tested via the automated suite in `src/compatibility/fedora.rs`, `src/sigpkg/fedora_rpm_engine.rs`, and standalone runners in `./run_sigma_tests.sh`.

To execute the full test suite:

```bash
./run_sigma_tests.sh
```
