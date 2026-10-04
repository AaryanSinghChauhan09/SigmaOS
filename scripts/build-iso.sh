#!/usr/bin/env bash
# Fail closed until SigmaOS has a real bare-metal kernel, initramfs, and boot test.
set -euo pipefail

cat >&2 <<'EOF'
SigmaOS does not yet have a validated bootable image build.

The kernel binary is not verified as a bare-metal boot target, the initramfs is
not built, and the boot-to-shell path has not been verified. This script will
not emit a placeholder ISO that could be mistaken for installation media.
See docs/PROJECT_STATUS.md and wiki/01-Installation.md for release gates.
EOF
exit 1
