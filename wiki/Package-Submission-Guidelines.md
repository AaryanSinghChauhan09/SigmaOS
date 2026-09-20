# Package Submission Guidelines: Contributing Packages to SigmaOS

## Introduction

This guide outlines the rules and procedure for maintainers, developers, and community contributors wishing to submit new packages or update existing software recipes in the SigmaOS package ecosystem.

## Submission Workflow Overview

```
 1. Write Recipe (Sigbuildfile)
    -> 2. Test Build in Isolated Sandbox (`sigbuild`)
    -> 3. Lint & Security Hardening Audit (`siglint`)
    -> 4. Cryptographic Signing
    -> 5. Create Pull Request to Package Repository
```

## Step 1: Create a Compliant Recipe (`Sigbuildfile`)

Every package submission requires a valid `Sigbuildfile`. Ensure your recipe adheres to the following standards:

1. **Naming Conventions**:
   - Package names must be lowercase ASCII strings containing only letters, numbers, hyphens, and underscores.
   - Avoid generic names (e.g. `terminal` -> use `zenith-terminal`).
2. **Upstream Source Verification**:
   - Always use HTTPS source URLs pointing directly to official release tarballs or tag archives.
   - Provide explicit cryptographic checksums (`sha256sums` or `b2sums`).
3. **Dependencies**:
   - List minimal runtime dependencies in `depends=()`.
   - List explicit build tools in `makedepends=()`.

## Step 2: Local Testing in Clean Sandbox

Before submitting a package, verify that it builds in a clean chroot environment without relying on unstated host dependencies:

```bash
# Build package in isolated chroot
sigbuild --clean

# Verify resulting .sigpkg archive creation
ls -la *.sigpkg
```

## Step 3: Package Quality & Security Linting

Run `siglint` on your built package to detect common packaging errors:

```bash
siglint zenith-terminal-1.2.0-1-x86_64.sigpkg
```

`siglint` checks for:
- [x] ELF binary security hardening flags (`PIE`, `RELRO`, `STACK-PROTECTOR`, `NX`).
- [x] Absence of hardcoded temporary paths (e.g., `/tmp` or `/home/...` references).
- [x] Correct installation paths (`/usr/bin/`, `/usr/share/`, no stray files in `/usr/local/`).
- [x] Presence of valid license files in `/usr/share/licenses/${pkgname}/`.

## Step 4: Submit Pull Request

1. Fork the official SigmaOS package repository (`sigmaos/packages`).
2. Create a feature branch named `pkg/add-pkgname` or `pkg/update-pkgname`.
3. Place your `Sigbuildfile` in `pkgs/community/pkgname/Sigbuildfile`.
4. Submit a Pull Request with a clear commit description detailing upstream release notes and verification results.

## Review & Promotion Process

1. **Automated CI**: GitHub Actions build farms execute `sigbuild` in a sandboxed container to verify buildability and security compliance.
2. **Maintainer Audit**: A core maintainer reviews code security, licensing, and dependency impacts.
3. **Merging & Repository Indexing**: Once approved, the package is signed with the automated release key and published to the `community` repository channel.
