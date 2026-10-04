# Future Development: Packaging, Installation, and Updates

**Status:** Proposal. Package-format recognition, manifests, or workflow models do not mean that packages can be securely installed or updated.

## Scope

Plan reproducible package inputs, dependency resolution, signature verification, transactional installation, rollback, and recovery. Current source areas include [`package`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/package), [`sigpkg`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/sigpkg), and [`installer`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/installer).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| NixOS | Declarative system generations and atomic rollback | Can a complete system generation be prepared and activated as one transaction? |
| Debian | Package metadata, dependency rules, and unattended installation | Which maintainer actions must be sandboxed or rejected? |
| Arch Linux | Transparent build recipes and user-maintained package workflows | How can recipes be reviewed, reproducibly built, and kept separate from trusted base packages? |
| Gentoo | Explicit build options and profile-controlled features | Can feature choices be represented in reproducible manifests? |
| FreeBSD | Signed package catalogs and isolated port builds | How should repository metadata, build provenance, and install-time policy be verified? |
| Alpine Linux | Small packages and a compact base system | How can minimal installations stay recoverable and auditable? |

## Proposed work sequence

1. **Define a canonical manifest.** Include name, version, target, dependencies, payload entries, permissions, and provenance. Reject ambiguous or missing fields.
2. **Implement real verification.** Verify repository metadata and payload digests with an audited provider. Never use fixed strings or empty-file digests as package signatures.
3. **Make dependency resolution deterministic.** Define version constraints, conflicts, ordering, cycle reporting, and resource limits.
4. **Sandbox build and maintainer actions.** Specify filesystem, process, and network permissions; default to denial and record every granted capability.
5. **Make installation transactional.** Stage changes, validate them, commit atomically, and recover to the prior generation after failure or power loss.
6. **Build a recoverable installer.** Validate target disks, boot configuration, and user input before writing; provide a recovery path when installation is interrupted.

## Completion criteria

- Invalid signatures, missing providers, malformed manifests, and unavailable repositories stop the operation with a clear error.
- A failed package install leaves the previous system state usable.
- Repeated builds from the same declared inputs produce verifiable equivalent outputs.
- Build recipes and maintainer scripts cannot access undeclared host resources.
- Installer writes are scoped to the selected target and can be recovered after interruption.

## Maintenance

Keep format support, repository support, cryptographic verification, build execution, and installation readiness as separate status claims. Link each supported format to its parser and verified installation path.
