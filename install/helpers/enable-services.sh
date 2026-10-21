# SigmaOS shared service enabling
# Single source of truth for enabling all SigmaOS system services.
# Sourced by enable-services.sh, post-install/all.sh, and the ISO installer.
#
# Design: Single responsibility, loose coupling — callers source this file
# and invoke enable_sigmaos_services(). No duplication of service lists.

set -euo pipefail

# Source logging helpers if available.
SIGMAOS_HELPERS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -r "${SIGMAOS_HELPERS_DIR}/logging.sh" ]; then
  # shellcheck source=/dev/null
  . "${SIGMAOS_HELPERS_DIR}/logging.sh"
fi

# Enable all SigmaOS core system services.
# Callers should run this as root in the target root.
enable_sigmaos_services() {
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

  log_success "SigmaOS core services enabled"
}
