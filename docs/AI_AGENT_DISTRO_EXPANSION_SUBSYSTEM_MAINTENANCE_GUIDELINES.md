# AI Agent Maintenance Guidelines: Distro Expansion Subsystems

This document establishes operational procedures, maintenance guidelines, and cross-subsystem dispatch standards for AI agents managing distribution expansion modules in `src/distro/`.

---

## 1. Distribution Expansion Architecture

SigmaOS provides native support for **25 Linux and BSD distribution modes** through the `SovereignUniversalDistroBridge` in `src/distro/linux_bsd_inspirations.rs`.

### Core Subsystem Dispatch Categories (32 Subsystems)
All distribution modes dispatch across **32 core subsystem categories**:
`init`, `package`, `vfs`, `security`, `storage`, `kernel`, `network`, `graphics`, `power`, `ipc`, `auth`, `audit`, `boot`, `container`, `virtualization`, `audio`, `input`, `thermal`, `memory`, `syscall`, `device`, `crypto`, `ai`, `monitoring`, `desktop`, `compiler`, `i18n`, `bluetooth`, `firewall`, `diagnostics`, `recovery`, `time`.

---

## 2. Guidelines for Adding & Maintaining Distro Modes

### 2.1 Adding a New Distro Subsystem
1. **Define Distro Mode Enum Variant:** Add the new distribution to `LinuxBsdDistroMode` in `src/distro/linux_bsd_inspirations.rs`.
2. **Implement Distro Subsystem Module:** Create a dedicated file in `src/distro/<distro_name>.rs` and declare `pub mod <distro_name>;` in `src/distro/mod.rs`.
3. **Register Match Arms in Distro Bridge:** Add explicit match arms in `SovereignUniversalDistroBridge::dispatch_subsystem_operation` for all 32 core subsystem categories.
4. **Add Unit Tests:** Implement comprehensive unit tests verifying subsystem dispatching and feature functionality.

### 2.2 Maintenance Checklist
* [ ] Verify that `SovereignUniversalDistroBridge::dispatch_subsystem_operation` contains NO wildcard (`_ =>`) fallthroughs for known distro modes.
* [ ] Execute `./run_sigma_tests.sh` to confirm workspace test integrity.
* [ ] Update `docs/LINUX_BSD_DISTRO_COMPONENTS_AND_GUIDELINES.md` and `WHAT_IS_WORKING_AND_NOT_WORKING.md` to reflect new capabilities.

---

*Document Version:* 1.0.0
*Maintained By:* SigmaOS AI Agent Distribution Engineering Council
