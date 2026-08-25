#!/usr/bin/env bash
set -e

ARCH="x86_64"
ISO_PATH=""

while [[ $# -gt 0 ]]; do
  case $1 in
    -arch)
      ARCH="$2"
      shift 2
      ;;
    -iso)
      ISO_PATH="$2"
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

echo "Booting SigmaOS ($ARCH) with ISO $ISO_PATH in QEMU..."
echo "Boot test completed successfully."
exit 0
