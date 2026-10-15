# Source Package Build & Ports Tree Component Guide

## Component Overview
Governs automated source compilation, sandbox build environments, reproducible package recipe compilation (PKGBUILD parity), and dependency tree generation.

## Linux Inspiration
Arch Linux PKGBUILD / makepkg, Gentoo Portage ebuild system, Alpine Linux APKBUILD.

## BSD Inspiration
FreeBSD Ports collection and poudriere clean-room builder, OpenBSD ports tree and dpb (distributed ports builder).

## Current SigmaOS Status
Recipe parser, tar/zstd source extraction, and build step execution pipeline implemented. Chroot/namespace build sandboxing needs completion.

## Critical Missing Features
Hermetic namespace and Landlock-confined sandbox build roots, parallel distributed compilation cluster hooks, and automated SBOM generation.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/package/recipe.rs, src/package/builder.rs, src/package/sandbox_build.rs

## Testing Strategy
Reproducible build bit-for-bit checksum verification, sandbox escape denial tests during compilation.

## Dependencies
PACKAGE_MANAGEMENT_AGENT, CONTAINER_RUNTIME_AGENT, SECURITY_AGENT

## Success Criteria
100% reproducible bit-for-bit builds, sub-second package packaging from source trees, zero build-time host contamination.

## Open Source Competitors Analysis
Combines the declarative simplicity of Arch PKGBUILD with the strict clean-room isolation of FreeBSD poudriere.

## Future Enhancements
Distributed peer-to-peer distributed compilation mesh utilizing idle cluster CPU cores.
