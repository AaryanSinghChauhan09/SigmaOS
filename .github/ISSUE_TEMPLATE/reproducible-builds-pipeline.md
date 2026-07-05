---
name: Reproducible Builds Pipeline
about: Implement CI job that produces byte-for-byte reproducible images and records build provenance
title: "[Phase 0] Implement Reproducible Builds Pipeline"
labels: "Phase 0, security, ci, medium-priority"
assignees: ""
---

## Issue Description

Implement a reproducible builds pipeline in CI that produces byte-for-byte identical images across different runners and records build provenance metadata.

## Background

Reproducible builds are critical for supply chain security and trust. This aligns with Phase 0 goals of stabilizing core & trust, and enables deterministic releases that can be cryptographically verified.

## Scope

### Primary Tasks

1. **CI Job Creation**
   - Create GitHub Actions workflow for reproducible builds
   - Configure build environment with pinned toolchain versions
   - Implement build-time source timestamp normalization
   - Add deterministic build flags to CMake/Cargo builds

2. **Build Provenance Recording**
   - Capture and record build environment metadata (OS, compiler versions, toolchain hashes)
   - Generate build manifest with SHA256 hashes of all inputs
   - Store provenance metadata alongside build artifacts
   - Implement signing of build manifests

3. **Verification**
   - Add verification step that compares builds across multiple runners
   - Implement diff tool to identify non-deterministic build outputs
   - Create reproducibility report generation

### Files to Modify/Create

- `.github/workflows/reproducible-build.yml` - New CI workflow
- `build/reproducible-build.sh` - Build script with deterministic flags
- `build/provenance.json` - Template for build metadata
- `CMakeLists.txt` - Add reproducible build flags
- `Cargo.toml` - Add deterministic build configuration

## Success Criteria

- [ ] CI workflow produces identical binaries across at least 2 different runners
- [ ] Build provenance metadata is automatically generated and stored
- [ ] Build manifests are signed with project keys
- [ ] Verification job fails if builds are not reproducible
- [ ] Documentation updated with reproducible build process

## Estimated Effort

**Difficulty**: Medium  
**Time**: 1–3 weeks

## Dependencies

- None (can be implemented in parallel with other Phase 0 tasks)

## Related Issues

- Phase 0: Stabilize core & trust
- ROADMAP_NEW.md Phase 0 deliverables

## Implementation Notes

Key considerations:
- Use `SOURCE_DATE_EPOCH` for deterministic timestamps
- Pin all dependency versions in lockfiles
- Remove build-time randomization (ASLR can be handled separately)
- Consider using `reprotest` or similar tools for verification

## Resources

- [Reproducible Builds Project](https://reproducible-builds.org/)
- [GitHub Actions for Reproducible Builds](https://docs.github.com/en/actions/guides/about-continuous-integration)
