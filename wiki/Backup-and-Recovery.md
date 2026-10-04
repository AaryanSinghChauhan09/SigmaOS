# Backup and Recovery

**Capability state: Prototype.** This is the single canonical page for SigmaOS backup manifests, personal-data backup, restore, references, validation, and roadmap. No end-to-end personal-data backup or recovery workflow is currently supported.

## Current capability

`src/desktop/mint_backup_tool.rs` implements an in-memory package-selection manifest. It can register validated package tokens, export a deterministic tab-separated v1 manifest, and parse the manifest while rejecting malformed rows and duplicate package names. Import returns the explicitly selected package names; it does not install packages.

Personal-data archive creation is not implemented. It returns an explicit unavailable-backend error and leaves backup history unchanged. The code does not walk source files, produce an archive, compute a checksum, or restore user data. Archive metadata types and exclusion patterns are models only.

## Design references

- **Linux Mint:** approachable backup and restore workflows with explicit source and destination choices.
- **Arch Linux:** inspectable package selection and configuration data.
- **SigmaOS:** do not claim a backup exists until file content has been written, verified, and restored successfully.

References provide design guidance; they do not imply support or parity.

## Validation

Run the standalone tests with:

```sh
rustc --edition=2021 --test src/desktop/mint_backup_tool.rs -o /tmp/sigmaos_backup_tests
/tmp/sigmaos_backup_tests
```

These tests cover manifest round-trip, unsafe field rejection, malformed and duplicate rows, and the archive operation's fail-closed behavior. They do not test filesystem backup or restore.

## Roadmap

1. Define a supported backup source/destination model and safe file traversal rules, including symlinks, exclusions, permissions, and unreadable files.
2. Select a maintained archive and cryptographic checksum implementation; never emit placeholder sizes or hashes.
3. Write archives atomically to a temporary destination, verify them, then publish the completed archive.
4. Add a dry-run restore plan with path-traversal defenses, overwrite confirmation, and interruption-safe restore behavior.
5. Restore package selections only through a verified repository and the transactional package backend.
6. Test backup and restore with disposable directories, injected I/O failures, corruption, cancellation, and recovery after interruption.

**Completion evidence:** a clean test environment backs up real files, verifies the archive, restores to a separate directory, detects corruption, rejects unsafe paths, and leaves source and destination data unchanged after injected failures.

## Related pages

- [Boot and installation](01-Installation.md)
- [Packaging and updates](09-Packaging.md)
- [Security](07-Security.md)
- [Testing](Testing.md)
