# Software Store and Command Palette

## Current capability

`src/desktop/mint_software_store.rs` is a catalog/search and permission-model prototype inspired by Linux Mint Software Manager. Its three seeded listings are illustrative examples: versions, sizes, ratings, and verification are deliberately marked unknown or unavailable. The component does not fetch repository metadata, verify signatures, download packages, install/remove files, or launch applications.

Install, remove, and batch-install methods now return an explicit unavailable-backend error and leave installed state unchanged. Delta-size estimation also reports unavailable until it has measured package-diff data. Permission auditing and hardening modify only the in-memory catalog model; they do not configure a real sandbox.

Tests in the source cover catalog search, fail-closed install/remove behavior, unchanged state after a failed batch operation, and model-only permission operations. Run with:

```sh
cargo test --lib desktop::mint_software_store::tests -- --nocapture
```

## Design references

- **Arch Linux:** keep package build recipes, source, dependencies, and signing metadata inspectable. SigmaOS must not present a package as installable without a verified recipe and repository artifact.
- **Linux Mint:** make search, install impact, permission choices, update status, and recovery understandable in the UI. Human-readable catalog presentation is not evidence that installation works.
- **Omarchy:** expose keyboard-first application discovery and documented commands. Keep palette actions limited to operations that are wired to real system services.

## Roadmap

1. Define a signed repository format, reproducible package recipe, provenance fields, and explicit supported architecture/repository policy.
2. Implement a transactional backend with signature and dependency validation, staged writes, cancellation, rollback, and durable installed-state reconciliation.
3. Connect the UI to that backend; show unknown metadata as unknown and permission changes before install.
4. Implement updates and rollback, including interrupted-transaction recovery and offline repair.
5. Add package fixture tests for corrupt signatures, missing dependencies, interrupted writes, and rollback; then test clean installs and updates on disposable QEMU disks.
6. Add an Omarchy-inspired keyboard command palette only after application launching and system actions have working service APIs and failure reporting.

No package manager, application installation, sandbox enforcement, or command palette integration is currently supported. Gate each capability on its implementation and end-to-end evidence, following the project-wide release plan.
