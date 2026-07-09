# SigmaOS Driver Reproducibility Guide

## Overview

This guide explains how to implement reproducible builds for SigmaOS drivers, ensuring deterministic compilation, trust, and debugging capabilities. Reproducible builds are essential for security, auditability, and community trust.

## Philosophy

- **Deterministic Builds**: Same source + same environment = identical binary
- **Bit-for-Bit Verification**: Verify builds match expected outputs
- **SBOM Generation**: Generate Software Bill of Materials for all drivers
- **Build Farm**: Use controlled build environments
- **Transparency**: Make build process auditable and verifiable

## Reproducible Build Requirements

### Build Environment

1. **Fixed Toolchain**:
   - GCC/Clang version pinned
   - Binutils version pinned
   - Kernel headers version pinned
   - Build dependencies version pinned

2. **Controlled Environment**:
   - Fixed base image (Docker/OCI)
   - Deterministic filesystem layout
   - Fixed timestamps
   - Controlled locale settings

3. **Build Isolation**:
   - Containerized builds
   - Network isolation during build
   - Deterministic random seeds
   - Fixed build order

### Source Code

1. **Deterministic Sources**:
   - Git commit hash pinned
   - Source tarball checksums verified
   - Patches applied in deterministic order
   - No embedded build timestamps

2. **Build Configuration**:
   - Fixed compiler flags
   - Deterministic optimization level
   - No debug symbols in release builds
   - Strip deterministic metadata

## Build Farm Architecture

### Build Farm Components

```
┌─────────────────┐
│  Build Queue    │
│  (RabbitMQ)     │
└────────┬────────┘
         │
         ├─────────────────────────────────┐
         │                                 │
┌────────▼────────┐              ┌────────▼────────┐
│  Build Worker 1 │              │  Build Worker 2 │
│  (Container)    │              │  (Container)    │
└────────┬────────┘              └────────┬────────┘
         │                                 │
         └─────────────────────────────────┘
                           │
                  ┌────────▼────────┐
                  │  Artifact Store │
                  │  (S3/Nexus)    │
                  └────────┬────────┘
                           │
                  ┌────────▼────────┐
                  │  SBOM Database  │
                  │  (PostgreSQL)   │
                  └─────────────────┘
```

### Build Worker Configuration

**Dockerfile for Build Environment**:
```dockerfile
FROM sigmaos/build-base:6.1.0

# Pin toolchain versions
ENV GCC_VERSION=12.2.0
ENV BINUTILS_VERSION=2.40
ENV KERNEL_HEADERS=6.1.0

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gcc-${GCC_VERSION} \
    binutils-${BINUTILS_VERSION} \
    linux-headers-${KERNEL_HEADERS} \
    make \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set deterministic environment
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV SOURCE_DATE_EPOCH=1704067200
```

## Reproducible Build Process

### Step 1: Source Preparation

```bash
#!/bin/bash
# prepare-source.sh

DRIVER_NAME=$1
VERSION=$2

# Clone repository at specific commit
git clone https://github.com/sigmaos/${DRIVER_NAME}.git
cd ${DRIVER_NAME}
git checkout ${COMMIT_HASH}

# Verify checksum
sha256sum ${DRIVER_NAME}-${VERSION}.tar.gz > checksums.sig
gpg --verify checksums.sig

# Apply patches in deterministic order
for patch in $(ls patches/*.patch | sort); do
    patch -p1 < $patch
done
```

### Step 2: Deterministic Build

```bash
#!/bin/bash
# reproducible-build.sh

# Set deterministic environment
export SOURCE_DATE_EPOCH=1704067200
export TZ=UTC
export LANG=C
export LC_ALL=C

# Build with deterministic flags
make \
    CC=gcc-12.2.0 \
    CFLAGS="-O2 -fno-strict-aliasing -fno-common -fno-delete-null-pointer-checks -fno-stack-protector-strong" \
    LDFLAGS="-Wl,--no-as-needed -Wl,--build-id=sha1" \
    KBUILD_BUILD_USER="sigmaos" \
    KBUILD_BUILD_HOST="build-farm" \
    KBUILD_BUILD_TIMESTAMP="1704067200"

# Strip deterministic metadata
strip --strip-unneeded --remove-section=.comment \
    --remove-section=.note driver.ko
```

### Step 3: Verification

```bash
#!/bin/bash
# verify-build.sh

# Calculate checksum
sha256sum driver.ko > driver.ko.sha256

# Compare with expected checksum
if [ "$(cat driver.ko.sha256)" != "$(cat expected.sha256)" ]; then
    echo "Build is not reproducible!"
    exit 1
fi

# Verify with diffoscope
diffoscope driver.ko reference.ko
```

## SBOM Generation

### SBOM Format

SigmaOS uses SPDX format for SBOMs:

