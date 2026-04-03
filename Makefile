# Σ SIGMAOS ZENITH: SOVEREIGN MAKEFILE (v2900.0)
# Mission: Absolute Build Purity & Hardware-Direct Compilation.

CC = x86_64-elf-gcc
AS = nasm
LD = x86_64-elf-ld

CFLAGS = -m64 -ffreestanding -fno-stack-protector -fno-builtin -nostdlib -Ikernel
LDFLAGS = -T kernel/kernel.ld -nostdlib

# Σ CORE KERNEL SHARDS (WORKING)
OBJS = \
    build/boot.o \
    build/kmain.o \
    build/slab.o \
    build/syscall.o \
    build/vfs_stub.o \
    build/vga.o \
    build/sigma_strings.o \
    build/exceptions.o \
    build/signals.o \
    build/sigma_shell_cli.o

all: build build/sigmaos_zenith

build:
	mkdir -p build

build/boot.o: kernel/boot.asm
	$(AS) -f elf64 kernel/boot.asm -o build/boot.o

build/kmain.o: kernel/kmain.c
	$(CC) $(CFLAGS) -c kernel/kmain.c -o build/kmain.o

build/slab.o: kernel/mm/slab.c
	$(CC) $(CFLAGS) -c kernel/mm/slab.c -o build/slab.o

build/syscall.o: kernel/syscall.c
	$(CC) $(CFLAGS) -c kernel/syscall.c -o build/syscall.o

build/vfs_stub.o: kernel/vfs_stub.c
	$(CC) $(CFLAGS) -c kernel/vfs_stub.c -o build/vfs_stub.o

build/sigmaos_zenith: $(OBJS)
	$(LD) $(LDFLAGS) $(OBJS) -o build/sigmaos_zenith

clean:
	rm -rf build
