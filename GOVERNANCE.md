# SigmaOS Governance Model

## Project Structure

SigmaOS is an open-source sovereign OS project led by a core maintainer
team with a transparent, community-driven contribution model.

## Roles

### Core Maintainers
- Full commit access to `main`
- Approve and merge PRs
- Set release schedules
- Triage security issues

### Contributors
- Submit PRs to feature branches
- Participate in design discussions via GitHub Issues/Discussions
- Write docs, tests, drivers, and subsystem code

### Community Members
- File bug reports and feature requests
- Help other users in Discussions
- Test pre-release builds

## Decision Making

1. **Small changes** (bug fixes, docs): single maintainer approval
2. **Subsystem changes** (kernel, drivers, security): two maintainer approvals + CI green
3. **Architecture decisions**: RFC process (see `docs/RFC_Template.md`)
4. **Release decisions**: core maintainers consensus

## RFC Process

1. Open an issue titled `RFC: <topic>`
2. Add the `rfc` label
3. Discussion period: minimum 7 days
4. Approved RFCs move to `docs/rfcs/`
5. Implementation tracked in linked PR

## Branch Policy

- `main` — protected, requires PR + CI green
- `release/*` — maintained in sync with `main` via S-BUSE pipeline
- `kernel-exp`, `drivers-dev`, `fs-dev` — experimental, no force-push protection
- `master` — deprecated mirror, will be removed after v16.0

## Code of Conduct

All participants must follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
Violations: contact maintainers privately.

## Release Process

1. Feature freeze on `main`
2. Release branch cut: `release/vX.Y.Z`
3. RC testing period (minimum 1 week)
4. Signed ISO artefact via `scripts/sign_release.sh`
5. GitHub release tag + changelog
6. Wiki updated + announcement

---

*See also: [CONTRIBUTING.md](CONTRIBUTING.md) · [CONTRIBUTOR_ROADMAP.md](CONTRIBUTOR_ROADMAP.md)*
