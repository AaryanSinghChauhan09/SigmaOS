# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman & PKGBUILD
- Debian dpkg, apt, & debconf
- Fedora RPM & dnf

## Purpose
Define package metadata parsing, dependency resolution safety, sandboxed hook execution, and update atomicity rules for SigmaOS package management.

## Mandatory rules

1. Package metadata must be parsed and verified before any installation or system action.
2. Dependency resolution must support dry-run mode (`--dry-run`) without modifying system state.
3. Package maintainer scripts must run only in a sandboxed container with declared minimal capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.
5. All package signatures must be cryptographically validated (e.g. Dilithium-5 PQC or GPG) prior to unpacking.

## Prohibited

- Unsandboxed execution of package lifecycle scripts (`preinst`, `postinst`, `prerm`, `postrm`)
- Destructive state changes during failed package dependency resolution
- Bypassing checksum or signature verification
- Borrowed-status claims matching Arch or Debian without local test execution

## Required evidence

- Package metadata parser and solver unit tests (`src/package/universal.rs`)
- Package dependency solver dry-run execution report
- Sandbox capability denial test log for package hook runners

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Package format ADR and dependency solver design |
| Prototype | Unit tests for metadata parser and dry-run solver |
| Integrated | Sandboxed installation and hook runner integration tests |
| Hardened | Malformed package fuzzing, PQC signature verification |
| Supported | Production package repository test evidence and install logs |
