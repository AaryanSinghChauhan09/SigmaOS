# AI Agent Process Management Specification for SigmaOS

This document provides specifications and guidelines for AI agents developing, supervising, scheduling, and isolating processes across **SigmaOS**.

---

## 1. Process Scheduling Architecture

SigmaOS incorporates hybrid scheduling algorithms inspired by advanced Linux and BSD kernel developments:

1. **Linux 6.12+ `sched_ext` Extensible Scheduler Framework**:
   - Implemented in `src/distro/sovereign_nextgen_distro_leap.rs` (`SovereignSchedExtEngine`).
   - Supports dynamic BPF policy switching across `ScxBpfland`, `ScxLavd`, `ScxCachyBore`, and `ScxCentral` with sub-microsecond preemption and NUMA node balancing.

2. **CachyOS BORE (Burst-Oriented Response Enhancer)**:
   - Implemented in `src/kernel/bore.rs` and `src/distro/linux_bsd_inspirations.rs` (`CachyBoreScheduler`).
   - Dynamically calculates task timeslices based on burstiness and interactivity scores (0-100) to minimize user-facing latency.

3. **Linux EEVDF (Earliest Eligible Virtual Deadline First)**:
   - Implemented in `src/kernel/scheduler.rs`. Calculates Virtual Runtime (vruntime) and virtual deadlines based on task weight.

4. **FreeBSD ULE Interactivity Scoring**:
   - Implemented in `src/kernel/scheduler.rs` and `src/distro/wiki_ideas_implementation.rs` (`interactivity_score`).

5. **Apache NuttX POSIX Real-Time Preemption-Threshold**:
   - Implemented in `src/distro/open_source_distro_innovations.rs` (`NuttxRealtimeTaskGovernor`).

---

## 2. Process Supervision & Service Management

SigmaOS bridges all major service supervisor models via `SovereignUniversalDistroBridge::get_supervisor_type()`:

- **Systemd**: `SystemdEngine` (`src/init/systemd_init.rs`)
- **OpenRC**: `OpenRCService` (`src/distro/linux_bsd_inspirations.rs`)
- **Runit**: `SovereignRunitSupervisor` (`src/distro/linux_bsd_inspirations.rs`) & `VoidRunitSupervisor` (`src/distro/improvements.rs`)
- **Shepherd**: `ShepherdServiceManager` (`src/distro/linux_bsd_inspirations.rs`)
- **Dinit / SysVInit / S6**: `S6ServiceInitSupervisor` & `ChimeraDinitSupervisor` (`src/distro/missing_distro_innovations.rs`)

---

## 3. Process Isolation & Resource Control

1. **Linux Landlock LSM (v1-v5)**:
   - Filesystem path sandboxing (`SovereignLandlockLsm`) and TCP `bind`/`connect` port gating (`LandlockV5NetworkGuard`).

2. **FreeBSD Jails & RACCT/RCTL**:
   - Process tree containment (`FreeBSDJail`) and RSS/PID/CPU resource limits (`FreeBsdRacctVnetGuard`).

3. **OpenBSD Pledge & Unveil**:
   - System call promise restriction (`OpenBSDPledge`) and path unveiling (`OpenBSDUnveil`).

4. **Linux Cgroup v2 Governor**:
   - `LinuxCgroupV2Governor` (`src/compatibility/linux_standards.rs`) for CPU quota and memory slice enforcement.

---

## 4. Testing & Verification Commands

```bash
# Run process scheduler inspection tests
rustc --test src/kernel/scheduler.rs --edition=2021 -o build/test_sched && ./build/test_sched

# Run full test suite
./run_sigma_tests.sh
```
