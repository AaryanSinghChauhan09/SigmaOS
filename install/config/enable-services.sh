#!/bin/bash
# SigmaOS enable-services (install/config/enable-services.sh)
# Enables the core system services needed for a working desktop.
# Modeled on Omarchy 4.0.5 install/config/enable-services.sh

set -euo pipefail

log_info() { echo "[sigmaos-enable-services] [INFO]  $*" >&2; }
log_warn() { echo "[sigmaos-enable-services] [WARN]  $*" >&2; }

log_info "Enabling SigmaOS core services..."

# Display manager
systemctl enable sddm.service 2>/dev/null || log_warn "sddm not available"

# Audio
systemctl enable wireplumber.service 2>/dev/null || log_warn "wireplumber not available"

# Boot splash
systemctl enable plymouth-start.service 2>/dev/null || log_warn "plymouth not available"

# Power management
systemctl enable zram-generator.service 2>/dev/null || log_warn "zram-generator not available"

# SigmaOS desktop session (system)
systemctl enable sigmaos-desktop-session.service 2>/dev/null || log_warn "sigmaos-desktop-session not available"

# SigmaOS shell (user - enabled per-user via loginctl)
systemctl enable sigmaos-shell.service 2>/dev/null || log_warn "sigmaos-shell not available"

# SigmaOS user daemons (enabled per-user)
systemctl enable sigmaos-clipboard-daemon.service 2>/dev/null || log_warn "sigmaos-clipboard-daemon not available"
systemctl enable sigmaos-reminderd.service 2>/dev/null || log_warn "sigmaos-reminderd not available"
systemctl enable sigmaos-power-manager.service 2>/dev/null || log_warn "sigmaos-power-manager not available"

# First-boot provisioning
systemctl enable sigmaos-provision-owner.service 2>/dev/null || log_warn "sigmaos-provision-owner not available"

# Theme system
systemctl enable theme-system.service 2>/dev/null || log_warn "theme-system not available"

# Apply sysctl
sysctl --system 2>/dev/null || log_warn "sysctl --system failed"

# Apply tmpfiles
systemd-tmpfiles --create 2>/dev/null || log_warn "systemd-tmpfiles failed"

log_info "SigmaOS core services enabled successfully"
