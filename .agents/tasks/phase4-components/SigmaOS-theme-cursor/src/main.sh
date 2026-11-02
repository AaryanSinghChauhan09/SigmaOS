#!/usr/bin/env bash
# SigmaOS Theme Cursor — cursor theme manager scaffold
# Upstream: Bibata cursor (https://github.com/ful1e5/Bibata_Cursor),
#           Adwaita cursor (GNOME)
set -euo pipefail

CURSOR_THEMES_DIR="${SIGMA_CURSOR_DIR:-/usr/share/icons}"
USER_CURSOR_DIR="${HOME}/.local/share/icons"
CURSOR_CONFIG="${HOME}/.config/sigmaos/cursor.toml"

BUILTIN_CURSORS=(
    "Adwaita:24:system default (GTK4)"
    "SigmaOS-Cursors:24:SigmaOS custom cursor theme"
    "Bibata-Modern-Classic:24:Bibata modern style"
    "Breeze:24:KDE Breeze cursor"
    "DMZ-White:24:classic white cursor"
)

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS cursor theme manager

Subcommands:
  list                    List available cursor themes
  active                  Show current cursor theme
  set  <theme> [--size n] Set cursor theme and size
  preview <theme>         Show theme info
  install <pkg>           Install a cursor theme package
  remove  <theme>         Remove a cursor theme
  sizes                   List common cursor sizes
  export  [--format gtk|env|xrdb]  Export cursor settings
  reset                   Reset to SigmaOS default cursor
  status                  Show cursor config state
  version                 Print manager version

EOF
}

cmd_list() {
    echo "Available cursor themes:"
    for entry in "${BUILTIN_CURSORS[@]}"; do
        local name="${entry%%:*}"
        local rest="${entry#*:}"
        local size="${rest%%:*}"
        local desc="${rest#*:}"
        printf "  %-28s size %-4s %s\n" "$name" "$size" "$desc"
    done
    # Also scan installed
    for dir in "$CURSOR_THEMES_DIR"/*/; do
        local n
        n=$(basename "$dir")
        [ -d "$dir/cursors" ] || continue
        # Skip if already in builtin list
        local found=0
        for entry in "${BUILTIN_CURSORS[@]}"; do
            [[ "${entry%%:*}" == "$n" ]] && found=1 && break
        done
        [ "$found" -eq 0 ] && echo "  $n"
    done
}

cmd_active() {
    local theme="SigmaOS-Cursors" size=24
    if [ -f "$CURSOR_CONFIG" ]; then
        local t; t=$(grep -m1 "^theme" "$CURSOR_CONFIG" 2>/dev/null | grep -o '"[^"]*"' | tr -d '"' || true)
        local s; s=$(grep -m1 "^size" "$CURSOR_CONFIG" 2>/dev/null | grep -oP '\d+' || true)
        [ -n "$t" ] && theme="$t"
        [ -n "$s" ] && size="$s"
    fi
    echo "Cursor theme : $theme"
    echo "Cursor size  : $size"
}

cmd_set() {
    local theme="${1:?'theme name required'}"
    local size=24
    if [ "${2:-}" = "--size" ]; then size="${3:-24}"; fi
    mkdir -p "$(dirname "$CURSOR_CONFIG")"
    cat > "$CURSOR_CONFIG" <<EOF
# SigmaOS cursor config
theme = "$theme"
size  = $size
EOF
    # Also write to GTK settings
    local gtk3_dir="$HOME/.config/gtk-3.0"
    mkdir -p "$gtk3_dir"
    if grep -q "gtk-cursor-theme-name" "$gtk3_dir/settings.ini" 2>/dev/null; then
        sed -i "s/gtk-cursor-theme-name=.*/gtk-cursor-theme-name=$theme/" "$gtk3_dir/settings.ini"
        sed -i "s/gtk-cursor-theme-size=.*/gtk-cursor-theme-size=$size/" "$gtk3_dir/settings.ini"
    else
        cat >> "$gtk3_dir/settings.ini" <<EOF
[Settings]
gtk-cursor-theme-name=$theme
gtk-cursor-theme-size=$size
EOF
    fi
    echo "[cursor] Set: $theme (size $size)"
    echo "[cursor] GTK3 settings updated: $gtk3_dir/settings.ini"
    echo "[cursor] Restart desktop session to apply"
}

