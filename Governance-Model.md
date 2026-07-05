# SigmaOS Governance Model

SigmaOS uses a transparent, community-driven governance model with clear roles and a lightweight RFC process.

---

## Roles

### Core Maintainers

- Full commit access to `main`

- Approve and merge PRs

- Set release schedules

- Triage and respond to security issues

### Module Owners

- Domain experts for a specific subsystem (kernel, security, net, drivers, UI)

- Must approve PRs touching their module

- Listed in `.github/CODEOWNERS`

### Contributors

- Submit PRs to feature branches

- Participate in design discussions via Issues/Discussions

- Write docs, tests, drivers, subsystem code

### Community Members

- File bug reports and feature requests

- Help other users in Discussions

- Test pre-release builds

---

## Decision Making

| Change Type | Approval Required |
|-------------|------------------|
| Bug fix / docs | 1 maintainer |
| New feature (single subsystem) | 1 maintainer + CI green |
| Cross-subsystem change | 2 maintainers + CI green |
| Architecture decision | RFC process (7-day discussion minimum) |
| Release | Core maintainers consensus |
| Security patch | Expedited — 1 maintainer, private review |

---

## RFC Process

1. Open a GitHub Issue titled `RFC: <topic>`

2. Tag it with the `rfc` label

3. Minimum 7-day open discussion period

4. Core maintainers vote: accept / reject / revise

5. Accepted RFC → moved to `docs/rfcs/<rfc-NNN>-<slug>.md`

6. Implementation tracked in linked PR

---

## Branch Policy

| Branch | Protection |
|--------|-----------|
| `main` | Protected — PR + CI required |
| `release/*` | Protected — no direct push |
| `kernel-exp`, `drivers-dev`, `fs-dev` | Open — experimental |
| `tools-dev`, `docs-update` | Open — low-risk |

---

## Release Cadence

| Release Type | Cadence | Criteria |
|-------------|---------|---------|
| Patch (v15.0.x) | As needed | Security + critical bugs |
| Minor (v15.x.0) | ~3 months | New features, all CI green |
| Major (v16.0.0) | ~9 months | Milestone achieved (e.g., bootable ISO) |
| LTS | Per major | Extended security support |

---

## Code of Conduct

All participants follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT).
Violations: contact maintainers privately via GitHub Security Advisory.

---

*Full governance doc: [GOVERNANCE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/GOVERNANCE.md)*
