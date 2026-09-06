# AI Agent Directives & Coding Guidelines for SigmaOS

This file provides instructions and guidelines for AI coding agents (Sentinel, Palette, Bolt, Jules) working on the SigmaOS codebase.

## 1. Zero-Dependency & `#![no_std]` Architecture Rules
* **Core Primitives:** `src/klib/` provides `#![no_std]` native Rust replacements for standard data structures (`BTreeMap`, `Vec`, `HashMap`, `JsonParser`, `String`).
* **Zero Allocation Primitives:** Prefer stack-based, zero-allocation primitives in cold paths (e.g. `u64_to_hex_str_stack`, `parse_u64_str` in `src/klib/conversion.rs`).
* **C++ Dependency Reduction:** Write new host/kernel code in safe, native Rust or standard C11. Avoid introducing new C++ dependencies.

## 2. Testing & Verification Requirements
* **Primary Test Runner:** Always execute `./run_sigma_tests.sh` to run all 326 atomic tests, 57 subsystem inspection tests, and 11 security validation tests.
* **Comprehensive Testing Guide:** Refer to `docs/AGENTS_TESTING_GUIDE.md` for full test suite management procedures.
* **Standalone Subsystem Tests:** Subsystem unit tests can also be executed directly using `rustc --test`:
  * Security Input Validation: `rustc --test src/security/input_validation.rs --edition=2021 -o build/input_val_test && ./build/input_val_test`
  * Distro Innovations: `rustc --test src/distro/missing_distro_innovations.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/missing_distros_test && ./build/missing_distros_test`
  * Distro Gaps: `rustc --test src/distro/linux_bsd_distro_gaps.rs --edition=2021 -o build/distro_gaps_test && ./build/distro_gaps_test`
  * Ecosystem Bridge: `rustc --test src/compatibility/linux_bsd_ecosystem_bridge.rs --edition=2021 -o build/ecosystem_bridge_test && ./build/ecosystem_bridge_test`
  * Media Engine: `rustc --test src/media/distro_media_engine.rs --edition=2021 -o build/distro_media_test && ./build/distro_media_test`
  * Sovereign Commands: `rustc --test src/tools/sovereign_commands.rs --edition=2021 -o build/tools_test && ./build/tools_test`
* **Python Integration & Stress Tests:** Run `pytest tests/test_unit_core.py tests/test_integration_system.py tests/test_stress_fuzz_bench.py` or `python3 -m unittest discover -s tests -p "test_*.py"`.
* **C11 Host Tests:** Compile C11 verification tests using `mkdir -p build/cpp_host_build && cd build/cpp_host_build && cmake ../../tests/cpp_host && make && ./host_tests`.

## 3. Security & Vulnerability Guidelines
* **Input Validation:** Ensure strict validation on IP addresses, port numbers, usernames, and path traversal strings in `src/security/input_validation.rs`.
* **IPv4 Validation:** Never allow leading zeros in multi-digit IPv4 octets (e.g. reject `010.0.0.1`) to prevent octal parser differential attacks and SSRF bypasses.
* **CI/CD Hardening:** All GitHub Actions workflows in `.github/workflows/` must pin third-party actions to immutable 40-character commit SHAs and specify explicit least-privilege `permissions: contents: read`.

## 4. Context & Persona Switching Directives
* **Persona Roles:** Follow `docs/AGENTS_SWITCHING_GUIDE.md` when transitioning operational focus between **Sentinel** (Security), **Palette** (UX/a11y), **Bolt** (Performance), and **Jules** (Engineering).
* **State Handoff Verification:** Always run `./run_sigma_tests.sh` and call `initiate_memory_recording` before switching persona operational contexts or submitting pull requests.

## 5. Spinlock & Synchronization Directives
* **Spinlock Guide:** Adhere to `docs/AGENTS_SPINLOCK_MANAGEMENT_GUIDE.md` for `#![no_std]` spinlock primitives, atomic memory ordering (`Acquire`/`Release`/`SeqCst`), interrupt-safe `lock_irqsave` wrappers, and deadlock prevention rules.
* **No Std Mutexes:** Never import `std::sync::Mutex` or `std::sync::RwLock` in core kernel and `klib` modules.

