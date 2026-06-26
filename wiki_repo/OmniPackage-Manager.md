# OmniPackage Manager (OmniPkg)

The OmniPackage Manager is SigmaOS's declarative, cryptographically-secure software distribution tool, rivaling `apt` and `Nix`.

## Features
- **Transactional Rollbacks**: Installations are treated as atomic transactions. If a package fails to install or verify, the system reverts to the pre-transaction state without leaving orphaned files.
- **Strict Verification**: Every package must be signed. `OmniPkg` integrates directly with the Dilithium-5 PQC shard to verify lattice signatures before unpacking software.
- **POSIX Shim Compatibility**: While native `.omni` packages are preferred, the package manager contains logic to sandbox and run legacy Linux binaries via our kernel's `POSIXShim`.
