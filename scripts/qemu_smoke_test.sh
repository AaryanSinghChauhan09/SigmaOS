#!/usr/bin/env sh
# A smoke test must never pass by simulating QEMU or manufacturing an image.
set -eu

QEMU_BIN="$(command -v qemu-system-x86_64 || true)"
IMAGE="build/sigmaos.iso"

if [ -z "$QEMU_BIN" ]; then
    echo "QEMU boot test unavailable: qemu-system-x86_64 is not installed." >&2
    exit 2
fi
if [ ! -s "$IMAGE" ]; then
    echo "QEMU boot test unavailable: no non-empty ISO at $IMAGE." >&2
    exit 2
fi

echo "A real ISO is present, but the kernel has no verified runtime-ready serial marker." >&2
echo "Refusing to report a successful boot based only on QEMU staying alive." >&2
exit 2
