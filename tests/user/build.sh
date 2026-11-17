#!/bin/sh
# Build the ring-3 test ELF for the SigmaOS exec loader.
set -eu
cd "$(dirname "$0")"
gcc -m32 -ffreestanding -static -no-pie -nostdlib -O2 hello.c -o hello.elf
file hello.elf
echo "Copy to FAT image: mcopy -i <img> hello.elf ::HELLO.ELF"