## 6. Protection Access Rights & Memory Security Directives
* **Protection Rights Guide:** Adhere to `docs/AGENTS_PROTECTION_RIGHTS_GUIDE.md` for `mprotect` W^X page protection rules, TLB `invlpg` invalidation, OpenBSD `pledge`/`unveil` monotonic privilege reduction, FreeBSD Capsicum descriptor rights limits, and AppArmor MAC profile enforcement.
* **W^X Rule:** Never assign `PROT_WRITE` and `PROT_EXEC` to the same virtual memory page frame simultaneously.

## 7. Block Device & Storage Directives
* **Block Storage Guide:** Adhere to `docs/AGENTS_BLOCKS_MANAGEMENT_GUIDE.md` for NVMe/VirtIO block drivers, Kyber/BFQ I/O schedulers, JBD2 Merkle transactional logging, and HAMMER2 block deduplication.
* **CoW Safety:** Never overwrite active CoW snapshot blocks directly; use Copy-on-Write allocation guards.

## 8. Class Operation & Subsystem Vtable Directives
* **Class Operation Guide:** Adhere to `docs/AGENTS_CLASS_OPERATION_MANAGEMENT_GUIDE.md` for zero-allocation kernel vtable structures (`FileOperations`, `VnodeOps`, `SchedClass`, `NetDeviceOps`, `BlockDeviceOps`), atomic class registration, and C11 FFI interoperability.
* **Zero Heap Allocation:** Never allocate heap objects inside core vtable method dispatch paths.

## 9. Readers/Writers Synchronization Directives
* **Readers/Writers Guide:** Adhere to `docs/AGENTS_READERS_WRITERS_MANAGEMENT_GUIDE.md` for Readers/Writers synchronization rules (`AtomicRwLock`, RCU lock-free pathways, writer starvation prevention, and interrupt-safe `read_irqsave` / `write_irqsave` locks).
* **No Std RwLock:** Never import `std::sync::RwLock` in kernel space.

## 10. Data & Memory Confidentiality Directives
* **Confidentiality Guide:** Adhere to `docs/AGENTS_CONFIDENTIALITY_MANAGEMENT_GUIDE.md` for data confidentiality, volatile memory zeroization upon drop, constant-time comparison algorithms, and confidential computing (AMD SEV-SNP / Intel TDX) guest state isolation.
* **Volatile Scrubbing:** Always use `write_volatile` to zero secret buffers upon deallocation.

## 11. Task Assignment & Governance Directives
* **Assignment Guide:** Follow `docs/AGENTS_ASSIGNMENT_MANAGEMENT_GUIDE.md` for task routing, triage protocols, subagent delegation rules, and submission criteria.
* **Persona Routing:** Route security tasks to **Sentinel**, UI/a11y to **Palette**, performance to **Bolt**, and distro infrastructure to **Jules**.

## 12. Information Management & Knowledge Base Directives
* **Information Guide:** Adhere to `docs/AGENTS_INFORMATION_MANAGEMENT_GUIDE.md` for knowledgebase lookup protocols, memory recording requirements, and context prioritization (User Directives > Source Code State > Memory Context).
* **Memory Recording:** Always call `initiate_memory_recording` before completing a task or submitting code.

All changes must be validated against the native SigmaOS test suite:

```bash
# Run full test suite (atomic unit tests, subsystem inspection, Python pytest suite)
./run_sigma_tests.sh

# Run standalone module tests
rustc --edition=2021 --test --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.3.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, Loader, & Desktop Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native architecture where autonomous agent processes govern kernel scheduling, memory pools, dynamic module loading, and desktop environments.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Render Frame Profiling   • Theme & Layout Engine     • Desktop App Sandbox Audit
  • Compositor Optimization  • WCAG 2.1 AA Focus Outlines • Web2App IPC Channel Check
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling (`src/tools/bootloader.rs`), Zenith compositor render frame-rate profiling (`zenith_desktop/`), zero-allocation hot paths.
- **Rules**:
  - Maintain 60+ FPS compositor rendering and eliminate window layout recalculation bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes (`TokyoNight`, `Catppuccin`, `Nord`), boot splash graphics, WCAG 2.1 AA focus visible outlines, ARIA annotations.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across all desktop controls and web console interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 module signatures, desktop process sandbox isolation (`DistrictSandbox`).
- **Rules**:
  - Enforce process isolation for desktop applets and web2app launchers.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. DESKTOP ENVIRONMENT & COMPOSITOR POLICIES (`docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md`)

- **Wayland Ozone Launchers**: Third-party web applications must be launched with Wayland Ozone isolation flags (`--ozone-platform=wayland`).
- **Accessibility Invariants**: All interactive UI elements must render high-contrast focus rings on keyboard TAB focus.

---

## 3. CANARY VALUE MANAGEMENT & SECURITY HARDENING (`docs/AGENTS_CANARY_VALUE_MANAGEMENT.md`)

- **Thread-Local SSP Canaries**: All thread guard values generated by `BinaryProtectionManager` in `src/security/binary_protection.rs` must enforce LSB NUL-byte formatting (`canary & 0xFF == 0x00`) to terminate string buffer overflow attacks.
- **OpenBSD Context Switch Guards**: CPU context switches in `src/kernel/roundrobin.rs` must validate context canary values (`stack_canary`) before restoring execution frames, triggering controlled `__stack_chk_fail` fault handling on mismatch.

---

## 4. CLOUD COMPUTING OPERATIONS MANAGEMENT (`docs/AGENTS_CLOUD_COMPUTING_OPERATIONS_MANAGEMENT.md`)

- **Headless Cloud Targets**: Booting under `SystemTarget::Cloud` (`cloud.target`) in `src/init/sigmainit.rs` must bypass GUI compositors and optimize zero-copy E1000/xHCI network queues (< 16MB RAM footprint).
- **Capability-Gated Cloud-Init**: User-data `#cloud-config` scripts executed by `CloudInitBootstrapEngine` (`src/distro/linux_bsd_parity_extended.rs`) must run inside Ring 3 sandboxes governed by `PledgeManager`.

---

## 5. STATE MANAGEMENT ARCHITECTURE (`docs/AGENTS_STATE_MANAGEMENT.md`)

- **Declarative System State Graph**: State mutations in `src/system/state.rs` must generate immutable generation snapshots supporting $O(1)$ atomic rollback (`rollback()`).
- **Process Lifecycle Machine**: Kernel process state transitions (`src/kernel/process.rs`, `src/kernel/sched/task.rs`) must adhere strictly to valid lifecycle paths (`New` $\to$ `Ready` $\to$ `Running` $\to$ `BlockedWaiting`/`BlockedSuspended` $\to$ `Zombie` $\to$ `Terminated`).

---

## 6. TOP-LEVEL COMPONENT MANAGEMENT (`docs/AGENTS_TOP_LEVEL_COMPONENT_MANAGEMENT.md`)

- **Subsystem Isolation**: Top-level components (Microkernel Core, HAL/Drivers, VFS Storage, Network, Security, Package System, Zenith Compositor, Universal Distro Bridge) must not share mutable raw global state across boundaries.
- **Cross-Subsystem Distro Bridge**: Cross-component interactions route through `SovereignUniversalDistroBridge` (`src/distro/linux_bsd_inspirations.rs`) using capability-gated IPC ring buffers and explicit trait interfaces.

---

## 7. MUTUAL EXCLUSION, MONITORS & PETERSON ALGORITHM (`docs/AGENTS_MUTUAL_EXCLUSION_MONITORS_PETERSON_MANAGEMENT.md`)

- **Peterson's Algorithm Memory Fences**: Software 2-process mutual exclusion (`flag[i] = true; turn = j;`) must issue `core::sync::atomic::fence(Ordering::SeqCst)` to guarantee memory visibility before evaluating `turn`.
- **Monitor Encapsulation**: Monitors (`BoundedBufferMonitor` in `src/kernel/linux_bsd_innovations.rs`) must fully encapsulate shared state, locks, and condition variables, preventing un-monitored direct buffer access.

---

## 8. CONCURRENT PROCESS MANAGEMENT (`docs/AGENTS_CONCURRENT_PROCESS_MANAGEMENT.md`)

- **Atomic PCB State Machine**: PCB state transitions in `src/kernel/process.rs` and `src/kernel/sched/task.rs` must update atomically without lock contention races across CPU cores.
- **Zombie Child Reaping & Signal Safety**: Child processes entering `ProcessState::Zombie` must support `waitpid()` exit status reclamation; forceful signal cancellation (`SIGKILL`) must automatically release held spinlocks and file locks to prevent deadlocks.

---

## 9. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
