# sigpkg Manifest and Signing Specification

**Version:** 1.0
**Status:** Draft
**Last Updated:** 2025-01-22
**Purpose:** Define the SigmaOS native package format (sigpkg) manifest schema and signing requirements

---

## 1. Overview

sigpkg is the native package format for SigmaOS. It is designed for:
- **Atomic updates** - Packages install or fail as a unit
- **Signed verification** - All packages are cryptographically signed
- **Reproducible builds** - Build process is deterministic
- **Sandbox awareness** - Packages declare required capabilities
- **Rollback support** - Package installation is reversible

---

## 2. Package Structure

A sigpkg package is a compressed archive containing:

```
sigma-package-1.0.0.sigpkg
├── SIGPKG_MANIFEST.json   # Package metadata
├── SIGPKG_SIGNATURE.bin    # Ed25519 signature
├── SIGPKG_CONTENTS.sha256 # Content hashes
├── content/                # Package files
│   ├── bin/
│   ├── lib/
│   ├── share/
│   └── etc/
└── SIGPKG_BUILD_INFO.json  # Build provenance
```

---

## 3. Manifest Schema (SIGPKG_MANIFEST.json)

```json
{
  "$schema": "https://sigmaos.org/schemas/sigpkg-manifest-v1.json",
  "sigpkg_version": "1.0",
  "package": {
    "name": "sigma-coreutils",
    "version": "1.0.0",
    "epoch": 0,
    "release": "1",
    "architecture": "x86_64",
    "platform": "linux",
    "license": "MIT",
    "summary": "SigmaOS core utilities",
    "description": "Essential command-line utilities for SigmaOS",
    "url": "https://github.com/AaryanSinghChauhan09/SigmaOS",
    "maintainer": {
      "name": "SigmaOS Team",
      "email": "team@sigmaos.org"
    }
  },
  "dependencies": [
    {
      "name": "sigma-libc",
      "version_constraint": ">=1.0.0",
      "optional": false
    }
  ],
  "provides": [
    "sigma-ls",
    "sigma-cp",
    "sigma-mv"
  ],
  "conflicts": [
    {
      "name": "gnu-coreutils",
      "version_constraint": "*"
    }
  ],
  "replaces": [
    "legacy-utils"
  ],
  "capabilities": [
    {
      "name": "network",
      "required": false,
      "description": "Network access for remote operations"
    },
    {
      "name": "filesystem",
      "required": true,
      "description": "Filesystem access for file operations"
    }
  ],
  "contents": {
    "files": [
      {
        "path": "/bin/ls",
        "mode": "0755",
        "owner": "root",
        "group": "root",
        "hash": "sha256:abc123...",
        "size": 12345
      }
    ],
    "directories": [
      {
        "path": "/var/lib/sigma-coreutils",
        "mode": "0755",
        "owner": "root",
        "group": "root"
      }
    ],
    "config_files": [
      {
        "path": "/etc/sigma-coreutils.conf",
        "mode": "0644",
        "owner": "root",
        "group": "root",
        "preserve_on_update": true
      }
    ]
  },
  "scripts": {
    "pre_install": "/usr/share/sigma-coreutils/scripts/pre-install.sh",
    "post_install": "/usr/share/sigma-coreutils/scripts/post-install.sh",
    "pre_remove": "/usr/share/sigma-coreutils/scripts/pre-remove.sh",
    "post_remove": "/usr/share/sigma-coreutils/scripts/post-remove.sh"
  },
  "build": {
    "build_date": "2025-01-22T00:00:00Z",
    "build_host": "sigmaos-build-01",
    "builder": "sigma-build-system v1.0.0",
    "reproducible": true,
    "source_hash": "sha256:def456...",
    "build_hash": "sha256:ghi789..."
  },
  "checksums": {
    "manifest": "sha256:jkl012...",
    "signature": "sha256:mno345...",
    "contents": "sha256:pqr678..."
  }
}
```

---

## 4. Field Definitions

