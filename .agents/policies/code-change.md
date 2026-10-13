# Code Change Policy for Agents

## Rules for Code Modifications

1. **Pull Request Isolation**: Every change must arrive via a dedicated feature branch (`feat/`, `fix/`, `test/`, `docs/`, `chore/`) and Pull Request.
2. **Conventional Commits**: Commit messages must follow the standard format: `<type>(<scope>): <short description>`.
3. **Smallest Safe Increment**: Agents must modify the minimum number of lines required to complete a task. Avoid reformatting unrelated files.
4. **No Interface Breaks**: Subsystem agents must not alter public module traits or cross-subsystem ABI contracts without an explicit integration review.
5. **Mandatory Tests**: Every new feature or bug fix must include unit tests and regression tests proving correctness.
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
