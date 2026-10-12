# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman & ALPM library
- Debian dpkg & apt-resolver
- Alpine APK3 package format

## Purpose
Define package metadata parsing, dry-run dependency resolution, sandboxed script execution, and atomic operations.

## Mandatory rules
1. Package metadata must be parsed and verified before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.
5. All package signatures must be validated against a trusted PQC or OpenPGP keyring.

## Prohibited
- Unsandboxed execution of post-install scriptlets
- Partial or unrecoverable package extraction
- Ignoring checksum mismatch errors
- Borrowed parity claims without local test evidence

## Required evidence
- Metadata parser unit test results
- Dry-run dependency resolution report
- Sandboxed installation test log
- Atomic rollback verification log on simulated failure

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Package format and resolver spec ADR |
| Prototype | Metadata parser and unit tests |
| Integrated | Dry-run & sandboxed install in runtime path |
| Hardened | Corpus fuzzing, signature verification, failure injection |
| Supported | Reproducible package install & rollback evidence |
