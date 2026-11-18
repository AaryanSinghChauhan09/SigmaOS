#!/usr/bin/env bash
# SigmaOS Substrate Sandbox & Isolation Container
# Leverages unshare / bubblewrap (bwrap) to run an isolated, real root environment
# providing an Arch Linux / Omarchy userspace sandbox without touching host OS.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SANDBOX_DIR="/tmp/sigmaos-sandbox"

mkdir -p "$SANDBOX_DIR"

echo -e "\033[1;36m==> Launching SigmaOS Isolated Sandbox Substrate...\033[0m"

if command -v bwrap >/dev/null 2>&1; then
    echo "[✓] Utilizing Bubblewrap (bwrap) unprivileged user namespace sandbox..."
    exec bwrap \
        --ro-bind /usr /usr \
        --ro-bind /lib /lib \
        --ro-bind-try /lib64 /lib64 \
        --ro-bind /bin /bin \
        --ro-bind /sbin /sbin \
        --ro-bind-try /etc /etc \
        --dir /tmp \
        --dir /var \
        --proc /proc \
        --dev /dev \
        --bind "$ROOT_DIR" /sigmaos \
        --bind "$HOME" "$HOME" \
        --setenv PATH "/sigmaos/bin:$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin" \
        --setenv SIGMAOS_SANDBOX "1" \
        --hostname sigmaos-box \
        bash -c "echo -e '\033[1;32m[✓] Inside SigmaOS Isolated Substrate.\033[0m'; exec bash --rcfile <(echo 'PS1=\"sigmaos-box:\w\$ \"')"
else
    echo "[✓] Utilizing unshare namespace isolation..."
    exec unshare --user --pid --mount --uts --fork \
        bash -c "export PATH=\"$ROOT_DIR/bin:$HOME/.local/bin:$PATH\"; exec bash --rcfile <(echo 'PS1=\"sigmaos-box:\w\$ \"')"
fi
