#!/usr/bin/env bash
# SigmaOS-dots-configuration: Dotfiles and configuration management system
# Manages symlinks, profiles, and configuration templates for SigmaOS.

set -euo pipefail

DOTS_DIR="${DOTS_DIR:-$HOME/.sigmadots}"
BACKUP_DIR="${BACKUP_DIR:-$HOME/.sigmadots-backup}"
CONFIG_FILE="${CONFIG_FILE:-./dots.conf}"

log() { printf '[dots] %s\n' "$*"; }
warn() { printf '[dots] WARNING: %s\n' "$*" >&2; }
die() { printf '[dots] ERROR: %s\n' "$*" >&2; exit 1; }

# Ensure required directories exist
ensure_dirs() {
    mkdir -p "$DOTS_DIR" "$BACKUP_DIR"
    log "Directories ready: $DOTS_DIR, $BACKUP_DIR"
}

# Back up an existing file before overwriting it
backup_file() {
    local target="$1"
    if [ -e "$target" ] && [ ! -L "$target" ]; then
        local backup="${BACKUP_DIR}/$(basename "$target").$(date +%Y%m%d%H%M%S)"
        cp -a "$target" "$backup"
        log "Backed up $target -> $backup"
    fi
}

# Create a symlink from source to target, backing up first
link_dot() {
    local src="$1"
    local target="$2"
    backup_file "$target"
    ln -sf "$src" "$target"
    log "Linked $src -> $target"
}

# Apply all dotfiles listed in the config file (format: src|target per line)
apply_dots() {
    if [ ! -f "$CONFIG_FILE" ]; then
        warn "Config file $CONFIG_FILE not found; nothing to apply."
        return 0
    fi

    while IFS='|' read -r src target; do
        # Skip comments and blank lines
        case "$src" in ''|\#*) continue ;; esac
        [ -z "$target" ] && { warn "Missing target for $src; skipping."; continue; }
        [ -e "$src" ] || { warn "Source $src does not exist; skipping."; continue; }
        link_dot "$(realpath "$src")" "$target"
    done < "$CONFIG_FILE"
}

# Print usage information
usage() {
    cat <<EOF
Usage: $(basename "$0") [command]

Commands:
  apply   Apply dotfiles from $CONFIG_FILE (default)
  list    List currently managed dotfiles
  backup  Back up existing target files without linking
  help    Show this message

Environment:
  DOTS_DIR     Directory for dot sources  (default: ~/.sigmadots)
  BACKUP_DIR   Directory for backups      (default: ~/.sigmadots-backup)
  CONFIG_FILE  Path to dots.conf          (default: ./dots.conf)
EOF
}

# List managed symlinks
list_dots() {
    log "Managed symlinks in $DOTS_DIR:"
    find "$DOTS_DIR" -maxdepth 2 -type l | while read -r link; do
        printf '  %s -> %s\n' "$link" "$(readlink "$link")"
    done
}

main() {
    local cmd="${1:-apply}"
    ensure_dirs
    case "$cmd" in
        apply)  apply_dots ;;
        list)   list_dots ;;
        backup) log "Backup-only mode: no symlinks created."; apply_dots ;;
        help|-h|--help) usage ;;
        *) die "Unknown command: $cmd. Run with 'help' for usage." ;;
    esac
    log "Done."
}

main "$@"
