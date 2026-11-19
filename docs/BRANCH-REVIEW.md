# Branch Review Summary

## Branch Status (2026-11-19)

### Current Main Branch
- **Commit**: 1760f95433 - "docs: Add GitHub Wiki content for key components"
- **Status**: Clean, focused Arch-based system
- **Changes**:
  - Archived 198 placeholder packages
  - Removed Omarchy references
  - Added real Hyprland, Waybar, Wallust configurations
  - Added GitHub Actions for ISO build
  - Created wiki documentation
  - Clarified README architecture

### Remote Branches

#### origin/docs/wiki-content
- **Latest commit**: eea4e98b22 - "docs: add comprehensive GitHub Wiki content"
- **Status**: CONFLICTS with focused architecture
- **Issues**:
  - Removes GitHub Actions workflows (build-iso.yml, test-packages.yml)
  - Restores Omarchy configs (config/omarchy/)
  - Restores 198 placeholder packages from pkgs_archive
  - Reverts README.md to multi-distro description
  - Restores old COMPONENTS.toml (32,581 lines)
  - Restores old bin/ scripts with Omarchy references
- **Recommendation**: DO NOT MERGE - this branch undoes the focused Arch-based work

#### origin/ci/test-pipeline
- **Latest commit**: 6211645f4a - "test: add CI/CD pipeline test marker"
- **Status**: Needs review
- **Recommendation**: Review before merging - may contain multi-distro complexity

#### origin/test/ci-cd-pipeline
- **Latest commit**: 46993fc606 - "fix: improve test suite memory usage and add incremental testing"
- **Status**: Needs review
- **Recommendation**: Review before merging - may contain multi-distro complexity

## Recommendation

Following **IF IT AIN'T BROKE, DON'T FIX IT** and **GALL'S LAW**:

1. **Delete origin/docs/wiki-content** - It conflicts with the focused architecture
2. **Review ci/test-pipeline and test/ci-cd-pipeline** - Check if they align with Arch-based focus
3. **Keep current main** - It represents the working, focused system

## Component Registry Status

COMPONENTS.toml currently declares 2,495 components, but:
- 1,235 declared URLs are not present on GitHub
- 850 imported account repositories remain Unclassified
- 45 names have repeated declarations with conflicting metadata

Following **CONSOLIDATE GITHUB REPOS OF SIMILAR COMPONENTS** and **OPTIMISE FOR DELETION**:
- The registry needs significant cleanup
- Many declared components are placeholder/duplicate
- Should audit and consolidate to necessary components only

## Next Steps

1. Delete conflicting branch: `git push origin --delete docs/wiki-content`
2. Review other branches for compatibility
3. Audit COMPONENTS.toml for consolidation
4. Continue with focused Arch-based ISO build and testing
