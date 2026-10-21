#!/bin/bash
# SigmaOS first-boot provisioning
# Inspired by Omarchy 4.0.5 install/provisioning/setup-form.sh
#
# Runs on first boot after installation. Prompts the user for keyboard,
# account, hostname, timezone — then applies system configuration.
#
# Exit statuses:
#   0   field set, move on
#   1   Esc — unwind to start of form
#   130 Ctrl+C — caller handles (deferred provisioning or reboot)

set -euo pipefail

# Resolve the SigmaOS install root (the directory containing this script's
# default/ and etc/ trees).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMAOS_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
notice() {
    echo "[sigmaos-provision] $1" >&2
}

run_logged() {
    local script="$1"
    notice "running: ${script}"
    bash "${script}" || notice "WARNING: ${script} exited non-zero"
}

# ---------------------------------------------------------------------------
# Phase 1: Keyboard layout
# ---------------------------------------------------------------------------
setup_keyboard() {
    notice "Selecting keyboard layout..."
    local layout
    layout=$(localectl list-keymaps | gum choose --header "Keyboard layout" --height 20 --default us 2>/dev/null || echo "us")
    localectl set-keymap "${layout}"
    notice "Keyboard set to ${layout}"
}

# ---------------------------------------------------------------------------
# Phase 2: User account
# ---------------------------------------------------------------------------
setup_user() {
    notice "Creating user account..."
    local username password
    username=$(gum input --header "Username" --value "sigma" --placeholder "Enter username" 2>/dev/null || echo "sigma")
    password=$(gum input --header "Password" --password --placeholder "Enter password" 2>/dev/null || echo "")
    if id "${username}" &>/dev/null; then
        notice "User ${username} already exists"
    else
        useradd -m -G wheel -s /bin/bash "${username}"
        if [ -n "${password}" ]; then
            echo "${username}:${password}" | chpasswd
        fi
        notice "User ${username} created"
    fi
}

# ---------------------------------------------------------------------------
# Phase 3: Hostname
# ---------------------------------------------------------------------------
setup_hostname() {
    notice "Setting hostname..."
    local hostname
    hostname=$(gum input --header "Hostname" --value "sigmaos" --placeholder "Enter hostname" 2>/dev/null || echo "sigmaos")
    hostnamectl set-hostname "${hostname}"
    notice "Hostname set to ${hostname}"
}

# ---------------------------------------------------------------------------
# Phase 4: Timezone
# ---------------------------------------------------------------------------
setup_timezone() {
    notice "Selecting timezone..."
    local tz
    tz=$(timedatectl list-timezones | gum choose --header "Timezone" --height 20 --default UTC 2>/dev/null || echo "UTC")
    timedatectl set-timezone "${tz}"
    notice "Timezone set to ${tz}"
}

# ---------------------------------------------------------------------------
# Phase 5: Apply system configuration (the /etc and default/ trees)
# ---------------------------------------------------------------------------
apply_system_config() {
    notice "Applying SigmaOS system configuration..."

    # Copy /etc configs to the target root.
    if [ -d "${SIGMAOS_ROOT}/etc" ]; then
        cp -a "${SIGMAOS_ROOT}/etc/"* /etc/ 2>/dev/null || true
        notice "Applied /etc configuration"
    fi

    # Copy default desktop configs.
    if [ -d "${SIGMAOS_ROOT}/default" ]; then
        mkdir -p /usr/share/sigmaos/default
        cp -a "${SIGMAOS_ROOT}/default/"* /usr/share/sigmaos/default/ 2>/dev/null || true
        notice "Applied default desktop configuration"
    fi

    # Enable essential services.
    systemctl enable sddm.service 2>/dev/null || true
    systemctl enable wireplumber.service 2>/dev/null || true
    systemctl enable sigmaos-shell.service 2>/dev/null || true
    systemctl enable sigmaos-desktop-session.service 2>/dev/null || true
    systemctl enable plymouth-start.service 2>/dev/null || true
    systemctl enable zram-generator.service 2>/dev/null || true

    # Rebuild initramfs so Plymouth and the boot hooks are included.
    if command -v dracut >/dev/null 2>&1; then
        dracut -f 2>/dev/null || true
    elif command -v mkinitcpio >/dev/null 2>&1; then
        mkinitcpio -P 2>/dev/null || true
    fi

    notice "System configuration applied"
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
main() {
    notice "SigmaOS first-boot provisioning starting..."

    setup_keyboard
    setup_user
    setup_hostname
    setup_timezone

    apply_system_config

    notice "SigmaOS provisioning complete. Reboot to start the desktop session."
}

main "$@"
