# AGENTS.md — AI Agent Development Guidelines for SigmaOS

Welcome, AI Agent! This guide specifies the rules, conventions, architectural patterns, and testing protocols required when contributing code, fixing bugs, or implementing new features in **SigmaOS**.

---

## 1. System Architecture & Core Directives

SigmaOS is a next-generation sovereign operating system built in Rust. It combines microkernel resilience, zero-dependency architecture (`klib`), universal multi-distro packaging adapters, and advanced Linux & BSD subsystem compatibility.

### Key Architectural Layers
* **Kernel & Memory (`src/kernel/`, `src/memory/`, `src/klib/`)**: Low-level kernel scheduling, paging, physical/virtual memory allocators, IPC channels (`SovereignPipe`), and the zero-dependency standard library (`klib`).
* **Hardware Abstraction & Drivers (`src/hal/`, `src/drivers/`, `src/hardware/`)**: Multi-architecture HAL (x86_64, AArch64, RISC-V), PCIe bus enumerator, NVMe, AHCI, USB, Intel HDA, and Wi-Fi drivers.
* **Packaging & SigPkg (`src/sigpkg/`, `src/package/`)**: Universal OOP package adapters parsing `.deb`, `.rpm`, `.apk`, `.xbps`, `AppImage`, `.snap`, `Flatpak`, `.hpkg`, `.eopkg`, `.ebuild`, `.tar.zst`, etc., with dependency mapping and sandbox capability translation.
* **Distro & Subsystem Innovations (`src/distro/`, `src/compatibility/`)**: Parity modules for OpenSUSE (Snapper), Pop!_OS (System76 scheduler), DragonFly BSD (HAMMER2 PFS), Void Linux (runit/XBPS), OpenBSD (Pledge/Unveil/Retguard), SmartOS (Zones), NixOS (Declarative Store), Omarchy (Hyprland/themes/PWA/Nerd Fonts), Solus (Stateless defaults), NetBSD (RUMP drivers), HardenedBSD (PaX/CFI), Garuda (Auto-CPU-FREQ/zram), and Debian (Multiarch/APT pinning).
* **Desktop Environments (`src/desktop/`, `src/ui/`, `src/customization/`)**: Zenith Wayland compositor, KDE Plasma 6 tiling/KRunner, GNOME 46 Mutter scaling/extensions, XFCE 4.18 panel IPC, Lumina BSD desktop, Sway/Regolith tree tiling, Mint Cinnamon presets, Plymouth bootsplash, and Folder Color switchers.
* **AI & Data Science Subsystem (`src/ai/`)**: Zero-alloc tensor memory management, compute scheduling, FP32/FP16/INT8/INT4 quantization, local LLM GGUF execution, Whisper speech-to-text, Scikit-learn/mlpack algorithms (K-Means, PCA), Jupyter notebook engine, and MLflow/DVC tracking.

---

## 2. Coding Conventions & Code Quality Rules

### Language & Alloc Safety
* **Rust Edition**: Rust 2021 edition.
* **`no_std` vs Standard Allocation**: Core data structures in `klib/` and `kernel/` must avoid standard library reliance where appropriate, using `alloc::vec::Vec`, `alloc::string::String`, or custom zero-alloc structures.
* **No Unsafe Sprawl**: Keep `unsafe` blocks strictly limited to necessary hardware MMIO, MSR access, context switching, or low-level DMA ring buffers. Every `unsafe` block must be accompanied by a safety comment explaining invariants.

### Module Export & Namespace Structure
* **File & Module Naming**: Use snake_case for file names (e.g. `ultimate_distro_innovations.rs`) and PascalCase for structs/enums/traits.
* **Re-exports**: Always register new modules in their parent `mod.rs` (e.g., `src/distro/mod.rs` or `src/desktop/mod.rs`) and re-export public structs/traits in `src/lib.rs` if they represent core system abstractions.

### Error Handling
* Prefer explicit `Result<T, &'static str>` or custom domain error enums (e.g., `ConfigError`, `CompatibilityError`) over `panic!`.
* For closures mapping errors, explicitly specify error types when required (e.g., `map_err(|e: std::io::Error| ...)`).

---

## 3. Testing Protocols & Verification Procedures

### Running the Full Test Suite
SigmaOS includes a comprehensive test runner script `./run_sigma_tests.sh` that executes:
1. Python integration test suite (`pytest tests/`).
2. Package Caching Engine unit tests (`src/package/cache.rs`).
3. Universal Package Adapter tests (`tests/test_universal_adapter.rs`).
4. Standalone sub-suite tests (`src/unimplemented_features.rs`, `src/unimplemented_tools.rs`, `src/open_source_os_gap_closure.rs`, `src/expanded_wiki_innovations.rs`, etc.).
5. UI/UX accessibility tests and security input validation tests (`src/security/input_validation.rs`).
6. Core Rust unit test suite (`cargo test --lib`).

To execute all tests:
```bash
./run_sigma_tests.sh
```

### Rules for Standalone Rust Tests (`rustc --test`)
When writing or modifying standalone test files compiled directly with `rustc --test <filepath>` (e.g., `src/open_source_os_gap_closure.rs` or `src/sigpkg/universal_adapter.rs`):
* Include explicit fallback imports for `HashMap`, `Box`, `Vec`, `String` under `#[cfg(test)]`.
* Verify that test structures and methods match the public API of the main module.

---

## 4. Distro & Subsystem Feature Addition Guidelines

When adding features inspired by Linux or BSD distributions:
1. **Research & Parity**: Identify the defining mechanism (e.g., Solus stateless defaults, NetBSD RUMP drivers, HardenedBSD PaX/CFI, Omarchy Hyprland themes).
2. **Modular Placement**: Place the implementation in an appropriate file under `src/distro/`, `src/desktop/`, `src/compatibility/`, or `src/sigpkg/`.
3. **Unit Test Coverage**: Include inline `#[cfg(test)] mod tests` at the bottom of the implementation file verifying all new methods, edge cases, and error returns.
4. **Documentation**: Document struct fields and public methods using idiomatic `///` doc comments.

---

## 5. Pre-Commit Checklist for AI Agents

Before submitting code changes:
1. **Run Tests**: Execute `./run_sigma_tests.sh` to confirm 100% test pass rate with zero regressions.
2. **Request Code Review**: Call `request_code_review` tool to obtain automated verification feedback.
3. **Record Learnings**: Call `initiate_memory_recording` tool to document key patterns and implementations.
4. **Final Submission**: Ensure git working tree is clean and call `submit` with a clear, standard commit message.
