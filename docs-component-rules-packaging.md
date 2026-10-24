# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman / PKGBUILD
- Debian dpkg / APT & Dilithium-5 PQC
- NixOS Declarative Package Store

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Packaging, Dependency Resolution, and Metadata components.

## Mandatory rules

1. Package metadata must be fully parsed and validated before any installation or script execution action.
2. Dependency resolution must support dry-run mode (`--dry-run`) without modifying filesystem state.
3. Package post-install scripts must run only in a restricted sandbox with declared minimal capabilities.
4. Updates must be atomic or recoverable; failed package installations must restore prior known-good state.
5. Package signatures must be verified against trusted PQC keys before unpacking.

## Prohibited

- Executing maintainer scripts unsandboxed with root privileges
- Modifying filesystem files during dry-run dependency checks
- Swallowing package metadata parse errors or checksum mismatches
- Partial package installs leaving broken dependency graphs
- Status promotion without dry-run and sandbox evidence logs

## Required evidence

- Package metadata parser unit test reports
- Dependency solver dry-run integration test logs
- Sandboxed package script execution report
- PQC signature verification test logs

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Package specification ADR and format definition |
| Prototype | Metadata parser and solver unit tests |
| Integrated | Dry-run installation and sandboxed script execution tests |
| Hardened | Malicious package corpus fuzzing and security review |
| Supported | Reproducible repository index & package installation evidence |
