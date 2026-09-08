# AI Agent Fedora Linux Subsystem Parity & Components Directives (`docs/AGENTS_FEDORA_COMPONENTS_MANAGEMENT.md`)

This document defines AI agent and contributor guidelines for implementing, maintaining, and verifying Fedora Linux parity components in `src/compatibility/fedora.rs` and across SigmaOS.

---

## 1. Overview of Fedora Parity Subsystems

SigmaOS implements complete native compatibility for key Fedora Linux infrastructure components without relying on external pre-compiled Python or C libraries:

1. **Fedora CoreOS Ignition Declarative Provisioning Engine (`FedoraIgnitionEngine`)**
   - Parses early boot declarative specifications for files (`IgnitionFile`), users (`IgnitionUser`), and systemd units (`IgnitionSystemdUnit`).
   - Runs once on first boot before handing off control to userspace init.

2. **Fedora Offline Update Engine (`FedoraOfflineUpdateEngine`)**
   - Implements `systemd-offline-update` semantics by staging updates in `/var/lib/systemd/updates`.
   - Triggers clean reboot updates and verifies pending package state transitions.

3. **Bodhi Update Triage & Feedback Engine (`BodhiUpdateTriage`)**
   - Tracks package update karma, CI test results (`BodhiTestResult`), and release state transitions (`Pending`, `Testing`, `Stable`).

4. **Koji Build System Client (`KojiBuildSystemClientEngine`)**
   - Manages Koji build tasks, SCM source URLs, build targets, and RPM build artifact verification.

5. **PipeWire WirePlumber Policy Governor (`FedoraPipewireWireplumberPolicyGovernor`)**
   - Manages audio/video node routing, default sink/source node assignments, and session policy enforcement.

6. **RPM Post-Install Seccomp BPF Syscall Filter (`FedoraRPMSeccompFilterEngine`)**
   - Enforces syscall sandboxing for RPM scriptlets and post-install hooks.

7. **"The New Hotness" Anitya Upstream Release Monitoring (`FedoraTheNewHotnessEngine`)**
   - Maps Anitya upstream project IDs to Fedora RPM package names and dispatches fedmsg events upon version increments.

8. **Fedora Messaging & fedmsg Bus (`FedoraMessagingEngine`)**
   - Provides AMQP/ZeroMQ topic-based message publication, subscription routing, and SHA256 cryptographic signature verification.

9. **Fedora MirrorManager2 Dynamic Routing (`FedoraMirrorManager2Engine`)**
   - Evaluates client ASN, IP geolocation, bandwidth, and sync lag to select optimal package mirror hosts (`FedoraMirrorHost`).

---

## 2. Invariants & Implementation Standards

- **Zero-Dependency `#![no_std]`:** All engines must remain zero-dependency `#![no_std]` compatible, utilizing `alloc::vec::Vec`, `alloc::string::String`, and `crate::klib::BTreeMap`.
- **Thread Safety:** Synchronization structures must use atomic counters or lock-free primitives where appropriate.
- **Error Handling:** Avoid panics in library code; return explicit `Result<T, &'static str>` or `Option<T>` types.

---

## 3. Verification Protocol

AI agents and contributors must verify Fedora components by running:
```bash
rustc --test --edition 2021 src/compatibility/fedora.rs -o build/test_fedora && ./build/test_fedora
```
or executing the unified native test runner:
```bash
./run_sigma_tests.sh
```
