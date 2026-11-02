# Governance: SigmaOS-release

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-release owns the release engineering pipeline for SigmaOS: defining the release channels (testing and stable), building the installable ISO images, version tagging across all component repos, authoring and publishing release notes, and running the QA gates that gate a release from testing to stable. It is the top-level orchestrator that assembles component packages into a shippable release.
- **Runtime layer:** tooling (release engineering, runs in CI)
- **Scope boundary with adjacent repos:** SigmaOS-installer provides the installer embedded in the ISO; SigmaOS-package-manager provides the base system packages assembled into the ISO; SigmaOS-update consumes the release channel definitions this repo publishes; SigmaOS-docs publishes the release notes this repo authors.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-release` CLI | CLI | stable |
| Release channel definitions (`channels/*.toml`) | Config file | stable |
| ISO build manifest format | File format | unstable |
| Release notes template (`templates/release-notes.md`) | Template | stable |
| Version manifest (per-release component versions) | File format | stable |
| QA gate definitions (`qa/*.yaml`) | Config file | stable |
| Internal build orchestrator | internal | internal-only |

- **Breaking-change policy:** Channel definitions and version manifest format stable within a MAJOR version; QA gate schema additive.
- **API documentation location:** `docs/release-process.md`, `docs/channels.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-installer | 0.2 | embedded in ISO | No |
| SigmaOS-package-manager | 0.3 | base system assembly | No |
| SigmaOS-kernel | 6.6.0 | kernel image in ISO | No |
| SigmaOS-initramfs | 0.3 | initramfs in ISO | No |
| SigmaOS-boot | 0.2 | bootloader in ISO | No |
| archiso / xorriso | 72 / 1.5 | ISO build tooling | No |
| qemu | 8.0 | QA gate testing | No |
| dbus | 1.14 | release orchestration IPC | No |

- **Optional/soft dependencies:** `s3cmd`/`rclone` for mirror upload; `gpg` for ISO signing.
- **Upstream patch inventory:** Local archiso profile; no source patches.
- **Dependency update policy:** Build tooling reviewed quarterly; ISO rebuilt per release.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-release-team
- **Co-maintainers:** @core-lead, @installer-lead
- **Review quorum:** 2 maintainer approvals required; channel definition or QA gate changes require @core-lead approval.
- **Escalation path:** Release-blocking issues are `priority:critical`; a broken release affects all users.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 35% — channel definition parsing, version manifest parsing, and QA gate validation unit-tested.
- **Integration tests:** Present — full ISO build + QEMU boot smoke test runs in CI per release candidate.
- **CI pipeline file:** `.github/workflows/release-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `manifest-validation`, `iso-build-smoke`, `qa-gate-definition-test`, `lint`
- **Known flaky tests:** `iso-build-smoke` occasionally times out on slow CI runners (issue #4).
- **CI build badge:** `![release CI](https://github.com/SigmaOS/SigmaOS-release/actions/workflows/release-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 (UEFI) | supported |
| x86_64 (BIOS) | supported |
| aarch64 (UEFI) | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop, server, VM — ISOs produced for all; QA gates run on QEMU.
- **Backend variants:** Linux build host; output is arch-specific ISO.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** ISO reproducible across builds, QA gates passing for testing channel, stable channel promotion validated end-to-end, release notes auto-generated from changelogs, signed ISOs verified.

## 8. What This Repo Does NOT Do

- Does not build component packages → handled by each repo's build system
- Does not manage package repositories → handled by SigmaOS-repo-manager (this uploads to them)
- Does not perform system updates → handled by SigmaOS-update (this publishes the channels it reads)
- Does not own the installer logic → handled by SigmaOS-installer (this embeds it)
- Does not define versioning policy for components → each repo's GOVERNANCE.md defines that

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/release.toml`, `channels/testing.toml`, `channels/stable.toml`, `qa/*.yaml`
- **Schema file location:** `schemas/release.toml.schema.json`, `schemas/channel.schema.json`, `schemas/version-manifest.schema.json`
- **Schema format:** TOML (config + channels), YAML (QA gates), JSON (version manifest)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_RELEASE_CHANNEL` | `stable` | Active channel for `sigmaos-release` operations |
| `SIGMAOS_ISO_ARCH` | `x86_64` | Target ISO architecture |
| `SIGMAOS_ISO_SIGN` | `1` | Sign ISO with release key |
| `SIGMAOS_QA_STRICT` | `1` | Enforce all QA gates before promotion |

- **Breaking config change policy:** Channel and manifest schema additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| ISO build failure | Missing component package or tooling | No ISO produced | Fix component build; rerun release pipeline |
| QA gate failure | Component fails a gate | Release blocked in testing | Fix component; rerun gate; do not promote |
| Version manifest mismatch | Component tagged wrong version | Release assembles wrong versions | Re-tag component; rebuild manifest |
| ISO signature failure | Missing or wrong release key | ISO unsigned; users can't verify | Load release key; re-sign; republish |
| Mirror upload failure | Network or mirror issue | ISO not available to users | Retry upload; fall back to alternate mirror |
| Channel promotion race | Stable promoted while testing still failing | Bad release shipped | QA gate is atomic per channel; promotion refuses if any gate red |

- **Fallback behavior:** If a QA gate fails, the release stays in testing and is never promoted to stable; if ISO build fails, the previous signed ISO remains published until a new one succeeds.
- **Operator escalation path:** Collect release pipeline log and ISO build log; file issue with `priority:critical`.
- **Crash dump artifacts:** Build log in CI artifacts; version manifest at `releases/<version>/manifest.json`.

## 11. Update / Rollback Behavior

- **Update mechanism:** This repo updates itself like any package; release pipeline is re-run on merge.
- **Rollback mechanism:** Previous ISO retained per release; channel can be rolled back to a prior version manifest; component rollback handled by SigmaOS-package-manager/SigmaOS-update.
- **Expected downtime on update:** None (tooling); a release rollback may require users to reinstall/revert.
- **Pre-update state checks:** No active release promotion in progress.
- **Post-update validation:** Next release pipeline dry-run verifies tooling.
- **Atomic update guarantee:** Partial — channel promotion is atomic (all-or-nothing per channel); ISO publication is atomic (new ISO replaces old only after upload + signature verify).

## 12. Security Boundary

- **Privilege level required:** Release pipeline runs in CI with release-engineering secrets; ISO signing requires the release private key (HSM-backed where available).
- **Trust boundary:** Trusts component packages signed by their repos; signs ISOs with the release key; does not trust unsigned component packages.
- **Attack surface:** ISO build pipeline, signing key, mirror upload, channel definitions
- **Hardening measures:** Release key stored in HSM; ISO signing happens in isolated CI runner; channel promotion gated by QA; release notes reviewed before publication.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12; signing ceremony documented.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-installer | 0.2+ | tested (ISO embed) |
| SigmaOS-package-manager | 0.3+ | tested (base assembly) |
| SigmaOS-kernel | 6.6.x | tested (ISO embed) |
| SigmaOS-update | 0.3+ | tested (channel consumption) |

| Package | Version Range | Status |
|---------|---------------|--------|
| archiso | 72+ | tested |
| xorriso | 1.5+ | tested |
| qemu | 8.0+ | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** Channel and manifest schema stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/release-process.md`, `docs/channels.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/release-engineering.md`
- **Man page or in-app help status:** present — `sigmaos-release(8)`
- **Documentation parity policy:** Channel or QA gate schema changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:channels` | Release channel issues |
| `component:iso` | ISO build issues |
| `component:qa-gates` | QA gate definition and execution issues |
| `component:release-notes` | Release notes issues |
| `component:signing` | ISO signing issues |
| `release:testing` | Issues affecting the testing channel |
| `release:stable` | Issues affecting the stable channel |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-release/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`) for this repo's tooling; release versions use the SigmaOS-wide version (e.g., `v1.0`).
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags; release versions tagged as `release/<version>`.
- **Changelog format:** Conventional Commits; release notes auto-generated from component changelogs.
- **Breaking change signal:** MAJOR bump for channel or manifest schema incompatibility.
- **Version pinning in dependents:** SigmaOS-update pins channel definitions from this repo.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** peer (release orchestrator) — assembles SigmaOS-core (and all components) into a release; core is the central component whose version anchors a release.
- **IPC / integration mechanism:** Build pipeline consumes core's package and version manifest; release notes aggregate core's changelog; no runtime IPC (release engineering is build-time).
- **Core API version it targets:** `SigmaOS-core >= 0.4` (the minimum core version included in a release).
- **Integration tested in CI:** yes — ISO build + boot smoke test boots SigmaOS-core and runs a smoke check.
- **Change coordination policy:** Core version bumps require a coordinated release tag; core API changes that affect the ISO's base system require a release pipeline update.
- **Core team review required for cross-boundary changes:** yes
