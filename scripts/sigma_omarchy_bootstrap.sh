#!/usr/bin/env bash
# Stage SigmaOS desktop defaults without overwriting existing user configs.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_HOME="${XDG_CONFIG_HOME:-${HOME:?HOME must be set}/.config}"
MODE="dry-run"

usage() {
    cat <<'EOF'
Usage: sigma_omarchy_bootstrap.sh [--dry-run | --apply]

By default, show which bundled desktop defaults would be staged. Pass --apply
to copy defaults only when the destination does not already exist. Existing
files are never overwritten. This script does not install packages or fonts.
EOF
}

case "${1:-}" in
    "") ;;
    --dry-run) MODE="dry-run" ;;
    --apply) MODE="apply" ;;
    --help|-h) usage; exit 0 ;;
    *) usage >&2; exit 2 ;;
esac

case "$CONFIG_HOME" in
    /*) ;;
    *) printf 'XDG_CONFIG_HOME must be an absolute path: %s\n' "$CONFIG_HOME" >&2; exit 2 ;;
esac

installed=0
unchanged=0
conflicts=0

stage_config() {
    local source="$1"
    local destination="$2"

    if [[ ! -f "$source" ]]; then
        printf 'Missing bundled default: %s\n' "$source" >&2
        return 1
    fi

    if [[ -e "$destination" || -L "$destination" ]]; then
        if [[ -f "$destination" ]] && cmp -s -- "$source" "$destination"; then
            printf 'Already current: %s\n' "$destination"
            ((unchanged += 1))
        else
            printf 'Preserving existing config: %s\n' "$destination"
            ((conflicts += 1))
        fi
        return 0
    fi

    if [[ "$MODE" == "dry-run" ]]; then
        printf 'Would stage: %s -> %s\n' "$source" "$destination"
        return 0
    fi

    mkdir -p -- "$(dirname -- "$destination")"
    if cp -n -- "$source" "$destination"; then
        if [[ -f "$destination" ]] && cmp -s -- "$source" "$destination"; then
            printf 'Staged: %s\n' "$destination"
            ((installed += 1))
        else
            printf 'Preserving concurrently-created config: %s\n' "$destination"
            ((conflicts += 1))
        fi
    else
        printf 'Could not stage: %s\n' "$destination" >&2
        return 1
    fi
}

printf 'SigmaOS desktop defaults (%s)\n' "$MODE"
printf 'Config directory: %s\n' "$CONFIG_HOME"

stage_config "$ROOT_DIR/default/hypr/hyprland.conf" "$CONFIG_HOME/hypr/hyprland.conf"
stage_config "$ROOT_DIR/default/waybar/config" "$CONFIG_HOME/waybar/config"
stage_config "$ROOT_DIR/default/waybar/style.css" "$CONFIG_HOME/waybar/style.css"
stage_config "$ROOT_DIR/default/ghostty/config" "$CONFIG_HOME/ghostty/config"

if command -v fc-list >/dev/null 2>&1; then
    if fc-list 2>/dev/null | grep -iq 'JetBrainsMono Nerd Font'; then
        printf 'Font available: JetBrainsMono Nerd Font\n'
    else
        printf 'Font not detected: JetBrainsMono Nerd Font (not installed by this script)\n'
    fi
else
    printf 'Font check skipped: fc-list is unavailable\n'
fi

if [[ "$MODE" == "dry-run" ]]; then
    printf 'Dry run only; pass --apply to stage missing files.\n'
elif (( conflicts > 0 )); then
    printf 'Applied with %d existing config conflict(s); no existing files were overwritten.\n' "$conflicts"
else
    printf 'Staged %d config file(s); %d already matched.\n' "$installed" "$unchanged"
fi