cmd_preview() {
    local theme="${1:?'theme name required'}"
    echo "Cursor theme preview: $theme"
    for entry in "${BUILTIN_CURSORS[@]}"; do
        if [[ "${entry%%:*}" == "$theme" ]]; then
            local rest="${entry#*:}"
            local size="${rest%%:*}" desc="${rest#*:}"
            echo "  Default size : $size"
            echo "  Description  : $desc"
            return
        fi
    done
    if [ -d "$CURSOR_THEMES_DIR/$theme/cursors" ]; then
        local count
        count=$(ls "$CURSOR_THEMES_DIR/$theme/cursors/" 2>/dev/null | wc -l)
        echo "  Cursor files : $count"
    else
        echo "  Theme not found in $CURSOR_THEMES_DIR"
    fi
}

cmd_install() {
    local pkg="${1:?'package name required'}"
    echo "[cursor] Installing cursor package: $pkg"
    echo "  sigmaos-pkg install $pkg"
    echo "  (scaffold: printed, not executed)"
}

cmd_remove() {
    local theme="${1:?'theme name required'}"
    if [ "$theme" = "Adwaita" ] || [ "$theme" = "SigmaOS-Cursors" ]; then
        echo "[cursor] Cannot remove system cursor: $theme" >&2; exit 1
    fi
    echo "[cursor] Removing cursor theme: $theme"
    rm -rf "$USER_CURSOR_DIR/$theme" 2>/dev/null || true
    echo "[cursor] Removed from user icons: $USER_CURSOR_DIR/$theme"
}

cmd_sizes() {
    cat <<'EOF'
Common cursor sizes:
  16   — small (HiDPI ×1 equivalent)
  24   — default (recommended)
  32   — medium-large
  48   — large (HiDPI ×2)
  64   — very large (accessibility)
  96   — extra-large
EOF
}

cmd_export() {
    local fmt="${2:-gtk}"
    case "$fmt" in
        gtk)
            echo "[Settings]"
            echo "gtk-cursor-theme-name=SigmaOS-Cursors"
            echo "gtk-cursor-theme-size=24"
            ;;
        env)
            echo "export XCURSOR_THEME=SigmaOS-Cursors"
            echo "export XCURSOR_SIZE=24"
            ;;
        xrdb)
            echo "Xcursor.theme: SigmaOS-Cursors"
            echo "Xcursor.size:  24"
            ;;
        *) echo "[cursor] Unknown format: $fmt  (valid: gtk env xrdb)" >&2; exit 1 ;;
    esac
}

cmd_reset() {
    rm -f "$CURSOR_CONFIG"
    echo "[cursor] Reset to default: SigmaOS-Cursors (size 24)"
}

cmd_status() {
    echo "Config     : $CURSOR_CONFIG ($([ -f "$CURSOR_CONFIG" ] && echo exists || echo default))"
    echo "System dir : $CURSOR_THEMES_DIR"
    echo "User dir   : $USER_CURSOR_DIR"
    cmd_active
}

case "${1:-}" in
    list)     cmd_list ;;
    active)   cmd_active ;;
    set)      cmd_set "${2:-}" "${3:-}" "${4:-}" ;;
    preview)  cmd_preview "${2:-}" ;;
    install)  cmd_install "${2:-}" ;;
    remove)   cmd_remove "${2:-}" ;;
    sizes)    cmd_sizes ;;
    export)   cmd_export "${2:-}" "${3:-gtk}" ;;
    reset)    cmd_reset ;;
    status)   cmd_status ;;
    version)  echo "SigmaOS cursor theme manager v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
