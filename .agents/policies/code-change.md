# Code Change Policy for Autonomous Agents

## 1. Single Ownership & Boundaries
- Each agent owns specific directory paths defined in `.agents/registry.toml`.
- An agent MUST NOT edit files outside its declared ownership boundary without initiating an integration review and cross-boundary test task.

## 2. Granular, Atomic Pull Requests
- PRs must address exactly one concern: one bug, one test scenario, one document, or one subsystem integration.
- Monolithic "improve everything" PRs are prohibited and will be rejected by the Repository Steward.

## 3. Conventional Commit & Branch Naming Standards
Every commit and branch name must follow Conventional Commits:
- `feat(<subsystem>): <description>`
- `fix(<subsystem>): <description>`
- `test(<subsystem>): <description>`
- `docs(<subsystem>): <description>`
- `chore(<subsystem>): <description>`

Branch naming convention:
- `feat/<subsystem>-<topic>-<id>`
- `fix/<subsystem>-<issue>-<id>`
- `test/<subsystem>-<scenario>-<id>`
- `study/<subsystem>-<topic>-<id>`

## 4. Upstream Reference Discipline
- SigmaOS explicitly draws inspiration from Linux, Arch Linux, Linux Mint, Redox, xv6, FreeBSD, and OpenBSD.
- However, code presence or reference project alignment DOES NOT imply feature parity or bootability in SigmaOS.
- Claims of parity must be backed by unit, integration, and QEMU runtime evidence.
