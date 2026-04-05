# Σ SIGMAOS ZENITH: SOVEREIGN MAKEFILE (v2950.0)
# Mission: Absolute Build Purity & Hardware-Direct Compilation.

CC = x86_64-elf-gcc
AS = nasm
LD = x86_64-elf-ld

# -I. ensures 'libc/SovereignLibC.h' and 'SovereignOmniShard.h' resolve from any subdir
# -Ilibc ensures '#include "SovereignLibC.h"' also resolves directly
CFLAGS  = -m64 -ffreestanding -fno-stack-protector -fno-builtin -nostdlib -I. -Ikernel -Ilibc
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
    build/SovereignDataScience.o \
    build/SovereignGaming.o \
    build/SovereignCyber.o \
    build/SovereignFintech.o \
    build/SovereignBio.o \
    build/SovereignWatchdog.o \
    build/SovereignMoE.o \
    build/SovereignEBPF.o \
    build/SovereignVectorDB.o \
    build/SovereignNUMA.o \
    build/SovereignContextBrain.o \
    build/SovereignIntelliViz.o \
    build/SovereignSecureWorkspace.o \
    build/SovereignPredictiveEngine.o \
    build/SovereignFederatedLearning.o \
    build/SovereignZeroTrust.o \
    build/SovereignNeuroSymbolic.o \
    build/SovereignGraphNet.o \
    build/SovereignAdaptiveZeroTrust.o \
    build/SovereignAdversarialDefense.o \
    build/SovereignDataPipeline.o \
    build/SovereignModelForge.o \
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

build/SovereignDataScience.o: kernel/SovereignDataScience.c
	$(CC) $(CFLAGS) -c kernel/SovereignDataScience.c -o build/SovereignDataScience.o

build/SovereignGaming.o: kernel/shards/SovereignGaming.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignGaming.c -o build/SovereignGaming.o

build/SovereignCyber.o: kernel/shards/SovereignCyber.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignCyber.c -o build/SovereignCyber.o

build/SovereignFintech.o: kernel/shards/SovereignFintech.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignFintech.c -o build/SovereignFintech.o

build/SovereignBio.o: kernel/shards/SovereignBio.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignBio.c -o build/SovereignBio.o

build/SovereignWatchdog.o: kernel/shards/SovereignWatchdog.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignWatchdog.c -o build/SovereignWatchdog.o

build/SovereignMoE.o: kernel/shards/SovereignMoE.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignMoE.c -o build/SovereignMoE.o

build/SovereignEBPF.o: kernel/shards/SovereignEBPF.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignEBPF.c -o build/SovereignEBPF.o

build/SovereignVectorDB.o: kernel/shards/SovereignVectorDB.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignVectorDB.c -o build/SovereignVectorDB.o

build/SovereignNUMA.o: kernel/shards/SovereignNUMA.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignNUMA.c -o build/SovereignNUMA.o

build/SovereignContextBrain.o: kernel/shards/SovereignContextBrain.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignContextBrain.c -o build/SovereignContextBrain.o

build/SovereignIntelliViz.o: kernel/shards/SovereignIntelliViz.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignIntelliViz.c -o build/SovereignIntelliViz.o

build/SovereignSecureWorkspace.o: kernel/shards/SovereignSecureWorkspace.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignSecureWorkspace.c -o build/SovereignSecureWorkspace.o

build/SovereignPredictiveEngine.o: kernel/shards/SovereignPredictiveEngine.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignPredictiveEngine.c -o build/SovereignPredictiveEngine.o

build/SovereignFederatedLearning.o: kernel/shards/SovereignFederatedLearning.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignFederatedLearning.c -o build/SovereignFederatedLearning.o

build/SovereignZeroTrust.o: kernel/shards/SovereignZeroTrust.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignZeroTrust.c -o build/SovereignZeroTrust.o

build/SovereignNeuroSymbolic.o: kernel/shards/SovereignNeuroSymbolic.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignNeuroSymbolic.c -o build/SovereignNeuroSymbolic.o

build/SovereignGraphNet.o: kernel/shards/SovereignGraphNet.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignGraphNet.c -o build/SovereignGraphNet.o

build/SovereignAdaptiveZeroTrust.o: kernel/shards/SovereignAdaptiveZeroTrust.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignAdaptiveZeroTrust.c -o build/SovereignAdaptiveZeroTrust.o

build/SovereignAdversarialDefense.o: kernel/shards/SovereignAdversarialDefense.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignAdversarialDefense.c -o build/SovereignAdversarialDefense.o

build/SovereignDataPipeline.o: kernel/shards/SovereignDataPipeline.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignDataPipeline.c -o build/SovereignDataPipeline.o

build/SovereignModelForge.o: kernel/shards/SovereignModelForge.c
	$(CC) $(CFLAGS) -c kernel/shards/SovereignModelForge.c -o build/SovereignModelForge.o

build/sigmaos_zenith: $(OBJS)
	$(LD) $(LDFLAGS) $(OBJS) -o build/sigmaos_zenith

clean:
	rm -rf build

# Σ KERNEL UNIT TESTING (host-side — native gcc, no cross-compiler needed)
test:
	@echo 'Σ [BUILD]: Compiling memory test suite...'
	gcc -std=c11 -O2 -Wall -Wextra -o build/sigma-test-memory tests/test_memory.c
	@echo 'Σ [BUILD]: Compiling scheduler test suite...'
	gcc -std=c11 -O2 -Wall -Wextra -o build/sigma-test-scheduler tests/test_scheduler.c
	@echo 'Σ [RUN]: Memory Tests...'
	./build/sigma-test-memory
	@echo 'Σ [RUN]: Scheduler Tests...'
	./build/sigma-test-scheduler
	@echo 'Σ [PASS]: All sovereign tests PASSED.'

# Σ PERFORMANCE BENCHMARKING
sigma-bench: build
	gcc -std=c11 -O3 -march=native -o build/sigma-bench tools/sigma-bench.c
	./build/sigma-bench
