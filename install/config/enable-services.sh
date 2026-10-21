#!/bin/bash
# SigmaOS enable-services (install/config/enable-services.sh)
# Enables the core system services needed for a working desktop.

set -euo pipefail

systemctl enable sddm.service 2>/dev/null || true
systemctl enable wireplumber.service 2>/dev/null || true
systemctl enable plymouth-start.service 2>/dev/null || true
systemctl enable sigmaos-shell.service 2>/dev/null || true
systemctl enable sigmaos-desktop-session.service 2>/dev/null || true
systemctl enable sigmaos-provision-owner.service 2>/dev/null || true
systemctl enable theme-system.service 2>/dev/null || true
systemctl enable zram-generator.service 2>/dev/null || true

echo "[sigmaos-enable-services] Core services enabled"
