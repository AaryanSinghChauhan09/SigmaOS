# Σ SIGMAOS ZENITH: SOVEREIGN MAKEFILE (v2950.0)
# Mission: Absolute Build Purity & Hardware-Direct Compilation.

CC = x86_64-elf-gcc
AS = nasm
LD = x86_64-elf-ld

CFLAGS = -m64 -ffreestanding -fno-stack-protector -fno-builtin -nostdlib -Ikernel
LDFLAGS = -T kernel/kernel.ld -nostdlib

# Σ CORE KERNEL SHARDS (COMPLETE LINKAGE)
OBJS = \
    build/boot.o \
    build/idt_asm.o \
    build/kmain.o \
    build/hal.o \
    build/idt.o \
    build/slab.o \
    build/syscall.o \
    build/vfs_stub.o \
    build/vga.o \
    build/exceptions.o \
    build/signals.o \
    build/sigma_shell_cli.o \
    build/sigma_shell.o \
    build/sigma_fs.o \
    build/init.o \
    build/vfs.o \
    build/omni_shell.o \
    build/AmnesicShard.o \
    build/AetherOrchestrator.o \
    build/ProcessManager.o \
    build/VoiceShard.o \
    build/LatticePQC.o \
    build/sigma_std.o

all: build build/sigmaos_zenith

build:
	mkdir -p build

build/boot.o: kernel/boot.asm
	$(AS) -f elf64 kernel/boot.asm -o build/boot.o

build/idt_asm.o: kernel/idt.asm
	$(AS) -f elf64 kernel/idt.asm -o build/idt_asm.o

build/kmain.o: kernel/kmain.c
	$(CC) $(CFLAGS) -c kernel/kmain.c -o build/kmain.o

build/hal.o: kernel/hal.c
	$(CC) $(CFLAGS) -c kernel/hal.c -o build/hal.o

build/idt.o: kernel/idt.c
	$(CC) $(CFLAGS) -c kernel/idt.c -o build/idt.o

build/slab.o: kernel/mm/slab.c
	$(CC) $(CFLAGS) -c kernel/mm/slab.c -o build/slab.o

build/syscall.o: kernel/syscall.c
	$(CC) $(CFLAGS) -c kernel/syscall.c -o build/syscall.o

build/vfs_stub.o: kernel/vfs_stub.c
	$(CC) $(CFLAGS) -c kernel/vfs_stub.c -o build/vfs_stub.o

build/vga.o: kernel/drivers/vga.c
	$(CC) $(CFLAGS) -c kernel/drivers/vga.c -o build/vga.o

build/exceptions.o: kernel/cpu/exceptions.c
	$(CC) $(CFLAGS) -c kernel/cpu/exceptions.c -o build/exceptions.o

build/signals.o: kernel/cpu/signals.c
	$(CC) $(CFLAGS) -c kernel/cpu/signals.c -o build/signals.o

build/sigma_shell_cli.o: kernel/shell/sigma_shell_cli.c
	$(CC) $(CFLAGS) -c kernel/shell/sigma_shell_cli.c -o build/sigma_shell_cli.o

build/sigma_shell.o: kernel/shell/sigma_shell.c
	$(CC) $(CFLAGS) -c kernel/shell/sigma_shell.c -o build/sigma_shell.o

build/sigma_fs.o: kernel/fs/sigma_fs.c
	$(CC) $(CFLAGS) -c kernel/fs/sigma_fs.c -o build/sigma_fs.o

build/sigma_std.o: kernel/sigma_std.c
	$(CC) $(CFLAGS) -c kernel/sigma_std.c -o build/sigma_std.o

build/init.o: kernel/core/init.c
	$(CC) $(CFLAGS) -Ilibc -c kernel/core/init.c -o build/init.o

build/vfs.o: kernel/vfs.c
	$(CC) $(CFLAGS) -c kernel/vfs.c -o build/vfs.o

build/omni_shell.o: kernel/omni_shell.c
	$(CC) $(CFLAGS) -c kernel/omni_shell.c -o build/omni_shell.o

build/AmnesicShard.o: kernel/SovereignAmnesicShard.c
	$(CC) $(CFLAGS) -c kernel/SovereignAmnesicShard.c -o build/AmnesicShard.o

build/AetherOrchestrator.o: kernel/SovereignAetherOrchestrator.c
	$(CC) $(CFLAGS) -c kernel/SovereignAetherOrchestrator.c -o build/AetherOrchestrator.o

build/ProcessManager.o: kernel/SovereignProcessManager.c
	$(CC) $(CFLAGS) -c kernel/SovereignProcessManager.c -o build/ProcessManager.o

build/VoiceShard.o: kernel/SovereignVoiceShard.c
	$(CC) $(CFLAGS) -c kernel/SovereignVoiceShard.c -o build/VoiceShard.o

build/LatticePQC.o: kernel/shards/SovereignLatticePQC.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignLatticePQC.c -o build/LatticePQC.o

build/sigmaos_zenith: $(OBJS)
	$(LD) $(LDFLAGS) $(OBJS) -o build/sigmaos_zenith

clean:
	rm -rf build

# Σ KERNEL UNIT TESTING
test: build
	gcc -o build/sigma-test-memory tests/test_memory.c -Ikernel
	gcc -o build/sigma-test-scheduler tests/test_scheduler.c -Ikernel
	./build/sigma-test-memory
	./build/sigma-test-scheduler

# Σ PERFORMANCE BENCHMARKING
sigma-bench: build
	gcc -o build/sigma-bench tools/sigma-bench.c -Ikernel
	./build/sigma-bench
