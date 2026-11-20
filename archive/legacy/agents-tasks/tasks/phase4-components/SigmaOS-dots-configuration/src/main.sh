#!/usr/bin/env bash
# SigmaOS dots-configuration — omadots/chezmoi-inspired declarative dotfile manager
# Upstream inspiration: omadots (https://github.com/basecamp/omarchy),
#                       chezmoi  (https://www.chezmoi.io/)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOTS_ROOT="${DOTS_ROOT:-$SCRIPT_DIR/../dots}"
DEST="${DESTDIR:-$HOME}"
BACKUP_DIR="${BACKUP_DIR:-$HOME/.sigma-dots-backup}"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

Subcommands:
  install    Deploy dotfile manifest to DEST (default: \$HOME)
  apply      Re-apply changed dotfiles (idempotent)
  backup     Back up existing dotfiles before deployment
  diff       Show what would change without applying

Options:
  --dest <dir>    Override destination directory (default: \$HOME)
  --dry-run       Print actions without executing them
  -h, --help      Show this help

Environment:
  DOTS_ROOT   Source dotfiles directory (default: ../dots relative to script)
  DESTDIR     Override destination (same as --dest)
  BACKUP_DIR  Backup directory (default: \$HOME/.sigma-dots-backup)
EOF
    exit 1
}

log()  { printf '[sigma-dots] %s\n' "$*"; }
warn() { printf '[sigma-dots] WARN: %s\n' "$*" >&2; }
die()  { printf '[sigma-dots] ERROR: %s\n' "$*" >&2; exit 1; }

DRY_RUN=0

# ---------------------------------------------------------------------------
# Declarative dotfile manifest: source_relative_path -> dest_relative_path
# Each entry is "source:dest" (relative to DOTS_ROOT and DEST respectively)
# ---------------------------------------------------------------------------
declare -a MANIFEST=(
    "hypr/hyprland.conf:.config/hypr/hyprland.conf"
    "hypr/hyprlock.conf:.config/hypr/hyprlock.conf"
    "hypr/hypridle.conf:.config/hypr/hypridle.conf"
    "waybar/config:.config/waybar/config"
    "waybar/style.css:.config/waybar/style.css"
    "mako/config:.config/mako/config"
    "foot/foot.ini:.config/foot/foot.ini"
    "nvim/init.lua:.config/nvim/init.lua"
    "zsh/.zshrc:.zshrc"
    "zsh/.zshenv:.zshenv"
    "git/.gitconfig:.gitconfig"
)

do_link() {
    local src="$1" dst="$2"
    local src_abs="$DOTS_ROOT/$src"
    local dst_abs="$DEST/$dst"

    if [[ ! -e "$src_abs" ]]; then
        warn "source not found: $src_abs (skipping)"
        return 0
    fi

    mkdir -p "$(dirname "$dst_abs")"

    if [[ "$DRY_RUN" -eq 1 ]]; then
        log "  [dry-run] link $src_abs -> $dst_abs"
        return 0
    fi

    if [[ -e "$dst_abs" && ! -L "$dst_abs" ]]; then
        warn "  $dst_abs exists and is not a symlink — skipping (run backup first)"
        return 0
    fi

    ln -sf "$src_abs" "$dst_abs"
    log "  linked $dst"
}

cmd_install() {
    log "Installing SigmaOS dotfiles from $DOTS_ROOT to $DEST"
    for entry in "${MANIFEST[@]}"; do
        local src="${entry%%:*}"
        local dst="${entry##*:}"
        do_link "$src" "$dst"
    done
    log "Install complete (${#MANIFEST[@]} entries)."
}

cmd_apply() {
    log "Applying SigmaOS dotfiles (idempotent re-link)"
    cmd_install
}

cmd_backup() {
    log "Backing up existing dotfiles to $BACKUP_DIR"
    mkdir -p "$BACKUP_DIR"
    for entry in "${MANIFEST[@]}"; do
        local dst="${entry##*:}"
        local dst_abs="$DEST/$dst"
        if [[ -e "$dst_abs" && ! -L "$dst_abs" ]]; then
            local bak="$BACKUP_DIR/$dst"
            mkdir -p "$(dirname "$bak")"
            if [[ "$DRY_RUN" -eq 1 ]]; then
                log "  [dry-run] backup $dst_abs -> $bak"
            else
                cp -a "$dst_abs" "$bak"
                log "  backed up $dst"
            fi
        fi
    done
    log "Backup complete."
}

cmd_diff() {
    log "Diff: showing what would change (no files modified)"
    for entry in "${MANIFEST[@]}"; do
        local src="${entry%%:*}"
        local dst="${entry##*:}"
        local src_abs="$DOTS_ROOT/$src"
        local dst_abs="$DEST/$dst"

        if [[ ! -e "$src_abs" ]]; then
            log "  MISSING_SRC  $src"
        elif [[ ! -e "$dst_abs" ]]; then
            log "  NEW          $dst  <- $src"
        elif [[ -L "$dst_abs" ]]; then
            local current_target
            current_target="$(readlink "$dst_abs")"
            if [[ "$current_target" == "$src_abs" ]]; then
                log "  UNCHANGED    $dst"
            else
                log "  REDIRECT     $dst  (currently -> $current_target, would be -> $src_abs)"
            fi
        else
            log "  CONFLICT     $dst  (exists as non-symlink; run backup first)"
        fi
    done
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
[[ $# -eq 0 ]] && usage

# Pre-scan for global flags
args=()
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=1 ;;
        --dest=*)  DEST="${arg#--dest=}" ;;
        *) args+=("$arg") ;;
    esac
done
set -- "${args[@]+"${args[@]}"}"

[[ $# -eq 0 ]] && usage

cmd="${1:-}"
shift || true

case "$cmd" in
    install) cmd_install "$@" ;;
    apply)   cmd_apply   "$@" ;;
    backup)  cmd_backup  "$@" ;;
    diff)    cmd_diff    "$@" ;;
    -h|--help) usage ;;
    *) die "unknown subcommand '$cmd'" ;;
esac
