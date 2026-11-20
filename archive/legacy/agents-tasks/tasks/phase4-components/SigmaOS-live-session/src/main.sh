#!/usr/bin/env bash
# SigmaOS Live Session Manager — manages the live ISO environment
# Upstream inspiration: archiso live-session (https://gitlab.archlinux.org/archlinux/archiso),
#                       omarchy-iso (https://github.com/basecamp/omarchy-iso)
set -euo pipefail

LIVE_USER="${LIVE_USER:-sigma}"
LIVE_HOME="${LIVE_HOME:-/home/$LIVE_USER}"
SESSION_MARKER="/run/sigmaos-live-session"
LOG_FILE="/var/log/sigmaos-live-session.log"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

Subcommands:
  start    Initialise the live session (create user, set up desktop, launch shell)
  stop     Tear down the live session
  status   Show live session status
  shell    Launch an interactive shell as the live user

Options:
  --user <name>    Live user name (default: sigma, env: LIVE_USER)
  --desktop <name>  Desktop to launch (hyprland/sway/none)
  -h, --help        Show this help
EOF
    exit 1
}

log()  { printf '[live-session] %s\n' "$*" | tee -a "$LOG_FILE" 2>/dev/null || printf '[live-session] %s\n' "$*"; }
die()  { printf '[live-session] ERROR: %s\n' "$*" >&2; exit 1; }

DESKTOP="hyprland"

# ---------------------------------------------------------------------------
# start — initialise the live session
# ---------------------------------------------------------------------------
cmd_start() {
    log "Initialising SigmaOS live session"

    # Check if already running
    if [[ -f "$SESSION_MARKER" ]]; then
        log "Live session already active (marker exists: $SESSION_MARKER)"
        return 0
    fi

    # Step 1: Create live user if it doesn't exist
    if ! id "$LIVE_USER" &>/dev/null; then
        log "Creating live user: $LIVE_USER"
        useradd -m -s /bin/bash -G wheel,video,audio,network "$LIVE_USER" 2>/dev/null \
            || log "  (user creation may require root — continuing)"
    fi

    # Step 2: Set up home directory with live defaults
    if [[ -d "$LIVE_HOME" ]]; then
        log "Setting up live home: $LIVE_HOME"
        # Copy skeleton defaults if home is empty
        if [[ -z "$(ls -A "$LIVE_HOME" 2>/dev/null)" ]]; then
            cp -r /etc/skel/. "$LIVE_HOME/" 2>/dev/null || true
        fi
    fi

    # Step 3: Configure live session environment
    log "Configuring live session environment"
    # Set hostname for live session
    echo "sigmaos-live" > /etc/hostname 2>/dev/null || true
    # Enable NetworkManager for live session
    systemctl enable --now NetworkManager 2>/dev/null || true

    # Step 4: Write session marker
    mkdir -p "$(dirname "$SESSION_MARKER")"
    cat > "$SESSION_MARKER" <<EOF
user=$LIVE_USER
desktop=$DESKTOP
started_at=$(date -Iseconds)
pid=$$
EOF
    log "Session marker written: $SESSION_MARKER"

    # Step 5: Launch desktop or shell
    if [[ "$DESKTOP" == "none" ]]; then
        log "Desktop disabled — session ready for manual login"
    else
        log "Live session ready — desktop: $DESKTOP"
        log "Use '$0 shell' for an interactive shell as $LIVE_USER"
    fi

    log "Live session initialisation complete"
}

# ---------------------------------------------------------------------------
# stop — tear down the live session
# ---------------------------------------------------------------------------
cmd_stop() {
    log "Tearing down SigmaOS live session"

    if [[ ! -f "$SESSION_MARKER" ]]; then
        log "No active live session (marker not found)"
        return 0
    fi

    # Remove session marker
    rm -f "$SESSION_MARKER"
    log "Session marker removed"

    log "Live session torn down"
}

# ---------------------------------------------------------------------------
# status — show live session status
# ---------------------------------------------------------------------------
cmd_status() {
    if [[ ! -f "$SESSION_MARKER" ]]; then
        echo "No active live session."
        return 0
    fi

    echo "Live session active:"
    while IFS='=' read -r key val; do
        printf "  %-12s %s\n" "$key:" "$val"
    done < "$SESSION_MARKER"
}

# ---------------------------------------------------------------------------
# shell — launch interactive shell as live user
# ---------------------------------------------------------------------------
cmd_shell() {
    if [[ ! -f "$SESSION_MARKER" ]]; then
        die "no active live session. Run '$0 start' first."
    fi
    log "Launching shell as $LIVE_USER"
    # In a real impl: su - "$LIVE_USER"
    echo "Would launch: su - $LIVE_USER"
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
[[ $# -eq 0 ]] && usage

# Pre-scan for global flags
while [[ $# -gt 0 ]]; do
    case "$1" in
        --user)    LIVE_USER="$2"; shift 2 ;;
        --desktop) DESKTOP="$2"; shift 2 ;;
        -h|--help) usage ;;
        *)         args+=("$1"); shift ;;
    esac
done

# Re-parse remaining as subcommand
cmd="${args[0]:-}"
shift_args=("${args[@]:1}") || true

case "$cmd" in
    start)  cmd_start ;;
    stop)   cmd_stop ;;
    status) cmd_status ;;
    shell)  cmd_shell ;;
    *)      die "unknown subcommand '$cmd'" ;;
esac
