# Roadmap (readiness)

This roadmap prioritizes **fundamentals** before futuristic “sovereignty” layers. Dates are not commitments; use GitHub milestones when the maintainers adopt them.

## Phase 0 — Honesty and hygiene (now)

- [x] Modular Zenith source (`js/zenith/`)

- [x] Wiki source in-repo with optional **auto-sync** (`docs/wiki/` + `wiki-sync.yml`)

- [x] Style tooling: `.clang-format`, PR scoped format check

- [x] Doxygen scaffold + CI artifact
- [ ] Green CI on `main` for build + quality jobs (fix breakages as they appear)

## Phase 1 — Codebase quality

- Incremental **modularization** of the largest kernel translation units (extract concerns; avoid mega-files).
- Consistent **error handling** policy in touched code paths (document in Developer Guide).
- **Unit tests** for pure logic (host-compiled) where feasible; **integration** tests under QEMU later.

## Phase 2 — Core OS depth

- Memory management and scheduling: document actual vs. intended behavior; add benchmarks.
- File system path: VFS completeness, persistence stories, corruption handling.
- **Profiling** hooks and documented workflows (sampling, traces).

## Phase 3 — Security baseline

- **Secure boot / verified boot** design doc + minimal implementation path.
- **Sandbox** boundaries for userland when a stable syscall surface exists.
- **Update pipeline** with signing and rollback.

## Phase 4 — Networking (incremental)

- IPv4/IPv6 feature matrix documented; implement parity in small steps.
- VPN/mesh: prototype isolated from default kernel until stable.
- **Post-quantum** TLS: follow standards (e.g. hybrid KEMs) when libraries and interop exist.

## Phase 5 — User and developer experience

- **Package manager** UX and policy.
- **GUI** beyond Zenith demo: real compositor/session story.
- **Accessibility** pass on Zenith + future shell.
- **SDK** and codegen for third-party devs.

## Phase 6 — Alpha release

- Installation guide frozen for a tagged version.
- Known limitations enumerated publicly.
- Security contact and release signing keys published.

## Related

- [Architecture](Architecture) — diagram and directory roles.
- [Feature Backlog (100)](Feature-Backlog-100) — icebox, milestones, define-done.
- [Competitive Gaps](Competitive-Gaps) — honest comparison vs incumbents.
- [Future Improvements](Future-Improvements) — CI, security process, docs meta-work ([repo file](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/REPO_FUTURE_IMPROVEMENTS.md)).
- [Security Posture](Security-Posture) — what is real vs. aspirational.
- `docs/COMPETITIVE_GAPS.md` — same as Competitive Gaps (repo canonical).

