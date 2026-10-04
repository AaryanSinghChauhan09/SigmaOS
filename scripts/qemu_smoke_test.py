#!/usr/bin/env python3
"""Fail closed when no real, validated SigmaOS boot image is available."""

from pathlib import Path
import shutil
import sys


def main() -> int:
    image = Path("build/sigmaos-desktop-preview.iso")
    qemu = shutil.which("qemu-system-x86_64")
    if qemu is None:
        print("QEMU boot test unavailable: qemu-system-x86_64 is not installed.", file=sys.stderr)
        return 2
    if not image.is_file() or image.stat().st_size == 0:
        print(f"QEMU boot test unavailable: no non-empty image at {image}.", file=sys.stderr)
        return 2
    with image.open("rb") as stream:
        stream.seek(32769)
        if stream.read(5) != b"CD001":
            print(f"Invalid ISO9660 image: {image} has no ISO9660 volume descriptor.", file=sys.stderr)
            return 1
    print("Image is a non-empty ISO9660 file, but SigmaOS has no verified boot-ready marker.", file=sys.stderr)
    print("This check cannot report a successful OS boot until the kernel emits a runtime-ready marker.", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
