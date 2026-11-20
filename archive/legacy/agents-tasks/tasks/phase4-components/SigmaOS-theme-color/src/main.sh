#!/usr/bin/env bash
# SigmaOS Theme Color — accent color and palette manager scaffold
# Upstream: GNOME accent colors (https://gitlab.gnome.org/GNOME/gnome-shell),
#           KDE color schemes
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COLOR_CONFIG="${SIGMA_COLOR_CONFIG:-$HOME/.config/sigmaos/theme-color.toml}"

BUILTIN_ACCENTS=(
    "sigmaos-blue:#3584e4"
    "sigmaos-purple:#9141ac"
    "sigmaos-green:#26a269"
    "sigmaos-teal:#2190a4"
    "sigmaos-orange:#e66100"
    "sigmaos-red:#c01c28"
    "sigmaos-pink:#f66151"
    "sigmaos-yellow:#e5a50a"
    "sigmaos-slate:#5e5c64"
)

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS accent color and palette manager

Subcommands:
  list                    List available accent colors
  active                  Show current accent color
  set     <name|#hex>     Set the accent color
  preview <name|#hex>     Preview a color (print swatch)
  palette [<name>]        Show full palette for a scheme
  export  [--format css|gtk|env]   Export color variables
  reset                   Reset to SigmaOS default accent
  status                  Show color config state
  version                 Print manager version

EOF
}

hex_to_rgb() {
    local hex="${1#\#}"
    printf "%d %d %d" "0x${hex:0:2}" "0x${hex:2:2}" "0x${hex:4:2}"
}

print_swatch() {
    local name="$1" hex="$2"
    # ANSI 24-bit color block if terminal supports it
    local r g b
    read -r r g b <<< "$(hex_to_rgb "$hex")"
    printf "  \e[48;2;%d;%d;%dm    \e[0m  %s (%s)\n" "$r" "$g" "$b" "$name" "$hex"
}

cmd_list() {
    echo "Available accent colors:"
    for entry in "${BUILTIN_ACCENTS[@]}"; do
        local name="${entry%%:*}" hex="${entry#*:}"
        print_swatch "$name" "$hex"
    done
}

get_active() {
    if [ -f "$COLOR_CONFIG" ]; then
        grep -m1 "^accent" "$COLOR_CONFIG" 2>/dev/null | grep -o '"[^"]*"' | tr -d '"' || true
    fi
}

cmd_active() {
    local active
    active=$(get_active)
    if [ -z "$active" ]; then
        echo "Accent: sigmaos-blue (#3584e4)  [default]"
    else
        echo "Accent: $active"
    fi
}

resolve_color() {
    local query="$1"
    for entry in "${BUILTIN_ACCENTS[@]}"; do
        local name="${entry%%:*}" hex="${entry#*:}"
        if [ "$name" = "$query" ] || [ "$hex" = "$query" ]; then
            echo "$name $hex"; return
        fi
    done
    # Accept raw hex
    if [[ "$query" =~ ^#[0-9a-fA-F]{6}$ ]]; then
        echo "custom $query"; return
    fi
    echo ""
}

cmd_set() {
    local query="${1:?'color name or hex required'}"
    local resolved
    resolved=$(resolve_color "$query")
    if [ -z "$resolved" ]; then
        echo "[color] Unknown color: $query" >&2
        echo "[color] Use a name from 'list' or a hex value like #3584e4" >&2
        exit 1
    fi
    local name="${resolved%% *}" hex="${resolved#* }"
    mkdir -p "$(dirname "$COLOR_CONFIG")"
    cat > "$COLOR_CONFIG" <<EOF
# SigmaOS theme color config
accent = "$name"
accent_hex = "$hex"
EOF
    echo "[color] Accent set to: $name ($hex)"
    print_swatch "$name" "$hex"
    echo "[color] Restart desktop session to apply: sigmaos-desktop-session --restart"
}

cmd_preview() {
    local query="${1:?'color name or hex required'}"
    local resolved
    resolved=$(resolve_color "$query")
    if [ -z "$resolved" ]; then
        echo "[color] Unknown color: $query" >&2; exit 1
    fi
    local name="${resolved%% *}" hex="${resolved#* }"
    echo "Preview:"
    print_swatch "$name" "$hex"
}

cmd_palette() {
    local scheme="${1:-default}"
    cat <<EOF
SigmaOS $scheme palette:
  Background    : #1e1e2e  (dark) / #ffffff  (light)
  Surface       : #313244  (dark) / #f0f0f0  (light)
  Overlay       : #45475a  (dark) / #e0e0e0  (light)
  Text          : #cdd6f4  (dark) / #1c1c2e  (light)
  Subtext       : #a6adc8  (dark) / #5c5c7e  (light)
  Accent        : #3584e4  (sigmaos-blue, default)
  Error         : #c01c28
  Warning       : #e5a50a
  Success       : #26a269
EOF
}

cmd_export() {
    local fmt="${1:---format}"; shift || true
    local fmtname="${1:-css}"
    case "$fmtname" in
        css)
            cat <<'EOF'
:root {
  --sigma-bg:      #1e1e2e;
  --sigma-surface: #313244;
  --sigma-text:    #cdd6f4;
  --sigma-accent:  #3584e4;
  --sigma-error:   #c01c28;
  --sigma-warn:    #e5a50a;
  --sigma-ok:      #26a269;
}
EOF
            ;;
        gtk)
            cat <<'EOF'
@define-color accent_bg_color #3584e4;
@define-color accent_fg_color #ffffff;
@define-color accent_color @accent_bg_color;
@define-color error_color #c01c28;
@define-color warning_color #e5a50a;
@define-color success_color #26a269;
EOF
            ;;
        env)
            cat <<'EOF'
export SIGMA_COLOR_ACCENT=#3584e4
export SIGMA_COLOR_BG=#1e1e2e
export SIGMA_COLOR_TEXT=#cdd6f4
export SIGMA_COLOR_ERROR=#c01c28
EOF
            ;;
        *) echo "[color] Unknown format: $fmtname  (valid: css gtk env)" >&2; exit 1 ;;
    esac
}

cmd_reset() {
    rm -f "$COLOR_CONFIG"
    echo "[color] Reset to default: sigmaos-blue (#3584e4)"
}

cmd_status() {
    echo "Config  : $COLOR_CONFIG ($([ -f "$COLOR_CONFIG" ] && echo exists || echo default))"
    cmd_active
}

case "${1:-}" in
    list)     cmd_list ;;
    active)   cmd_active ;;
    set)      cmd_set "${2:-}" ;;
    preview)  cmd_preview "${2:-}" ;;
    palette)  cmd_palette "${2:-default}" ;;
    export)   cmd_export "${2:---format}" "${3:-css}" ;;
    reset)    cmd_reset ;;
    status)   cmd_status ;;
    version)  echo "SigmaOS theme-color manager v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
