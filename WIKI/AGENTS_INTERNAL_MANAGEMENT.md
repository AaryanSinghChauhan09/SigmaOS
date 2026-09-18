# AI Agent Internal Management & Operations Architecture (`docs/AGENTS_INTERNAL_MANAGEMENT.md`)

This guide describes operational management workflows, release management cadence, and subsystem maintainer delegation for AI agents managing internal SigmaOS operations.

---

## 1. Internal Management Subsystem Mapping

AI internal management agents interact with the following core subsystems in `src/`:

1. **Future Development Protocol (`src/governance/future_protocol.rs`):**
   - Manage Special Interest Groups (`SigmaSpecialInterestGroup`) for Kernel, Drivers, Desktop, Security, and Apps.
   - Enforce RFC development workflows (`RfcDevelopmentWorkflow`) requiring double maintainer code review.
   - Maintain community contributor recognition and Hall of Fame tracking (`CommunityContributorRecognition`).

2. **Subsystem Maintainer Governance (`docs/MAINTAINERS.md`):**
   - Kernel & Memory Management: `src/kernel/`, `src/klib/`
   - Driver Framework & Hardware: `src/drivers/`
   - Security & PQC Cryptography: `src/security/`, `src/auth/`
   - Package Management & Reproducibility: `src/package/`, `src/sigpkg/`
   - Distro Parity & Desktop: `src/distro/`, `src/desktop/`

3. **Reproducible Build Verification (`tools/sigma_repro_build.sh`):**
   - Enforce environment sanitization (`LC_ALL=C`, `TZ=UTC`, `umask 0022`).
   - Pin `SOURCE_DATE_EPOCH` to git commit timestamp.
   - Capture SHA-256 manifest hashes in `.buildinfo` for bit-for-bit build reproducibility audits.

---

## 2. Release Management & Cadence

AI agents coordinate release channels (`Stable`, `LTS`, `Rolling`, `Edge`) using `OmarchyReleaseChannelSnapshotEngine` in `src/distro/omarchy_inspiration.rs`:
- **Rolling Releases:** Daily automated CI verification.
- **LTS Releases:** Bi-annual reproducible build audits with post-quantum signed manifests.
- **Pre-flight Rollback Snapshots:** Create snapshot generation prior to any system-wide upgrade.
