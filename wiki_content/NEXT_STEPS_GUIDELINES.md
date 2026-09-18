# Next Steps & Continuous Improvement Guidelines for SigmaOS

## Overview
This document outlines the operational protocols, architectural guidelines, and prioritized execution steps for maintainers and AI agents working on **SigmaOS** (`https://github.com/AaryanSinghChauhan09/SigmaOS/`).

---

## 1. Development & Direct Commit Workflow
* **No PR Policy**: All verified changes must be committed directly to the `main` branch. Pull requests should not be opened.
* **Verification First**: Before committing, run `bash run_sigma_tests.sh` to confirm that all 67 native unit test suites pass without regression.
* **Clean Compilation**: Ensure `cargo check --lib --tests` runs with zero syntax or unresolved symbol errors.

---

## 2. Tri-Agent Steering System Protocols

### ⚡ Bolt Agent Protocol (Performance & Optimization)
1. **Identify Bottlenecks**: Focus on inner loops, memory allocations, and linear scans in package resolution and IPC buffers.
2. **SIMD & Vectorization**: Prefer `copy_from_slice`, `extend_from_slice`, and zero-copy slicing over manual byte-copy loops.
3. **Map Query Reduction**: Hoist repeated map/hash lookups out of nested loops to achieve $O(N \log N)$ or $O(N)$ complexity.
4. **Journaling**: Record unexpected performance learnings in `.jules/bolt.md`.

### 🎨 Palette Agent Protocol (Micro-UX & Accessibility)
1. **WCAG 2.1 AA Compliance**: Ensure every interactive UI element in Zenith Web Desktop (`zenith_desktop/`) includes proper `aria-label`, `type="button"`, and keyboard focus visible indicators.
2. **Keyboard Navigation**: Maintain logical tab index ordering across windows, system tray, and desktop shortcuts.
3. **Responsive Spacing**: Use semantic CSS classes and standard design tokens without adding custom inline overrides.
4. **Journaling**: Record accessibility insights and UX constraints in `.jules/palette.md`.

### 🛡️ Sentinel Agent Protocol (Security & Compliance)
1. **Zero Secret Policy**: Scan all files for hardcoded credentials, API keys, or private certificates before pushing.
2. **Memory Safety & Scrubbing**: Enforce `zeroize_memory` across user namespaces, key vaults, and process control blocks upon process exit.
3. **Capability Sandboxing**: Apply OpenBSD `pledge`/`unveil` syscall restrictions and Linux Landlock LSM rules to isolated worker tasks.
4. **Post-Quantum Cryptography**: Maintain FALCON-1024 and Dilithium-5 signature verification across package manifests.
5. **Journaling**: Record security vulnerability learnings in `.jules/sentinel.md`.

---

## 3. Prioritized Implementation Roadmap

### Phase 1: High-Priority Engineering (Immediate)
1. **Modular Subsystem Decomposition**:
   - Refactor monolithic files (`src/compatibility/fedora.rs` and `src/package/universal.rs`) into clean subdirectories (`src/compatibility/fedora/` and `src/package/universal/`).
2. **Lock-Free IPC Ring Buffers**:
   - Replace mutex-guarded process queues in `src/process/` with SPSC atomic ring buffers modeled on Linux `kfifo`.
3. **24-Hour PQC Key Rotation**:
   - Automate scheduled post-quantum cryptographic key rotation in `src/security/secrets.rs`.

### Phase 2: Medium-Priority Refactoring (Short-Term)
1. **High-Contrast Desktop Themes**:
   - Add `@media (forced-colors: active)` support in `zenith_desktop/zenith_desktop.css` for high-contrast accessibility.
2. **Encapsulate PML4 Page Tables**:
   - Refactor bare CR3 register operations in `src/kernel/vmm.rs` into an encapsulated `VmmSpace` object.

### Phase 3: Low-Priority Maintenance (Ongoing)
1. **Wiki Bidirectional Sync Automation**:
   - Maintain `tools/sync_wiki.sh` to synchronize Markdown specifications across `docs/`, `wiki/`, `WIKI/`, and `wiki_repo/`.

---

*SigmaOS Maintainer Guidelines — Main Branch.*