```json
{
  "SPDXID": "SPDXRef-DOCUMENT",
  "spdxVersion": "SPDX-2.3",
  "name": "nvidia-driver-535.154.05",
  "documentNamespace": "https://sigmaos.org/sbom/nvidia-driver-535.154.05",
  "creationInfo": {
    "created": "2024-01-15T10:30:00Z",
    "creators": ["Tool: sigma-sbom-generator-1.0"]
  },
  "packages": [
    {
      "SPDXID": "SPDXRef-Package-nvidia-driver",
      "name": "nvidia-driver",
      "versionInfo": "535.154.05",
      "downloadLocation": "https://github.com/NVIDIA/nvidia/archive/535.154.05.tar.gz",
      "filesAnalyzed": false,
      "licenseConcluded": "NOASSERTION",
      "externalRefs": [
        {
          "referenceCategory": "PACKAGE-MANAGER",
          "referenceLocator": "pkg:sigmaos/nvidia-driver@535.154.05",
          "referenceType": "purl"
        }
      ]
    }
  ],
  "relationships": [
    {
      "spdxElementId": "SPDXRef-Package-nvidia-driver",
      "relatedSpdxElement": "SPDXRef-Package-kernel-6.1.0",
      "relationshipType": "DEPENDS_ON"
    }
  ]
}
```

### SBOM Generation Script

```bash
#!/bin/bash
# generate-sbom.sh

sigma-sbom generate \
    --package nvidia-driver \
    --version 535.154.05 \
    --source-dir /tmp/nvidia-driver \
    --output sbom.json \
    --format spdx-json

# Sign SBOM
gpg --detach-sign --armor sbom.json
```

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Reproducible Driver Build

on:
  push:
    paths:
      - 'drivers/nvidia/**'
  pull_request:
    paths:
      - 'drivers/nvidia/**'

jobs:
  reproducible-build:
    runs-on: ubuntu-latest
    container: sigmaos/build-env:6.1.0
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
      
      - name: Build driver
        run: |
          ./scripts/reproducible-build.sh nvidia-driver 535.154.05
      
      - name: Verify reproducibility
        run: |
          ./scripts/verify-build.sh nvidia-driver.ko
      
      - name: Generate SBOM
        run: |
          ./scripts/generate-sbom.sh nvidia-driver
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: nvidia-driver
          path: |
            nvidia-driver.ko
            sbom.json
            sbom.json.sig
```

## Verification Infrastructure

### Build Verification Service

```go
// build-verifier.go
package main

import (
    "crypto/sha256"
    "encoding/hex"
    "io"
    "os"
)

type BuildVerifier struct {
    expectedChecksums map[string]string
}

func (bv *BuildVerifier) VerifyBuild(binaryPath string) (bool, error) {
    file, err := os.Open(binaryPath)
    if err != nil {
        return false, err
    }
    defer file.Close()
    
    hash := sha256.New()
    if _, err := io.Copy(hash, file); err != nil {
        return false, err
    }
    
    actualChecksum := hex.EncodeToString(hash.Sum(nil))
    expectedChecksum := bv.expectedChecksums[binaryPath]
    
    return actualChecksum == expectedChecksum, nil
}
```

### Diffoscope Integration

```bash
# Compare builds
diffoscope driver1.ko driver2.ko --html diff.html

# Detailed comparison
diffoscope --max-depth 10 driver1.ko driver2.ko
```

## Troubleshooting

### Non-Reproducible Builds

**Common Causes**:
1. Timestamps embedded in binary
   - Fix: Use `SOURCE_DATE_EPOCH`
2. Different compiler versions
   - Fix: Pin toolchain versions
3. Build order differences
   - Fix: Use deterministic build order
4. Filesystem differences
   - Fix: Use containerized builds

**Debugging Steps**:
```bash
# Enable build logging
make V=1 > build.log

# Compare with reference build
diffoscope driver.ko reference.ko

# Check for embedded timestamps
strings driver.ko | grep -i date
```

### SBOM Generation Issues

**Common Issues**:
1. Missing dependency information
   - Fix: Use dependency analysis tools
2. Incorrect license information
   - Fix: Manually verify licenses
3. Incomplete file list
   - Fix: Scan all source files

## Best Practices

### Development

1. **Version Pinning**: Pin all toolchain and dependency versions
2. **Deterministic Configuration**: Use fixed compiler flags and options
3. **Build Isolation**: Build in isolated containers
4. **Verification**: Always verify builds against references

### CI/CD

1. **Automated Verification**: Verify reproducibility in CI
2. **SBOM Generation**: Generate SBOMs for all builds
3. **Artifact Storage**: Store build artifacts with metadata
4. **Monitoring**: Monitor build reproducibility metrics

### Security

1. **Supply Chain Security**: Verify all dependencies
2. **Signature Verification**: Sign all build artifacts
3. **Audit Trail**: Maintain complete build logs
4. **Transparency**: Make build process public

## References

- [Reproducible Builds Project](https://reproducible-builds.org/)
- [SPDX Specification](https://spdx.github.io/spdx-spec/)
- [Diffoscope](https://diffoscope.org/)
- [NixOS Reproducible Builds](https://nixos.org/manual/nix/stable/)
