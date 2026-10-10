# Contributing to SigmaOS

Thank you for your interest in contributing to **SigmaOS**! SigmaOS is a high-performance, capability-based microkernel operating system written in Rust with strict `#![no_std]` core enforcement.

---

## Core Architectural Directives

All contributions to SigmaOS must strictly adhere to the following architectural rules:

1. **`no_std` Core Enforcement**:
   - All kernel modules, drivers, and core subsystems must compile under `#![no_std]`.
   - Direct imports of `std` are prohibited outside test targets (`#[cfg(test)]`).

2. **Capability-Based Security Model**:
   - Every syscall entrypoint must verify the caller's `CapabilityToken` before executing privileged kernel or resource operations.
   - Capability tokens must be explicit, unforgeable, and checked at API boundaries.

3. **WDM-Style Driver Architecture**:
   - Drivers must follow the Windows Driver Model (WDM) pattern: `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension`.
   - Every driver submission must include a standalone lifecycle unit test verifying driver creation, device attachment, dispatch handling, and teardown.

4. **Paged vs NonPaged Memory Pools**:
   - Interrupt service routines (ISRs) and core kernel structures must use `NonPaged` memory pools.
   - User-pageable buffers and background data must utilize `Paged` memory pools with bounds-checked operations (`copy_nonoverlapping`).

5. **Explicit Type Annotations**:
   - Public API functions, constants, and key collection types must feature explicit type annotations.

6. **Standalone Unit Testing**:
   - Standalone module tests must be runnable via `rustc --test`.
   - Workspace build scripts like `./scripts/changed_files_rustc_tests.sh` execute standalone tests per modified file to ensure high performance and isolated verification.

---

## Development Workflow

### Prerequisites
- **Rust Toolchain**: `stable` toolchain with `rustfmt` and `clippy`.
- **Build Utilities**: `python3`, `qemu-system-x86_64`, `gcc`, `make`.

### Fast Checks & Pre-Commit Testing
Before submitting a pull request, run the local quality checks:

```bash
# Code formatting check
cargo fmt --check

# Clippy lint check with denied warnings
cargo clippy --all-targets --all-features -- -D warnings

# Check no_std compliance across kernel modules
./scripts/no_std_check.sh

# Run standalone rustc unit tests for modified files
./scripts/changed_files_rustc_tests.sh

# Execute complete test suite
./run_sigma_tests.sh
```

---

## Submitting Pull Requests

1. **Branch Naming**: Use descriptive branch names like `feature/virtio-blk-driver`, `fix/capability-check-syscall`, or `docs/architecture-update`.
2. **PR Checklist**: Complete all items in `.github/PULL_REQUEST_TEMPLATE.md`.
3. **Commit Messages**: Write clear, concise commit messages following standard conventions:
   ```
   subsystem: brief summary of changes (50 chars max)

   Detailed explanation of the problem solved, design choices made,
   and test verification steps completed.
   ```

---

## Open-Source OS Idea Adoption Process

SigmaOS actively studies and adopts best-in-class concepts from open-source operating systems (Redox, seL4, Tock OS, Fuchsia, WASI, Linux, BSD). If you want to propose or port a feature:
1. Open an issue using the **Open-Source OS Idea Adoption** template (`.github/ISSUE_TEMPLATE/oss_idea_adoption.md`).
2. Timebox a 1–2 sprint prototype to demonstrate feasibility and measure performance/footprint metrics.
3. Submit a PR referencing the proposal issue once unit tests and architectural checklists pass.
