# SigmaOS Release Process

---

## Release Types

| Type | Example | When |
|------|---------|------|
| Patch | v15.0.1 | Security fix, critical bug |
| Minor | v15.1.0 | New features, all CI green |
| Major | v16.0.0 | Major milestone (e.g., bootable ISO) |
| LTS | v15.0.0-LTS | Long-term support designation |

---

## Release Checklist

### 1. Feature Freeze
```bash
# Announce feature freeze on GitHub Discussions
# Stop merging feature PRs to main
# Only bug fixes, security patches, and docs allowed
```

### 2. Release Branch
```bash
git checkout -b release/v15.1.0 main
git push origin release/v15.1.0
```

### 3. RC Testing (minimum 1 week)
- Run full CI matrix: x86_64, ARM64 cross-build
- QEMU smoke test
- Manual testing checklist from `PHASE_A_EXECUTION_CHECKLIST.md`
- Fuzz PQC modules: `make fuzz TARGET=pqc`
- Performance regression check: `scripts/regression_check.sh`

### 4. Changelog
```bash
scripts/gen_changelog.sh v15.0.0..HEAD > CHANGELOG_v15.1.0.md
# Edit for readability, add highlights section
```

### 5. Sign & Tag
```bash
# Sign the release commit
git tag -s v15.1.0 -m "SigmaOS v15.1.0 Zenith LTS"
git push origin v15.1.0

# Sign the ISO (Phase G+)
scripts/sign_release.sh build/sigmaos-v15.1.0.iso
```

### 6. Publish
```bash
# Create GitHub release
scripts/publish_release.ps1 -version v15.1.0 -notes CHANGELOG_v15.1.0.md

# Sync all branches to new tag
node tools/sync_all_branches.js

# Update wiki version references
# Update Home.md, Version-Timeline.md, Development-Roadmap.md
```

### 7. Announce
- GitHub Releases page
- Update `wiki_repo/Release-Notes.md`
- Update `wiki_repo/Home.md` version badge

---

## Version Numbering

`vMAJOR.MINOR.PATCH[-CODENAME]`

- **MAJOR**: architectural milestone (bootable OS, distributed, India Stack)
- **MINOR**: feature release (new subsystem, profile, significant capability)
- **PATCH**: security fix, critical bug fix
- **CODENAME**: human-friendly name (Zenith, Apex, Sovereign, Transcendence)

---

## Artefacts

| Artefact | Description |
|----------|-------------|
| `sigmaos-v15.1.0-x86_64.iso` | Bootable ISO (Phase G+) |
| `sigmaos-v15.1.0-arm64.img` | ARM64 image (Phase G+) |
| `sigmaos-v15.1.0-cloud.qcow2` | Cloud VM image (Phase G+) |
| `sigma-sdk-v15.1.0.tar.gz` | Developer SDK |
| `SHA256SUMS` | Checksums for all artefacts |
| `SHA256SUMS.dilithium5.sig` | Dilithium-5 signature |

---

*See also: [Version-Timeline](Version-Timeline) · [CHANGELOG](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CHANGELOG.md) · [Governance-Model](Governance-Model)*