### 4.1 Package Metadata

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Package name (lowercase, alphanumeric, hyphens) |
| `version` | string | Yes | Semantic version (MAJOR.MINOR.PATCH) |
| `epoch` | integer | Yes | Version epoch (default 0) |
| `release` | string | Yes | Release number (default 1) |
| `architecture` | string | Yes | Target architecture (x86_64, aarch64, riscv64) |
| `platform` | string | Yes | Target platform (linux, bsd) |
| `license` | string | Yes | SPDX license identifier |
| `summary` | string | Yes | Short description (max 80 chars) |
| `description` | string | Yes | Long description |
| `url` | string | Yes | Project URL |
| `maintainer` | object | Yes | Maintainer information |

### 4.2 Dependencies

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Dependency package name |
| `version_constraint` | string | Yes | SemVer constraint (>=1.0.0, =2.0.0, etc.) |
| `optional` | boolean | Yes | Whether dependency is optional |

### 4.3 Capabilities

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Capability name (network, filesystem, graphics, etc.) |
| `required` | boolean | Yes | Whether capability is required |
| `description` | string | Yes | Human-readable description |

### 4.4 Contents

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | string | Yes | File system path |
| `mode` | string | Yes | Unix file mode (octal) |
| `owner` | string | Yes | File owner (username or UID) |
| `group` | string | Yes | File group (groupname or GID) |
| `hash` | string | Yes | SHA-256 hash (format: sha256:...) |
| `size` | integer | Yes | File size in bytes |

---

## 5. Signing Specification

### 5.1 Signature Algorithm

SigmaOS uses **Ed25519** for package signing:

- **Algorithm:** Ed25519 (Twisted Edwards Curve)
- **Key Size:** 256-bit private key, 256-bit public key
- **Signature Size:** 64 bytes
- **Hash Function:** SHA-512 (internal to Ed25519)

### 5.2 Signature Process

1. **Generate Signing Key Pair:**
   ```bash
   sigma-keygen --output keys/
   # Generates: private.key, public.key
   ```

2. **Sign Package:**
   ```bash
   sigma-sign --private-key keys/private.key \
              --manifest SIGPKG_MANIFEST.json \
              --output SIGPKG_SIGNATURE.bin
   ```

3. **Verify Signature:**
   ```bash
   sigma-verify --public-key keys/public.key \
                --manifest SIGPKG_MANIFEST.json \
                --signature SIGPKG_SIGNATURE.bin
   ```

### 5.3 Signature Format

The signature file (SIGPKG_SIGNATURE.bin) contains:

```
[signature: 64 bytes Ed25519 signature]
[public_key: 32 bytes Ed25519 public key]
[timestamp: 8 bytes Unix timestamp]
[key_id: 32 bytes key identifier]
```

### 5.4 Key Management

#### Key Hierarchy

1. **Root Key:** Master signing key (offline, air-gapped)
2. **Release Keys:** Per-release signing keys (signed by root)
3. **Builder Keys:** Per-builder keys (signed by release)

#### Key Rotation

- Root key rotation requires ADR approval
- Release keys rotate quarterly
- Builder keys rotate monthly
- Old keys remain in trust store for 1 year

#### Key Distribution

- Public keys distributed via repository metadata
- Key revocation via repository update
- Key expiration enforced

---

## 6. Repository Metadata

### 6.1 Repository Structure

```
repository/
├── SIGPKG_REPO_METADATA.json
├── keys/
│   ├── root.pub
│   ├── release-2025-01.pub
│   └── builder-01.pub
├── packages/
│   ├── sigma-coreutils/
│   │   ├── sigma-coreutils-1.0.0.sigpkg
│   │   └── sigma-coreutils-1.0.0.sigpkg.meta
│   └── ...
└── metadata/
    ├── packages-index.json
    └── repository-signature.bin
```

### 6.2 Repository Metadata Schema

```json
{
  "$schema": "https://sigmaos.org/schemas/sigpkg-repo-v1.json",
  "sigpkg_repo_version": "1.0",
  "repository": {
    "name": "SigmaOS Official Repository",
    "url": "https://packages.sigmaos.org",
    "description": "Official SigmaOS package repository",
    "mirrors": [
      "https://mirror1.sigmaos.org",
      "https://mirror2.sigmaos.org"
    ]
  },
  "keys": {
    "root_key_id": "sha256:abc123...",
    "release_keys": [
      {
        "key_id": "sha256:def456...",
        "public_key": "BASE64_ENCODED_PUBLIC_KEY",
        "expires": "2025-04-01T00:00:00Z"
      }
    ]
  },
  "signature": {
    "algorithm": "Ed25519",
    "key_id": "sha256:ghi789...",
    "signature": "BASE64_ENCODED_SIGNATURE",
    "timestamp": "2025-01-22T00:00:00Z"
  }
}
```

