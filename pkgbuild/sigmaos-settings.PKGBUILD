# sigmaos-settings - SigmaOS system settings package
# Modeled on Omarchy 4.0.6 omarchy-settings package
pkgname=sigmaos-settings
pkgver=4.0.6
pkgrel=1
pkgdesc="SigmaOS system settings and configuration"
arch=('any')
url="https://github.com/AaryanSinghChauhan09/SigmaOS"
license=('MIT OR GPL-2.0')
depends=(
  'gum' 'fzf' 'git' 'curl' 'wget'
  'hyprland' 'waybar' 'wofi' 'foot' 'ghostty'
  'swww' 'hyprpaper'
  'pipewire' 'wireplumber' 'pamixer' 'pavucontrol'
  'brightnessctl' 'power-profiles-daemon'
  'networkmanager' 'bluez' 'bluez-utils'
)

package() {
  # Install SigmaOS settings configuration
  mkdir -p "${pkgdir}/usr/share/sigmaos/settings"
  
  # Default settings
  cat > "${pkgdir}/usr/share/sigmaos/settings/settings.conf" <<'EOF'
# SigmaOS System Settings
# Modeled on Omarchy 4.0.6 omarchy-settings

# Theme
theme=sigmaos
background=default
font=JetBrainsMono Nerd Font

# Desktop
terminal=foot
editor=micro
browser=chromium
file_manager=nautilus

# Audio
audio_backend=wireplumber
volume_step=5

# Display
bar_position=top
bar_theme=catppuccin

# Accessibility
screen_reader=off
high_contrast=off

# Power
power_profile=balanced
</EOF>
  
  # Install settings scripts
  mkdir -p "${pkgdir}/usr/bin"
  cat > "${pkgdir}/usr/bin/sigmaos-settings" <<'SCRIPT'
#!/bin/bash
# sigmaos-settings - SigmaOS system settings manager
# Modeled on Omarchy 4.0.5 omarchy-settings

set -euo pipefail

SETTINGS_FILE="${XDG_CONFIG_HOME:-$HOME/.config}/sigmaos/settings.conf"

show_settings() {
    echo "=== SigmaOS System Settings ==="
    if [ -f "$SETTINGS_FILE" ]; then
        cat "$SETTINGS_FILE"
    else
        echo "Using default settings"
    fi
}

set_setting() {
    local key="$1"
    local value="$2"
    if [ -z "$key" ] || [ -z "$value" ]; then
        echo "Usage: sigmaos-settings set <key> <value>" >&2
        exit 1
    fi
    mkdir -p "$(dirname "$SETTINGS_FILE")"
    if [ -f "$SETTINGS_FILE" ] && grep -q "^${key}=" "$SETTINGS_FILE"; then
        sed -i "s/^${key}=.*/${key}=${value}/" "$SETTINGS_FILE"
    else
        echo "${key}=${value}" >> "$SETTINGS_FILE"
    fi
    echo "Set ${key}=${value}"
}

case "${1:-}" in
    show)
        show_settings
        ;;
    set)
        set_setting "${2:-}" "${3:-}"
        ;;
    *)
        echo "Usage: sigmaos-settings <show|set <key> <value>>" >&2
        exit 1
        ;;
esac
SCRIPT
  chmod +x "${pkgdir}/usr/bin/sigmaos-settings"
  
  echo "[sigmaos-settings] Package installed successfully"
}