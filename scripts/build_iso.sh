#!/usr/bin/env bash
# Fail closed until SigmaOS has a real bare-metal kernel, initramfs, and boot test.
set -euo pipefail

cat >&2 <<'EOF'
SigmaOS does not yet have a validated bootable image build.

The current kernel binary is a hosted Rust target, the initramfs is not built,
and the boot-to-shell path has not been verified. Refusing to emit a placeholder
ISO prevents an empty or non-bootable file from being mistaken for installation
media. See docs/PROJECT_STATUS.md and wiki/01-Installation.md for release gates.
EOF
exit 1
