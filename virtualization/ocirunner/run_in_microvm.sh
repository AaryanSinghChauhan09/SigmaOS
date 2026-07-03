#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# virtualization/ocirunner/run_in_microvm.sh
# Minimal microVM OCI launcher for SigmaOS CI and development.
# Runs an OCI image inside a lightweight VM (QEMU microVM mode).
#
# Usage:
#   ./run_in_microvm.sh <oci-image> [cmd...]
#   ./run_in_microvm.sh nginx:latest
#   ./run_in_microvm.sh ubuntu:22.04 bash -c "echo hello"
#
# Requirements: qemu-system-x86_64, docker (for image pull)
set -euo pipefail

IMAGE="${1:-ubuntu:22.04}"
shift || true
CMD="${*:-/bin/sh}"

WORKDIR="$(mktemp -d /tmp/sigma-microvm-XXXXXX)"
ROOTFS="${WORKDIR}/rootfs.ext4"
KERNEL="${WORKDIR}/vmlinux"
INITRD="${WORKDIR}/initrd"

cleanup() { rm -rf "${WORKDIR}"; }
trap cleanup EXIT

echo "[sigma-microvm] Pulling OCI image: ${IMAGE}"
docker pull --quiet "${IMAGE}" 2>/dev/null || true

echo "[sigma-microvm] Extracting rootfs"
CID=$(docker create "${IMAGE}" sh)
docker export "${CID}" | tar -C "${WORKDIR}" -x 2>/dev/null || true
docker rm "${CID}" >/dev/null

# Build a minimal ext4 rootfs (32 MB)
dd if=/dev/zero of="${ROOTFS}" bs=1M count=32 status=none
mkfs.ext4 -q "${ROOTFS}" 2>/dev/null || true

# Use a prebuilt minimal kernel if available, else fall back to host
PREBUILT_KERNEL="/opt/sigma/microvm/vmlinux"
if [[ -f "${PREBUILT_KERNEL}" ]]; then
    cp "${PREBUILT_KERNEL}" "${KERNEL}"
else
    # Use host kernel as fallback for CI
    KERNEL="/boot/vmlinuz-$(uname -r)"
    [[ -f "${KERNEL}" ]] || KERNEL="$(ls /boot/vmlinuz* 2>/dev/null | tail -1)"
fi

echo "[sigma-microvm] Booting: image=${IMAGE} cmd=${CMD}"
timeout 60 qemu-system-x86_64 \
    -M microvm,x-option-roms=off,pic=off,pit=off,rtc=off \
    -enable-kvm -m 256M -smp 2 -nographic \
    -kernel "${KERNEL}" \
    -append "console=ttyS0 root=/dev/vda rw quiet init=/bin/sh -- -c '${CMD}'" \
    -drive id=rootfs,file="${ROOTFS}",format=raw,if=none \
    -device virtio-blk-device,drive=rootfs \
    -netdev user,id=net0 \
    -device virtio-net-device,netdev=net0 \
    -no-reboot 2>&1 | grep -v "^QEMU" || true

echo "[sigma-microvm] Done"