---

## 7. Security Requirements

### 7.1 Verification Requirements

1. **All packages must be signed**
2. **Signatures must be verified before installation**
3. **Repository metadata must be signed**
4. **Key revocation must be enforced**
5. **Expired keys must be rejected**

### 7.2 Rollback Protection

1. **Transaction journaling:** All package operations logged
2. **Automatic rollback:** Failed transactions revert automatically
3. **Snapshot before install:** System snapshot before package installation
4. **Rollback verification:** Rollback integrity verified

### 7.3 Reproducible Builds

1. **Build environment documented**
2. **Source hashes recorded**
3. **Build hashes verified**
4. **Timestamps normalized**
5. **Deterministic compilation flags**

---

## 8. Package Lifecycle

### 8.1 Installation Process

```
1. Download package
2. Verify repository signature
3. Verify package signature
4. Verify content hashes
5. Check dependencies
6. Check conflicts
7. Create system snapshot
8. Execute pre-install script
9. Extract contents
10. Set permissions
11. Execute post-install script
12. Update package database
13. On failure: rollback to snapshot
```

### 8.2 Removal Process

```
1. Check reverse dependencies
2. Create system snapshot
3. Execute pre-remove script
4. Remove files
5. Remove directories
6. Execute post-remove script
7. Update package database
8. On failure: rollback to snapshot
```

### 8.3 Update Process

```
1. Download new package
2. Verify signatures
3. Check compatibility
4. Create system snapshot
5. Install new package
6. Verify installation
7. Remove old package
8. On failure: rollback to snapshot
```

---

## 9. Compatibility Layers

### 9.1 Foreign Package Translation

SigmaOS can translate foreign package formats (DEB, RPM, APK, etc.) to sigpkg:

1. **Parse foreign package metadata**
2. **Extract contents**
3. **Generate sigpkg manifest**
4. **Compute content hashes**
5. **Sign translated package**
6. **Mark as "translated" in manifest**

### 9.2 Translation Metadata

```json
{
  "translation": {
    "original_format": "deb",
    "original_name": "coreutils",
    "original_version": "9.4-1",
    "translation_date": "2025-01-22T00:00:00Z",
    "translation_tool": "sigma-deb2sigpkg v1.0.0",
    "trust_level": "verified"
  }
}
```

---

## 10. Error Handling

### 10.1 Signature Verification Errors

| Error | Action |
|-------|--------|
| Invalid signature | Reject package, log security event |
| Expired key | Reject package, check for key update |
| Revoked key | Reject package, log security event |
| Unknown key | Reject package, require key installation |

### 10.2 Content Hash Errors

| Error | Action |
|-------|--------|
| Hash mismatch | Reject package, log corruption event |
| Missing hash | Reject package, require regeneration |
| Weak hash | Reject package, require SHA-256 |

---

## 11. Future Extensions

### 11.1 Post-Quantum Cryptography

Future versions may support:
- **Dilithium-5** for package signing
- **Kyber-1024** for key exchange
- Hybrid signature schemes (Ed25519 + Dilithium)

### 11.2 Transparency Logs

Future versions may include:
- **Rekor-style** transparency log integration
- **Certificate transparency** for keys
- **Build transparency** for reproducibility

---

## 12. Implementation Status

| Component | Status | Implementation |
|-----------|--------|----------------|
| Manifest schema | `Specification only` | docs/SIGPKG_MANIFEST_SPECIFICATION.md |
| Signature verification | `Prototype` | src/security/ stubs |
| Package manager | `Partially implemented` | src/package/ |
| Repository metadata | `Not started` | None |
| Rollback mechanism | `Not started` | None |

---

## 13. References

- [Ed25519 RFC 8032](https://rfc-editor.org/rfc/rfc8032)
- [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html)
- [SPDX License List](https://spdx.org/licenses/)
- [PROJECT_STATUS.md](PROJECT_STATUS.md)
- [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)
