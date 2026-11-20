# /etc/profile.d/sigmaos.sh
# Sourced by login shells. Keeps environment bootstrap separate from
# session-level config (which lives in /etc/environment.d/ and uwsm).

# SigmaOS-specific PATH additions. Keeps system binaries first
# (least privilege: user scripts don't shadow system tools).
PATH="/usr/local/sbin:/usr/local/bin:/usr/bin:/bin${PATH:+:$PATH}"

# Editor preference (used by many tools).
export EDITOR="${EDITOR:-vi}"

# Pager.
export PAGER="${PAGER:-less}"

# Less options: quit if one screen, raw control chars, case-insensitive search.
export LESS="${LESS:-R}"

# Enable color in grep/ls by default.
export GREP_OPTIONS="--color=auto" 2>/dev/null || true
export LS_OPTIONS="--color=auto" 2>/dev/null || true

# SigmaOS data directories.
export XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
export XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
export XDG_STATE_HOME="${XDG_STATE_HOME:-$HOME/.local/state}"
export XDG_CACHE_HOME="${XDG_CACHE_HOME:-$HOME/.cache}"
