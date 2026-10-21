#!/bin/bash
# SigmaOS post-install configuration
# Inspired by Omarchy 4.0.5 install/post-install/all.sh
# Sources shared helpers (DRY) instead of duplicating logic.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMAOS_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# shellcheck source=/dev/null
. "${SCRIPT_DIR}/../helpers/logging.sh"

# Apply /etc configuration.
if [ -d "${SIGMAOS_ROOT}/etc" ]; then
  cp -a "${SIGMAOS_ROOT}/etc/"* /etc/ 2>/dev/null || true
  log_info "Applied /etc configuration"
fi

# Copy default desktop configuration.
if [ -d "${SIGMAOS_ROOT}/default" ]; then
  mkdir -p /usr/share/sigmaos/default
  cp -a "${SIGMAOS_ROOT}/default/"* /usr/share/sigmaos/default/ 2>/dev/null || true
  log_info "Applied default desktop configuration"
fi

# Copy shell configuration.
if [ -d "${SIGMAOS_ROOT}/shell" ]; then
  mkdir -p /usr/share/sigmaos/shell
  cp -a "${SIGMAOS_ROOT}/shell/"* /usr/share/sigmaos/shell/ 2>/dev/null || true
  log_info "Applied shell configuration"
fi

# Copy runtime binaries.
if [ -d "${SIGMAOS_ROOT}/bin" ]; then
  mkdir -p /usr/bin
  cp -a "${SIGMAOS_ROOT}/bin/"* /usr/bin/ 2>/dev/null || true
  log_info "Applied runtime binaries"
fi

# Copy install helpers.
if [ -d "${SIGMAOS_ROOT}/install/helpers" ]; then
  mkdir -p /usr/share/sigmaos/install/helpers
  cp -a "${SIGMAOS_ROOT}/install/helpers/"* /usr/share/sigmaos/install/helpers/ 2>/dev/null || true
  log_info "Applied install helpers"
fi

# Enable system services via shared helper (single source of truth).
enable_sigmaos_services

# Rebuild initramfs.
if command -v dracut >/dev/null 2>&1; then
  dracut -f 2>/dev/null || true
elif command -v mkinitcpio >/dev/null 2>&1; then
  mkinitcpio -P 2>/dev/null || true
fi

log_success "SigmaOS post-install configuration complete"
