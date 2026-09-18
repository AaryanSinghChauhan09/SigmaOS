# AI Agent Fedora Parity Management Specification for SigmaOS

This document provides guidelines and architectural specifications for AI agents maintaining and developing Red Hat / Fedora Linux compatibility components within **SigmaOS**.

---

## 1. Overview & Fedora Parity Subsystem

SigmaOS implements a clean-room, zero-external-dependency Red Hat / Fedora ecosystem subsystem across `src/compatibility/fedora.rs`, `src/distro/fedora_parity.rs`, `src/package/fedora_dnf.rs`, and `src/sigpkg/fedora_rpm_engine.rs`.

Key components managed by AI agents:

1. **Fedora Account System (FAS) & OIDC Auth (`FedoraFasAuthEngine`)**:
   - Manages FAS user profiles, FPCA (Fedora Project Contributor Agreement) signing, group memberships (`packager`, `provenpackager`, `qa`, `sysadmin-main`), and OIDC authentication tokens.
2. **Greenwave CI Policy Gating Engine (`FedoraGreenwaveCiEngine`)**:
   - Evaluates automated Greenwave CI gating rules (e.g., `dist.rpmdeplint`, `upgrades.rpmdeplint`, `openQA.boot`), waives test requirements (`submit_waiver`), and gates Bodhi package promotion to stable repositories.
3. **DNF & RPM Dependency Resolver (`DnfPackageResolver`)**:
   - Performs recursive RPM dependency checks, tracks repository metadata, and validates GPG package signatures.
4. **Koji Collaborative Build Server (`KojiBuildServer` & `FedoraKojiTaskRunner`)**:
   - Receives build tasks for target architectures (`x86_64`, `aarch64`, `riscv64`), schedules workers, and handles release tagging.
5. **Bodhi Update Triage System (`BodhiUpdateTriage`)**:
   - Tracks update types (`Bugfix`, `Enhancement`, `Security`, `NewPackage`), karma thresholds, Greenwave CI gates, and generates DNF `updateinfo.xml` repodata.
6. **SELinux Context & Policy Transition Engine (`SeLinuxEngine` & `SeLinuxEnforcer`)**:
   - Enforces SELinux Security Contexts (`user:role:type:sensitivity`), AVC permissions checks, and dynamic domain transition rules.
7. **Anaconda Installer & Kickstart Configurator (`AnacondaInstaller` & `FedoraAnacondaKickstartGenerator`)**:
   - Parses and generates declarative Anaconda Kickstart files (`rootpw`, `lang`, `keyboard`, `part`, `%packages`).
8. **Toolbx OCI Development Container Engine (`FedoraToolbxContainerEngine`)**:
   - Manages interactive OCI development containers with automatic host bind-mounts (`/home`, `/dev`, `/run/host`).

---

## 2. Rules for AI Agents Developing Fedora Parity Modules

1. **Zero External Dependencies**:
   - Implement all algorithms natively using Rust or `klib` primitives without external crates.
2. **Greenwave CI & Karma Safety**:
   - Critical-path updates require a minimum of `3` karma and `7` days in testing before promotion unless overridden by an explicit Greenwave waiver or security fast-track.
3. **SELinux Enforcing Mode**:
   - Access checks in `Enforcing` mode must return an AVC denial error when no explicit rule permits the transition.
4. **Testing Matrix**:
   - Include unit tests in `src/compatibility/fedora.rs` for any new Fedora parity structures or methods.

---

## 3. Verification Commands

AI agents must verify Fedora compatibility changes using:

```bash
# Run standalone Fedora unit test suite
rustc --test src/compatibility/fedora.rs --edition=2021 -o build/test_fedora && ./build/test_fedora
```
