# SigmaOS Release Policy

## Release Tiers

| Release Tier | Acceptance Criteria |
|---|---|
| **Experimental** | Kernel/subsystem unit tests pass cleanly |
| **Developer Preview** | Automated QEMU boot reaches shell marker (`[SHELL_PROMPT_READY]`) |
| **Test Distribution** | Installer dry-run and failure injection recovery rehearsals pass |
| **Release Candidate** | Security audit complete, known issues documented, SBOM generated |
| **Stable** | Hardware support matrix audited, atomic update/rollback verified |

## Versioning Scheme

SigmaOS follows Semantic Versioning (`MAJOR.MINOR.PATCH`):
- `MAJOR`: Incompatible kernel ABI / syscall changes.
- `MINOR`: New backwards-compatible kernel features or desktop innovations.
- `PATCH`: Security fixes, bug repairs, and maintenance updates.
