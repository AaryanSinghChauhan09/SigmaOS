#!/usr/bin/env bash
# SigmaOS mac-run — omarchy-mac-inspired macOS app compatibility shims
# Upstream inspiration: omarchy-mac (https://github.com/basecamp/omarchy-mac)
set -euo pipefail

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

Subcommands:
  run  <app>    Launch a macOS app via the compatibility shim
  shim list     List all registered app shims
  env  list     List all environment translation mappings

Options:
  -h, --help    Show this help

Examples:
  $(basename "$0") run Safari
  $(basename "$0") shim list
  $(basename "$0") env list
EOF
    exit 1
}

log()  { printf '[sigma-mac-run] %s\n' "$*"; }
die()  { printf '[sigma-mac-run] ERROR: %s\n' "$*" >&2; exit 1; }

# ---------------------------------------------------------------------------
# Shim table: macOS app name -> Linux equivalent
# ---------------------------------------------------------------------------
declare -A SHIMS=(
    ["Safari"]="chromium"
    ["Chrome"]="chromium"
    ["Finder"]="nautilus"
    ["Terminal"]="foot"
    ["TextEdit"]="helix"
    ["Preview"]="eog"
    ["Photos"]="shotwell"
    ["Messages"]="signal-desktop"
    ["Mail"]="thunderbird"
    ["Calendar"]="gnome-calendar"
    ["Notes"]="xed"
    ["Xcode"]="code"
    ["Activity Monitor"]="btop"
    ["System Preferences"]="sigma-settings"
    ["App Store"]="sigma-software-manager"
)

# ---------------------------------------------------------------------------
# Environment translation table: macOS env var -> Linux equivalent
# ---------------------------------------------------------------------------
declare -A ENV_MAP=(
    ["HOME"]="HOME"
    ["TMPDIR"]="TMPDIR:/tmp"
    ["USER"]="USER"
    ["SHELL"]="SHELL"
    ["DYLD_LIBRARY_PATH"]="LD_LIBRARY_PATH"
    ["DYLD_FALLBACK_LIBRARY_PATH"]="LD_LIBRARY_PATH"
    ["NSDocumentDirectory"]="XDG_DOCUMENTS_DIR"
    ["NSDownloadDirectory"]="XDG_DOWNLOAD_DIR"
    ["NSPicturesDirectory"]="XDG_PICTURES_DIR"
    ["NSMusicDirectory"]="XDG_MUSIC_DIR"
    ["NSMoviesDirectory"]="XDG_VIDEOS_DIR"
)

# ---------------------------------------------------------------------------
# run <app>
# ---------------------------------------------------------------------------
cmd_run() {
    local app="${1:-}"
    [[ -z "$app" ]] && die "run requires an app name. Try: $0 shim list"

    local linux_cmd="${SHIMS[$app]:-}"
    if [[ -z "$linux_cmd" ]]; then
        die "no shim registered for '$app'. Available: $(printf '%s ' "${!SHIMS[@]}")"
    fi

    log "Launching '$app' via shim -> $linux_cmd"
    # In a real implementation this would exec $linux_cmd with env translation.
    # Here we print the resolved command for test-safe execution.
    log "exec: $linux_cmd"
}

# ---------------------------------------------------------------------------
# shim list
# ---------------------------------------------------------------------------
cmd_shim_list() {
    log "Registered macOS app shims:"
    printf '  %-30s %s\n' "macOS App" "Linux Equivalent"
    printf '  %-30s %s\n' "---------" "----------------"
    for app in $(printf '%s\n' "${!SHIMS[@]}" | sort); do
        printf '  %-30s %s\n' "$app" "${SHIMS[$app]}"
    done
}

# ---------------------------------------------------------------------------
# env list
# ---------------------------------------------------------------------------
cmd_env_list() {
    log "Environment variable translation table:"
    printf '  %-40s %s\n' "macOS Variable" "Linux Equivalent"
    printf '  %-40s %s\n' "--------------" "----------------"
    for var in $(printf '%s\n' "${!ENV_MAP[@]}" | sort); do
        printf '  %-40s %s\n' "$var" "${ENV_MAP[$var]}"
    done
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
[[ $# -eq 0 ]] && usage

cmd="${1:-}"
shift || true

case "$cmd" in
    run)
        cmd_run "${1:-}"
        ;;
    shim)
        subcmd="${1:-}"
        shift || true
        case "$subcmd" in
            list) cmd_shim_list ;;
            *)    die "unknown shim subcommand '$subcmd'" ;;
        esac
        ;;
    env)
        subcmd="${1:-}"
        shift || true
        case "$subcmd" in
            list) cmd_env_list ;;
            *)    die "unknown env subcommand '$subcmd'" ;;
        esac
        ;;
    -h|--help)
        usage
        ;;
    *)
        die "unknown subcommand '$cmd'"
        ;;
esac
