#!/usr/bin/env bash
# SigmaOS Node.js Binary — Node.js runtime integration and version manager scaffold
# Upstream: nvm (https://github.com/nvm-sh/nvm), fnm (https://github.com/Schniz/fnm)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSIONS_DIR="${SIGMA_NODE_DIR:-$HOME/.sigma-node/versions}"
ACTIVE_LINK="${SIGMA_NODE_DIR:-$HOME/.sigma-node}/active"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS Node.js binary manager

Subcommands:
  install  <version>     Install a Node.js version (e.g. 20.11.0, lts, latest)
  remove   <version>     Remove an installed version
  use      <version>     Switch the active Node.js version
  list                   List installed versions
  list-remote [--lts]    List available remote versions
  current                Show the active version
  which                  Print the path to the active node binary
  exec     <version> <cmd...>  Run a command with a specific version
  status                 Show manager state
  version                Print manager version

EOF
}

resolve_version() {
    local v="$1"
    case "$v" in
        lts)    echo "20.11.0" ;;  # scaffold: LTS alias
        latest) echo "21.6.0"  ;;  # scaffold: latest alias
        *)      echo "$v"       ;;
    esac
}

cmd_install() {
    local ver
    ver=$(resolve_version "${1:?'version required'}")
    local dir="$VERSIONS_DIR/$ver"
    if [ -d "$dir" ]; then
        echo "[node] Already installed: v$ver"
        return
    fi
    mkdir -p "$dir/bin"
    # Scaffold: create a stub node/npm shim
    cat > "$dir/bin/node" <<NODEEOF
#!/usr/bin/env bash
echo "node v$ver (SigmaOS scaffold stub)"
NODEEOF
    chmod +x "$dir/bin/node"
    cp "$dir/bin/node" "$dir/bin/npm"
    echo "[node] Installed v$ver → $dir"
    echo "[node] Run: $(basename "$0") use $ver  to activate"
}

cmd_remove() {
    local ver
    ver=$(resolve_version "${1:?'version required'}")
    local dir="$VERSIONS_DIR/$ver"
    if [ ! -d "$dir" ]; then
        echo "[node] Not installed: v$ver" >&2; exit 1
    fi
    rm -rf "$dir"
    # Clear active link if it was this version
    if [ -L "$ACTIVE_LINK" ]; then
        local target
        target=$(readlink "$ACTIVE_LINK")
        if [ "$target" = "$dir" ]; then
            rm -f "$ACTIVE_LINK"
            echo "[node] Active link cleared"
        fi
    fi
    echo "[node] Removed: v$ver"
}

cmd_use() {
    local ver
    ver=$(resolve_version "${1:?'version required'}")
    local dir="$VERSIONS_DIR/$ver"
    if [ ! -d "$dir" ]; then
        echo "[node] Not installed: v$ver — run: $(basename "$0") install $ver" >&2; exit 1
    fi
    ln -snf "$dir" "$ACTIVE_LINK"
    echo "[node] Active: v$ver"
    echo "[node] Add to PATH: export PATH=\"$ACTIVE_LINK/bin:\$PATH\""
}

cmd_list() {
    if [ ! -d "$VERSIONS_DIR" ] || [ -z "$(ls "$VERSIONS_DIR" 2>/dev/null)" ]; then
        echo "[node] No versions installed"
        return
    fi
    local active_ver=""
    if [ -L "$ACTIVE_LINK" ]; then
        active_ver=$(basename "$(readlink "$ACTIVE_LINK")")
    fi
    for v in "$VERSIONS_DIR"/*/; do
        local vname
        vname=$(basename "$v")
        local marker=" "
        [ "$vname" = "$active_ver" ] && marker="▶"
        echo "  $marker v$vname"
    done
}

cmd_list_remote() {
    local lts_only="${1:-}"
    echo "Available Node.js versions (scaffold data):"
    if [ "$lts_only" = "--lts" ]; then
        echo "  20.11.0  (LTS Iron)"
        echo "  18.19.0  (LTS Hydrogen)"
        echo "  16.20.2  (LTS Gallium — maintenance)"
    else
        echo "  21.6.0   (Current)"
        echo "  20.11.0  (LTS Iron ★)"
        echo "  18.19.0  (LTS Hydrogen ★)"
        echo "  16.20.2  (LTS Gallium — maintenance)"
        echo "  (full list: https://nodejs.org/en/download/releases)"
    fi
}

cmd_current() {
    if [ ! -L "$ACTIVE_LINK" ]; then
        echo "[node] No active version"; return
    fi
    echo "v$(basename "$(readlink "$ACTIVE_LINK")")"
}

cmd_which() {
    if [ ! -L "$ACTIVE_LINK" ]; then
        echo "[node] No active version" >&2; exit 1
    fi
    echo "$ACTIVE_LINK/bin/node"
}

cmd_exec() {
    local ver
    ver=$(resolve_version "${1:?'version required'}"); shift
    local node_bin="$VERSIONS_DIR/$ver/bin/node"
    if [ ! -f "$node_bin" ]; then
        echo "[node] Not installed: v$ver" >&2; exit 1
    fi
    PATH="$VERSIONS_DIR/$ver/bin:$PATH" exec "$@"
}

cmd_status() {
    local active_ver="(none)"
    if [ -L "$ACTIVE_LINK" ]; then
        active_ver="v$(basename "$(readlink "$ACTIVE_LINK")")"
    fi
    local count=0
    [ -d "$VERSIONS_DIR" ] && count=$(ls "$VERSIONS_DIR" 2>/dev/null | wc -l)
    echo "Manager : SigmaOS Node.js binary manager v0.5"
    echo "Dir     : $VERSIONS_DIR"
    echo "Active  : $active_ver"
    echo "Installed: $count version(s)"
}

case "${1:-}" in
    install)     cmd_install "${2:-}" ;;
    remove)      cmd_remove  "${2:-}" ;;
    use)         cmd_use     "${2:-}" ;;
    list)        cmd_list ;;
    list-remote) cmd_list_remote "${2:-}" ;;
    current)     cmd_current ;;
    which)       cmd_which ;;
    exec)        shift; cmd_exec "$@" ;;
    status)      cmd_status ;;
    version)     echo "SigmaOS Node.js manager v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
