# SigmaOS Master Improvement Plan

## Domain 1: Code Quality & Testing
- Enforce clean Rust compilation (`cargo check --lib`) and zero-warning policy on hot paths.
- Maintain full passing status for pytest suite (`pytest tests/`).
- Enforce standalone unit testing for modular low-level components.

## Domain 2: Performance (Bolt ⚡)
- Implement page-frame aligned 32-byte slab descriptors (`PackageHeader32ByteDescriptor`).
- Replace $O(N^2)$ linear searches in package resolution with $O(N)$ hash maps.
- Maintain journal in `.jules/bolt.md`.

## Domain 3: UX & Accessibility (Palette 🎨)
- Standardize high-contrast terminal focus states and CLI visual feedback.
- Add keyboard accessibility shortcuts and semantic formatting.
- Maintain journal in `.jules/palette.md`.

## Domain 4: Security & Compliance (Sentinel 🛡️)
- Prevent unaligned memory access in hardware structures (`TaskStateSegment64`).
- Enforce input sanitization on all syscall/IPC boundaries.
- Maintain journal in `.jules/sentinel.md`.

## Domain 5: 500+ Repositories Absorption Strategy
- Execute 4-phase chronological absorption strategy covering kernels, distributions, package managers, container runtimes, hypervisors, and filesystems.
