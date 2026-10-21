#!/bin/bash
# SigmaOS post-install configuration
# Inspired by Omarchy 4.0.5 install/post-install/all.sh

set -euo pipefail

# Resolve install root.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMAOS_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Apply /etc configuration.
if [ -d "${SIGMAOS_ROOT}/etc" ]; then
    cp -a "${SIGMAOS_ROOT}/etc/"* /etc/ 2>/dev/null || true
    echo "[sigmaos-post-install] Applied /etc configuration"
fi

# Enable system services.
systemctl enable sddm.service 2>/dev/null || true
systemctl enable wireplumber.service 2>/dev/null || true
systemctl enable sigmaos-shell.service 2>/dev/null || true
systemctl enable plymouth-start.service 2>/dev/null || true
systemctl enable zram-generator.service 2>/dev/null || true

# Rebuild initramfs.
if command -v dracut >/dev/null 2>&1; then
    dracut -f 2>/dev/null || true
elif command -v mkinitcpio >/dev/null 2>&1; then
    mkinitcpio -P 2>/dev/null || true
fi

echo "[sigmaos-post-install] Post-install configuration complete"
