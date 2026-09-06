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

## 10. Task Assignment & Governance Directives
* **Assignment Guide:** Follow `docs/AGENTS_ASSIGNMENT_MANAGEMENT_GUIDE.md` for task routing, triage protocols, subagent delegation rules, and submission criteria.
* **Persona Routing:** Route security tasks to **Sentinel**, UI/a11y to **Palette**, performance to **Bolt**, and distro infrastructure to **Jules**.

## 11. Information Management & Knowledge Base Directives
* **Information Guide:** Adhere to `docs/AGENTS_INFORMATION_MANAGEMENT_GUIDE.md` for knowledgebase lookup protocols, memory recording requirements, and context prioritization (User Directives > Source Code State > Memory Context).
* **Memory Recording:** Always call `initiate_memory_recording` before completing a task or submitting code.

## 12. Documentation & Wiki Alignment
* **In-Tree Troff Man Pages:** Keep `docs/man/man1/` and `docs/man/man8/` troff manual pages up to date when modifying commands or system utilities.
* **Wiki Sync Utility:** Run `./scripts/sync_wiki.sh` after updating documentation assets to synchronize files across `WIKI/`, `wiki/`, and `wiki_repo/`.
