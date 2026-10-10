#!/usr/bin/env bash
# SigmaOS Build Manifest Generator
set -euo pipefail

MANIFEST_DIR="reports"
MANIFEST_FILE="${MANIFEST_DIR}/build_manifest.json"

mkdir -p "${MANIFEST_DIR}"

GIT_COMMIT=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
BUILD_TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
RUSTC_VERSION=$(rustc --version 2>/dev/null || echo "unknown")
CARGO_VERSION=$(cargo --version 2>/dev/null || echo "unknown")

cat <<EOF > "${MANIFEST_FILE}"
{
  "project": "SigmaOS",
  "build_timestamp": "${BUILD_TIMESTAMP}",
  "git_commit": "${GIT_COMMIT}",
  "git_branch": "${GIT_BRANCH}",
  "toolchain": {
    "rustc": "${RUSTC_VERSION}",
    "cargo": "${CARGO_VERSION}"
  },
  "verification_commands": [
    "cargo check --lib",
    "cargo test --lib",
    "./run_sigma_tests.sh",
    "make check",
    "make test",
    "make format"
  ]
}
EOF

echo "Build manifest generated at ${MANIFEST_FILE}"
cat "${MANIFEST_FILE}"
