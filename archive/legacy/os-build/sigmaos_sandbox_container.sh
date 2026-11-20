#!/usr/bin/env bash
# SigmaOS Substrate Sandbox & Isolation Container
# Leverages unshare / bubblewrap (bwrap) to run an isolated, real root environment
# providing an Arch Linux / Omarchy userspace sandbox without touching host OS.
# Supports full GUI (X11 & Wayland), GPU acceleration (DRM/DRI), and audio.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SANDBOX_DIR="/tmp/sigmaos-sandbox"

mkdir -p "$SANDBOX_DIR"

echo -e "\033[1;36m==> Launching SigmaOS Isolated Sandbox Substrate...\033[0m"

if command -v bwrap >/dev/null 2>&1; then
    echo "[✓] Utilizing Bubblewrap (bwrap) unprivileged user namespace sandbox with GUI passthrough..."

    BWRAP_CMD=(
        bwrap
        --ro-bind /usr /usr
        --ro-bind /lib /lib
        --ro-bind-try /lib64 /lib64
        --ro-bind /bin /bin
        --ro-bind /sbin /sbin
        --ro-bind-try /etc /etc
        --ro-bind-try /sys /sys
        --dir /tmp
        --ro-bind-try /tmp/.X11-unix /tmp/.X11-unix
        --ro-bind-try /run/user /run/user
        --dir /var
        --proc /proc
        --dev /dev
        --dev-bind-try /dev/dri /dev/dri
        --dev-bind-try /dev/snd /dev/snd
        --bind "$ROOT_DIR" /sigmaos
        --bind "$HOME" "$HOME"
        --setenv PATH "$HOME/.cargo/bin:/sigmaos/bin:$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin"
        --setenv SIGMAOS_SANDBOX "1"
        --setenv SIGMAOS_ROOT "/sigmaos"
        --setenv DISPLAY "${DISPLAY:-:0}"
        --setenv XDG_RUNTIME_DIR "${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"
        --setenv XAUTHORITY "${XAUTHORITY:-$HOME/.Xauthority}"
        --setenv USER "${USER:-$(whoami)}"
        --setenv HOME "$HOME"
        --setenv TERM "${TERM:-xterm-256color}"
        --setenv COLORTERM "${COLORTERM:-truecolor}"
        --setenv WEBKIT_DISABLE_COMPOSITING_MODE "1"
        --setenv WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS "1"
        --unshare-uts
        --hostname sigmaos-box
    )

    if [ -n "${WAYLAND_DISPLAY:-}" ]; then
        BWRAP_CMD+=(--setenv WAYLAND_DISPLAY "$WAYLAND_DISPLAY")
    fi

    if [ $# -gt 0 ]; then
        exec "${BWRAP_CMD[@]}" "$@"
    else
        exec "${BWRAP_CMD[@]}" bash -c "
            echo -e '\033[1;32m[✓] Inside SigmaOS Isolated Substrate with GUI Passthrough.\033[0m'
            echo -e '    Commands: \033[1;34msigmaos --gui\033[0m, \033[1;34msigmaos --native\033[0m, \033[1;34msigmaos --doctor\033[0m, \033[1;34mrun.sh\033[0m'
            exec bash --rcfile <(echo 'PS1=\"sigmaos-box:\w\$ \"')
        "
    fi
else
    echo "[✓] Utilizing unshare namespace isolation..."
    if [ $# -gt 0 ]; then
        exec unshare --user --pid --mount --uts --fork \
            bash -c "export PATH=\"$HOME/.cargo/bin:$ROOT_DIR/bin:$HOME/.local/bin:$PATH\"; exec \"$@\""
    else
        exec unshare --user --pid --mount --uts --fork \
            bash -c "export PATH=\"$HOME/.cargo/bin:$ROOT_DIR/bin:$HOME/.local/bin:$PATH\"; exec bash --rcfile <(echo 'PS1=\"sigmaos-box:\w\$ \"')"
    fi
fi
