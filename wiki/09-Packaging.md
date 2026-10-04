# Packaging and Updates

**Capability state: Prototype.** This is the canonical page for package management, updates, reference-project comparisons, validation, and the component roadmap. Shared status terms are defined in the [status vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The source tree contains package metadata, parsing, and transaction-related models. That does not establish an end-to-end package manager. There is no verified supported workflow that fetches a trusted repository, resolves dependencies, validates signatures, installs files transactionally, updates a running system, and recovers after interruption.

The software-store catalog is currently a fail-closed prototype: it can present catalog entries but cannot install software. See [Software Store and Command Palette](Software-Store-and-Command-Palette.md). Commands and repository URLs in older proposals are examples only and must not be treated as available SigmaOS commands or live repositories.

Package formats are **not declared supported** based on filename recognition, archive parsing, metadata models, or tests that do not exercise a real install backend.

## Design references

- **Arch Linux:** inspectable package recipes, explicit dependencies, and precise documentation.
- **Linux Mint:** approachable software discovery, update status, and user recovery guidance.
- **NixOS:** transactional generations and rollback, after the system has a verified persistent storage path.
- **FreeBSD:** signed repository metadata and explicit trust boundaries.

These references guide future work; they do not imply feature parity.

## Roadmap

1. Define one canonical package manifest and trusted repository metadata format.
2. Use an audited cryptographic provider; reject invalid, missing, or unverifiable signatures.
3. Implement deterministic dependency resolution with explicit conflict and cycle errors.
4. Stage file changes transactionally and prove rollback after injected failures and interruption.
5. Connect software discovery to the installer, update flow, and recovery path only after those backends work.
6. Publish a tested package/format support matrix and a beginner-readable update guide.

**Completion evidence:** clean-install tests cover successful installation, dependency failures, signature rejection, interrupted transactions, update rollback, and recovery. A package format is supported only after the complete path is validated.

## Related pages

- [Security](07-Security.md)
- [Boot and installation](01-Installation.md)
- [Desktop](08-Desktop.md)
- [Testing](Testing.md)
