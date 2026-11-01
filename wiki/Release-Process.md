# SigmaOS Release Process

> This document defines how SigmaOS releases are created, signed, published, and announced. Every release — alpha, beta, RC, and stable — follows this process.

---

## Table of Contents

1. [Version Scheme](#version-scheme)
2. [Release Channels](#release-channels)
3. [Release Checklist Overview](#release-checklist-overview)
4. [Phase 1: Release Branch](#phase-1-release-branch)
5. [Phase 2: RC Process](#phase-2-rc-process)
6. [Phase 3: Signing](#phase-3-signing)
7. [Phase 4: ISO Publishing](#phase-4-iso-publishing)
8. [Phase 5: Changelog & Release Notes](#phase-5-changelog--release-notes)
9. [Phase 6: Announcement](#phase-6-announcement)
10. [Hotfix Releases](#hotfix-releases)
11. [Support Lifecycle](#support-lifecycle)

---

## Version Scheme

SigmaOS uses **CalVer** (Calendar Versioning) for stable releases:

```
YYYY.MM[-suffix]

Examples:
  2027.06          — Stable release, June 2027
  2027.06-lts      — LTS designation
  2027.06-rc1      — Release Candidate 1
  2027.06-rc2      — Release Candidate 2
  2027.06.1        — Hotfix release
  0.3-alpha        — Pre-stable alpha (phases 1-5)
  0.6-beta         — Pre-stable beta (phases 6-8)
```

### Component Library Versioning

Internal component libraries (sigma-common, sigma-types, etc.) use **SemVer**:

```
MAJOR.MINOR.PATCH

MAJOR — breaking change to public API
MINOR — backward-compatible new features
PATCH — backward-compatible bug fixes
```

### Pre-stable Version Numbers

Until the first stable release:
- `v0.N-alpha` for Phase 1-5 releases
- `v0.N-beta` for Phase 6-8 releases
- `v1.0-rcN` for RC builds

---

## Release Channels

| Channel | Audience | Update Frequency | Stability |
|---------|----------|-----------------|-----------|
| `stable` | All users | LTS: every ~12 months | Highest |
| `security` | All users (auto) | As needed (72h SLA) | Critical fixes only |
| `beta` | Advanced users | Every 6-8 weeks | Beta quality |
| `alpha` | Developers | Every 2-4 weeks | Pre-feature-complete |
| `nightly` | Developers | Daily | Unstable |

Users select their channel in sigma-settings. Default is `stable`.

---

## Release Checklist Overview

```
Feature freeze
     │
     ▼
Create release/vX.Y branch
     │
     ▼
Fix blockers only (no new features)
     │
     ▼
Publish RC1 ISO
     │
     ▼
14-day community testing window
     │
     ▼
Fix P1 bugs → RC2
     │
     ▼
7-day community testing window
     │
     ▼
Zero P1 bugs? → Sign + Publish Stable ISO
     │
     ▼
Announce → Activate update channel
```

---

## Phase 1: Release Branch

### 1.1 Feature Freeze

Declared by a Core Maintainer 6 weeks before target release date. After feature freeze:
- No new features merge to `main`
- Only bug fixes, security fixes, and documentation updates merge

### 1.2 Create Release Branch

```bash
# Tag feature freeze point
git tag -s "v${VERSION}-freeze" -m "Feature freeze for ${VERSION}"
git push origin "v${VERSION}-freeze"

# Create release branch
git checkout -b "release/v${VERSION}"
git push -u origin "release/v${VERSION}"
```

### 1.3 Version Bump

Update version in:
- `sigma-common/src/version.rs` — primary version declaration
- All component `Cargo.toml` files (automated via `sigma-release` script)

```bash
./scripts/sigma-release bump --version "${VERSION}"
```

### 1.4 Build Release ISO

```bash
./scripts/sigma-release build --channel stable --version "${VERSION}"
```

This triggers:
1. Clean build of all 149 components from `release/v${VERSION}` branch
2. Package assembly into rootfs
3. ISO creation via `sigma-iso`
4. SBOM generation
5. Artifact upload to staging storage

---

## Phase 2: RC Process

### RC1

**Goal:** First public test ISO. Find issues before GA.

1. Build RC1 ISO from `release/v${VERSION}` branch
2. Run automated test suite (QEMU-based, 20+ hardware profiles)
3. Publish RC1 to the `beta` channel and announce to community
4. **14-day community testing window**
5. All bugs reported during window triaged into:
   - **P1 (blocker):** Must fix before stable. Blocks release.
   - **P2 (major):** Should fix. Release delayed if too many.
   - **P3 (minor):** Nice to fix. Documented in known issues.

### RC2

**Goal:** Fix all P1 blockers from RC1. Final candidate.

1. Merge all P1 fixes to `release/v${VERSION}`
2. Build RC2 ISO
3. Re-run automated test suite
4. Publish RC2 with **7-day community testing window**
5. If new P1 bugs found: RC3 (repeat). If clean: proceed to stable.

### RC → Stable Criteria

Release goes stable when:
- Zero open P1 bugs
- Zero regressions vs previous stable release
- Security audit completed (no critical findings)
- All automated tests passing
- Release notes reviewed and approved

---

## Phase 3: Signing

All SigmaOS release artifacts are cryptographically signed.

### Keys

| Key | Used For | Algorithm |
|-----|----------|-----------|
| Project Release Key | ISO, packages | Ed25519 GPG |
| Package Signing Key | Individual packages | Ed25519 GPG |
| Sigstore/Rekor | Transparency log entry | ECDSA (via cosign) |

### Key Storage

- Release keys stored in hardware security modules (HSMs) — never on disk
- Signing happens in isolated CI environment with audit log
- Key rotation every 2 years; old keys retained for verification of old releases

### ISO Signing Process

```bash
# Sign ISO with GPG
gpg --batch --yes --armor --detach-sign \
  --local-user "SigmaOS Release Key <release@sigmaos.dev>" \
  "sigmaos-${VERSION}-x86_64.iso"

# Create SHA256 checksum file
sha256sum "sigmaos-${VERSION}-x86_64.iso" > "sigmaos-${VERSION}-x86_64.sha256"

# Sign the checksum file too
gpg --batch --yes --armor --detach-sign \
  --local-user "SigmaOS Release Key <release@sigmaos.dev>" \
  "sigmaos-${VERSION}-x86_64.sha256"

# Publish to Sigstore Rekor transparency log
cosign sign-blob \
  --key "hsm:sigma-release-key" \
  "sigmaos-${VERSION}-x86_64.iso"
```

### Verifying a SigmaOS ISO (User Instructions)

```bash
# Download ISO, checksum, and signature
wget https://dl.sigmaos.dev/2027.06/sigmaos-2027.06-x86_64.iso
wget https://dl.sigmaos.dev/2027.06/sigmaos-2027.06-x86_64.sha256
wget https://dl.sigmaos.dev/2027.06/sigmaos-2027.06-x86_64.sha256.asc

# Import SigmaOS public key
gpg --recv-keys <FINGERPRINT>

# Verify GPG signature on checksum file
gpg --verify sigmaos-2027.06-x86_64.sha256.asc sigmaos-2027.06-x86_64.sha256

# Verify checksum matches ISO
sha256sum -c sigmaos-2027.06-x86_64.sha256
```

---

## Phase 4: ISO Publishing

### Mirror Network

ISOs are published to a mirror network of 3+ servers. Mirror servers are community-operated and vetted by maintainers.

**Mirror regions:** EU, North America, Asia-Pacific

```
Primary:    https://dl.sigmaos.dev/
Mirror EU:  https://eu.mirror.sigmaos.dev/
Mirror US:  https://us.mirror.sigmaos.dev/
Mirror AP:  https://ap.mirror.sigmaos.dev/
```

### Publishing Process

```bash
./scripts/sigma-release publish --version "${VERSION}" --type stable

# This script:
# 1. Uploads ISO + signatures to CDN origin
# 2. Syncs to all mirror servers
# 3. Publishes torrent file
# 4. Updates download page metadata
# 5. Creates GitHub Release with artifacts
# 6. Publishes update channel metadata (triggers auto-update for existing users)
```

### GitHub Release

The GitHub Release includes:
- ISO file(s) (x86_64, ARM64 if applicable)
- SHA256 checksum file
- GPG signature file
- SBOM (CycloneDX JSON)
- Release notes (summary only — full notes linked)

---

## Phase 5: Changelog & Release Notes

### Automated Changelog

The changelog is generated from conventional commits using `sigma-changelog`:

```bash
./scripts/sigma-release generate-changelog \
  --from "v${PREVIOUS_VERSION}" \
  --to "v${VERSION}" \
  --output "CHANGELOG.md"
```

This produces a structured changelog grouped by type (Features, Bug Fixes, Security, etc.).

### Release Notes Format

Release notes are published on the SigmaOS website and GitHub. Format:

```markdown
# SigmaOS 2027.06 Release Notes

## Highlights

[3-5 bullet points about the most important changes]

## What's New

### Desktop
- ...

### System
- ...

### Security
- ...

### Hardware Support
- ...

## Bug Fixes

[Top 10 bug fixes — link to full changelog for complete list]

## Known Issues

[Any P3 issues documented here]

## Upgrade Instructions

[How to upgrade from previous stable]

## Download

[Link to download page]

## Checksums

[SHA256 checksums]
```

---

## Phase 6: Announcement

### Announcement Channels

1. **GitHub Release** — Always first. Auto-notifies watchers.
2. **Project website** — Updated with release notes and download links.
3. **GitHub Discussions** — Announcement post in "Releases" category.
4. **Mastodon/Fediverse** — Short announcement with download link.
5. **Reddit** (r/linux, r/SigmaOS) — Release announcement post.
6. **DistroWatch** — Submit release information.

### Announcement Timing

- GitHub Release: immediately after ISO is published to all mirrors
- Website: simultaneously
- Social media: within 1 hour of GitHub Release
- DistroWatch: within 24 hours

### Announcement Template

```
SigmaOS {VERSION} is now available!

Highlights:
- {highlight 1}
- {highlight 2}
- {highlight 3}

Download: https://sigmaos.dev/download
Release notes: https://sigmaos.dev/releases/{VERSION}
GPG signature: https://sigmaos.dev/keys
```

---

## Hotfix Releases

Hotfixes address critical bugs or security issues in stable releases without a full RC cycle.

### Hotfix Criteria

A hotfix is warranted when:
- A CVE affects all stable users with a CVSS score >= 7.0
- A bug causes data loss or data corruption
- A bug makes the system unbootable for a significant percentage of users

### Hotfix Process

```bash
# Branch from the affected release tag
git checkout -b "fix/critical-issue" "v${VERSION}"

# Apply fix
# ... commit changes ...

# Build hotfix ISO (if kernel/boot affected) or just packages
./scripts/sigma-release hotfix --base "${VERSION}" --suffix ".1"

# Sign and publish to security channel immediately
./scripts/sigma-release publish --version "${VERSION}.1" --type hotfix --channel security

# Announce on all channels with security advisory
```

### Security Hotfix Timeline

- CVE reported → acknowledged: < 24 hours
- CVE fix developed: < 48 hours
- Hotfix published: < 72 hours
- Public disclosure: after patch is available (coordinated disclosure)

---

## Support Lifecycle

| Release Type | Support Duration | Security Updates |
|-------------|-----------------|-----------------|
| Stable LTS | 2 years | Yes, throughout |
| Stable non-LTS | 9 months | Yes, throughout |
| Beta | No guarantee | Best-effort |
| Alpha | No guarantee | None |

### End of Life Process

1. EOL announced 3 months before end date
2. EOL warning shown in sigma-settings and sigma-update
3. Updates cease at EOL date
4. Users are prompted to upgrade to current stable

### LTS Policy

SigmaOS declares an LTS release once every ~24 months. LTS releases:
- Receive 2 years of security updates
- Receive bug fix backports for critical issues
- Do NOT receive feature backports
- Are the only releases suitable for production/enterprise use
