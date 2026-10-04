# Packaging and Updates

**Capability state: Prototype.** This is the canonical page for package management, software discovery, updates, reference-project comparisons, validation, and the component roadmap. Shared status terms are defined in the [status vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The source tree contains package metadata, parsing, and transaction-related models. There is no verified end-to-end package workflow that fetches trusted repository metadata, resolves dependencies, validates signatures, installs files transactionally, updates a running system, and recovers after interruption. Package formats are not supported based on filename recognition or parsing alone.

The software-store catalog is a fail-closed prototype. It can present metadata, but installation and removal return an unavailable-backend error. See [Software Store and Command Palette](Software-Store-and-Command-Palette.md).

`src/desktop/mint_update_manager.rs` contains an in-memory update policy model. It no longer invents sample updates, CVEs, or repository mirrors. Refresh yields no candidates until a repository backend exists; mirror selection uses only supplied latency values and does not probe the network. Update application returns an error without changing candidates or claiming that a snapshot was created. These behaviors are covered by the standalone test suite. The model is not a live update manager.

No `sigpkg` command listed in older proposals should be treated as available unless it is present in the built system and tested end to end.

## Design references

- **Arch Linux:** inspectable package recipes, explicit dependencies, and task-focused documentation.
- **Linux Mint:** approachable software discovery, understandable update status, and recovery guidance. See the [Linux Mint documentation](https://linuxmint.com/documentation.php).
- **Omarchy:** deliberate defaults and keyboard-first navigation for user-facing software discovery; see its [hotkey guide](https://github.com/basecamp/omarchy/blob/quattro/manual/07-hotkeys.md).
- **FreeBSD:** signed repository metadata and explicit trust boundaries.

These references guide future work; they do not imply feature parity.

## Validation

The current update-policy model can be tested with:

```sh
rustc --test --edition=2021 src/desktop/mint_update_manager.rs -o /tmp/sigmaos_update_policy_tests
/tmp/sigmaos_update_policy_tests
```

These tests cover safety-tier filtering, supplied mirror-latency selection, and fail-closed behavior. They do not validate network access, signatures, package transactions, snapshots, or rollback.

## Roadmap

1. Define one canonical package manifest and trusted repository metadata format.
2. Use an audited cryptographic provider; reject invalid, missing, or unverifiable signatures.
3. Connect update discovery to a signed repository and show only verified candidates with source, version, size, and reboot requirements.
4. Implement deterministic dependency resolution with explicit conflict and cycle errors.
5. Stage file changes transactionally; create a real pre-update recovery point and prove rollback after injected failures and interruption.
6. Connect software discovery to the installer and desktop only after package and update backends work.
7. Publish a tested package-format support matrix and beginner-readable update and recovery guides.

**Completion evidence:** clean-install tests cover successful installation, dependency failures, signature rejection, interrupted transactions, update rollback, and recovery. A format is supported only after its complete path is validated.

## Related pages

- [Security](07-Security.md)
- [Boot and installation](01-Installation.md)
- [Desktop](08-Desktop.md)
- [Software Store and Command Palette](Software-Store-and-Command-Palette.md)
- [Testing](Testing.md)
