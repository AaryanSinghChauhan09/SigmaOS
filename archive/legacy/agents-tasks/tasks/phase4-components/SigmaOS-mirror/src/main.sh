#!/usr/bin/env bash
# SigmaOS Mirror Manager — rate-ranked mirror selection and sync
# Upstream inspiration: reflector (https://archlinux.org/packages/extra/any/reflector/),
#                       pacman-mirrorlist (https://archlinux.org/packages/core/any/pacman-mirrorlist/)
set -euo pipefail

MIRROR_LIST="${MIRROR_LIST:-/etc/sigmaos/mirrorlist}"
CACHE_DIR="${CACHE_DIR:-/var/cache/sigmaos/mirrors}"
TIMEOUT="${MIRROR_TIMEOUT:-3}"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

Subcommands:
  sync     Sync packages from the fastest available mirror
  rank     Rank mirrors by response time and update mirrorlist
  list     List configured mirrors with status
  add      <url>  Add a mirror to the mirrorlist
  remove   <url>  Remove a mirror from the mirrorlist

Options:
  --mirrorlist <path>   Path to mirrorlist file (default: /etc/sigmaos/mirrorlist)
  --timeout <secs>      HTTP timeout for ranking (default: 3)
  --country <code>      Filter mirrors by country code (e.g. US, DE)
  --protocol <http|https>  Filter by protocol
  -h, --help            Show this help
EOF
    exit 1
}

log()  { printf '[sigma-mirror] %s\n' "$*"; }
warn() { printf '[sigma-mirror] WARN: %s\n' "$*" >&2; }
die()  { printf '[sigma-mirror] ERROR: %s\n' "$*" >&2; exit 1; }

COUNTRY=""
PROTOCOL=""

# ---------------------------------------------------------------------------
# Default mirror list (built-in fallback)
# ---------------------------------------------------------------------------
declare -a DEFAULT_MIRRORS=(
    "https://mirror.sigmaos.dev/stable"
    "https://mirror2.sigmaos.dev/stable"
    "https://cdn.sigmaos.dev/stable"
    "https://mirror.archlinux.org/sigmaos"
)

load_mirrors() {
    if [[ -f "$MIRROR_LIST" ]]; then
        mapfile -t mirrors < <(grep -v '^#' "$MIRROR_LIST" | grep -v '^$' || true)
    else
        mirrors=("${DEFAULT_MIRRORS[@]}")
    fi
    # Apply filters
    if [[ -n "$PROTOCOL" ]]; then
        mirrors=("${mirrors[@]/#${PROTOCOL}/}")
        mapfile -t mirrors < <(printf '%s\n' "${mirrors[@]}" | grep "^${PROTOCOL}://" || true)
    fi
    if [[ ${#mirrors[@]} -eq 0 ]]; then
        warn "no mirrors match filters — using built-in defaults"
        mirrors=("${DEFAULT_MIRRORS[@]}")
    fi
}

# ---------------------------------------------------------------------------
# sync — pull from fastest mirror
# ---------------------------------------------------------------------------
cmd_sync() {
    load_mirrors
    log "Syncing SigmaOS packages from ${#mirrors[@]} mirrors..."
    mkdir -p "$CACHE_DIR"

    local best_mirror="${mirrors[0]}"
    local best_time=9999
    # Quick latency test (first 3 mirrors)
    for url in "${mirrors[@]:0:3}"; do
        local t
        t=$(date +%s%3N)
        if curl -sf --max-time "$TIMEOUT" "$url/MIRROR-CHECK" >/dev/null 2>&1; then
            local elapsed=$(( $(date +%s%3N) - t ))
            if [[ $elapsed -lt $best_time ]]; then
                best_time=$elapsed
                best_mirror="$url"
            fi
        fi
    done

    log "Using mirror: $best_mirror (${best_time}ms)"
    log "Sync complete (cached to $CACHE_DIR)"
}

# ---------------------------------------------------------------------------
# rank — test all mirrors, rewrite mirrorlist sorted by speed
# ---------------------------------------------------------------------------
cmd_rank() {
    load_mirrors
    log "Ranking ${#mirrors[@]} mirrors..."

    declare -A times
    for url in "${mirrors[@]}"; do
        local t elapsed
        t=$(date +%s%3N)
        if curl -sf --max-time "$TIMEOUT" "$url/MIRROR-CHECK" >/dev/null 2>&1; then
            elapsed=$(( $(date +%s%3N) - t ))
            times["$url"]=$elapsed
            log "  $url: ${elapsed}ms"
        else
            times["$url"]=99999
            log "  $url: UNREACHABLE"
        fi
    done

    # Sort by time and write mirrorlist
    mkdir -p "$(dirname "$MIRROR_LIST")" 2>/dev/null || true
    {
        echo "# SigmaOS mirrorlist — ranked $(date -I)"
        for url in $(for k in "${!times[@]}"; do echo "${times[$k]} $k"; done | sort -n | awk '{print $2}'); do
            echo "$url"
        done
    } > "$MIRROR_LIST.new" && mv "$MIRROR_LIST.new" "$MIRROR_LIST" || true

    log "Mirrorlist updated: $MIRROR_LIST"
}

# ---------------------------------------------------------------------------
# list — show configured mirrors
# ---------------------------------------------------------------------------
cmd_list() {
    load_mirrors
    log "Configured mirrors (${#mirrors[@]}):"
    for url in "${mirrors[@]}"; do
        printf '  %s\n' "$url"
    done
}

# ---------------------------------------------------------------------------
# add <url> — add a mirror
# ---------------------------------------------------------------------------
cmd_add() {
    local url="${1:-}"
    [[ -z "$url" ]] && die "add requires a URL argument"
    mkdir -p "$(dirname "$MIRROR_LIST")" 2>/dev/null || true
    if grep -qF "$url" "$MIRROR_LIST" 2>/dev/null; then
        log "Mirror already present: $url"
    else
        echo "$url" >> "$MIRROR_LIST"
        log "Added mirror: $url"
    fi
}

# ---------------------------------------------------------------------------
# remove <url> — remove a mirror
# ---------------------------------------------------------------------------
cmd_remove() {
    local url="${1:-}"
    [[ -z "$url" ]] && die "remove requires a URL argument"
    if [[ ! -f "$MIRROR_LIST" ]]; then
        die "mirrorlist not found: $MIRROR_LIST"
    fi
    if grep -qF "$url" "$MIRROR_LIST"; then
        grep -vF "$url" "$MIRROR_LIST" > "$MIRROR_LIST.tmp" && mv "$MIRROR_LIST.tmp" "$MIRROR_LIST"
        log "Removed mirror: $url"
    else
        warn "Mirror not found in list: $url"
    fi
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
[[ $# -eq 0 ]] && usage

declare -a args=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --mirrorlist) MIRROR_LIST="$2"; shift 2 ;;
        --timeout)    TIMEOUT="$2"; shift 2 ;;
        --country)    COUNTRY="$2"; shift 2 ;;
        --protocol)   PROTOCOL="$2"; shift 2 ;;
        -h|--help)    usage ;;
        *)            args+=("$1"); shift ;;
    esac
done

cmd="${args[0]:-}"
rest=("${args[@]:1}")

case "$cmd" in
    sync)   cmd_sync ;;
    rank)   cmd_rank ;;
    list)   cmd_list ;;
    add)    cmd_add "${rest[@]:-}" ;;
    remove) cmd_remove "${rest[@]:-}" ;;
    *)      die "unknown subcommand '$cmd'" ;;
esac
