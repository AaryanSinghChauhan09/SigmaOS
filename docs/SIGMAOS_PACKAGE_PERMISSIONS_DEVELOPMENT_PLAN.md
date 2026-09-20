# Strategic Development Plan for SigmaOS Package Format Permissions

## Executive Summary
This document establishes the strategic 5-phase development roadmap for SigmaOS package format permissions (`src/package/universal.rs`, `src/package/hardening.rs`, `src/package/sandbox.rs`, `src/sigpkg/`). Drawing inspiration from Linux package permission models (Debian `dpkg-statoverride`, Fedora RPM `%caps`, Flatpak `finish-args`, Ubuntu Snap plugs/slots) and BSD package directives (FreeBSD `pkg` plist `@mode`/`@owner`/`@group`, OpenBSD `pkg_add` `@mode`/`@owner`/`@exec` and `pledge`/`unveil` promises), SigmaOS combines traditional POSIX permission bits with zero-trust capability tokens and post-quantum cryptographic signatures.

---

## 1. Package Permission Model Benchmark & Inspiration Matrix

| Package System / Distro | Permission Model / Mechanism | Key Features Absorbed by SigmaOS | SigmaOS Integration Layer |
| :--- | :--- | :--- | :--- |
| **FreeBSD `pkg` / OpenBSD `pkg_add`** | Plist `@mode`, `@owner`, `@group`, `@exec` directives | Explicit, deterministic file mode masks and ownership declarations | `src/package/universal.rs` (`PackageMetadataAdapter`) |
| **Debian `dpkg` / `apt`** | `dpkg-statoverride` database overrides | Administrative system-wide permission overrides for installed files | `src/package/bsd_linux_package_innovations.rs` (`DpkgStatoverrideRule`) |
| **Fedora / RHEL RPM** | `%caps(cap_net_bind_service=ep)` RPM capabilities | Executable Linux capabilities assigned without setuid root bits | `src/package/hardening.rs` (`PackageSecurityMetadata`) |
| **Flatpak / Ubuntu Snap** | `finish-args` sandboxing manifest (`--share=network`, `--filesystem=host`) | Declarative sandboxing claims translated into process pledges | `src/package/universal.rs` (`PackageAdapter::translate_flatpak_sandbox_policy`) |
| **OpenBSD Base System** | `pledge` promises & `unveil` filesystem path rules | Process syscall restriction and restricted filesystem visibility | `src/package/sandbox.rs` (`PackageSandboxPolicy`) |

---

## 2. Strategic 5-Phase Package Permissions Roadmap

```
┌───────────────────────────────────────────────────────────────────────────┐
│               SIGMAOS PACKAGE PERMISSIONS ROADMAP                         │
└───────────────────────────────────────────────────────────────────────────┘
   Phase 1: POSIX Mode & Ownership Manifest Declarations
   ├── Explicit file permissions (`0o755`, `0o644`, `0o4755` SUID) in manifests
   ├── User (`owner_uid`) and group (`group_gid`) ownership assignments
   └── FreeBSD/OpenBSD plist `@mode` and `@owner` directive parity

   Phase 2: Granular Capability Token Binding
   ├── Capability claims (`Permission::FileRead`, `Permission::Network`, `Permission::DeviceIo`)
   ├── Unforgeable capability derivation tree binding
   └── Fine-grained system call privilege escalation prevention

   Phase 3: Executable Capabilities & Statoverride Override DB
   ├── Executable capability masks (`cap_net_bind_service`, `cap_sys_admin`)
   ├── Administrative statoverride database (`/etc/sigmaos/statoverride`)
   └── Suppression of unnecessary setuid-root binary flags

   Phase 4: Sandboxing Pledges & Landlock LSM Path Rules
   ├── OpenBSD `pledge` promise assertions (`stdio rpath wpath inet`)
   ├── OpenBSD `unveil` and Linux Landlock LSM path access rules
   └── Flatpak `finish-args` and Snapcraft confinement translation

   Phase 5: Post-Quantum Signature Verification & MAC Policies
   ├── PQC Dilithium-5 digital signature verification for binary packages
   ├── Mandatory Access Control (MAC) label assignment (SELinux / AppArmor parity)
   └── Immutable store checksum auditing (`ContentAddressedStore`)
```

---

## 3. Detailed Phase Architecture

### Phase 1: POSIX Mode & Ownership Manifest Declarations
- **Manifest File Modes**: Each file entry in a `.sigpkg` or translated foreign package manifest (`.deb`, `.rpm`, `.apk`) contains explicit octal file permissions (`0o755` for executables, `0o644` for data files, `0o4755` for SUID binaries).
- **Ownership Attributes**: Specifies target numeric UID (`owner_uid`) and GID (`group_gid`) alongside user and group string names (`owner_user`, `owner_group`).
- **Plist Directive Parity**: Supports FreeBSD/OpenBSD style plist directives (`@mode 0755`, `@owner root`, `@group wheel`) during package extraction and installation.

### Phase 2: Granular Capability Token Binding
- **Capability Tokens**: Rather than granting full root privileges, binaries declare required capability tokens in their package manifest:
  ```toml
  # Sample SigPkg permission manifest block
  [permissions]
  capabilities = ["Permission::FileRead", "Permission::Network", "Permission::DeviceIo"]
  max_memory_mb = 512
  allow_network_hosts = ["api.sigmaos.org", "cdn.sigmaos.org"]
  ```
- **Derivation Tree**: The package manager issues restricted capability tokens derived from system master capabilities, ensuring unhandled capabilities cannot be forged.

### Phase 3: Executable Capabilities & Statoverride Override DB
- **Executable Capabilities**: Binds Linux file capabilities directly to installed ELF binary extended attributes (e.g., `setcap cap_net_bind_service=+ep /usr/bin/webserver`), avoiding setuid root vulnerabilities.
- **Statoverride DB**: Implements a Debian `dpkg-statoverride` equivalent database (`/etc/sigmaos/statoverride`), allowing system administrators to override default package permissions persistently across package upgrades.

### Phase 4: Sandboxing Pledges & Landlock LSM Path Rules
- **Syscall Promises**: Packages declare OpenBSD-style `pledge` promises restricting available system calls during runtime execution (e.g., `pledge = "stdio rpath wpath inet"`).
- **Filesystem Unveil**: Binds Landlock LSM and OpenBSD `unveil` path rules, restricting application file visibility exclusively to declared paths (e.g., `unveil = [["/etc/app.conf", "r"], ["/var/log/app", "rw"]]`).
- **Flatpak/Snap Translation**: The universal package adapter translates Flatpak `finish-args` (`--share=network`, `--filesystem=host`) into equivalent pledge and unveil rules.

### Phase 5: Post-Quantum Signature Verification & MAC Policies
- **PQC Signatures**: Every `.sigpkg` package header carries a Post-Quantum Cryptographic signature (Dilithium-5 or Falcon-1024), verified against system trust anchors before permission assignment.
- **Mandatory Access Control**: Assigns SELinux/AppArmor compatible security labels (`system_u:object_r:httpd_exec_t:s0`) to installed files, enforcing kernel-level MAC policies.

---

## 4. Verification & Testing Strategy
Package permissions and security metadata are verified using native test suites:
```bash
# Run package manager unit tests
cargo test --package sigmaos --lib package

# Run security pledge and unveil test suite
cargo test --package sigmaos --lib security

# Run full native test runner
bash run_sigma_tests.sh
```
