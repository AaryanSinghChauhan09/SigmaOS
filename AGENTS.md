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

## 5. Information Management & Knowledge Base Directives
* **Information Guide:** Adhere to `docs/AGENTS_INFORMATION_MANAGEMENT_GUIDE.md` for knowledgebase lookup protocols, memory recording requirements, and context prioritization (User Directives > Source Code State > Memory Context).
* **Memory Recording:** Always call `initiate_memory_recording` before completing a task or submitting code.

## 6. Documentation & Wiki Alignment
* **In-Tree Troff Man Pages:** Keep `docs/man/man1/` and `docs/man/man8/` troff manual pages up to date when modifying commands or system utilities.
* **Wiki Sync Utility:** Run `./scripts/sync_wiki.sh` after updating documentation assets to synchronize files across `WIKI/`, `wiki/`, and `wiki_repo/`.
