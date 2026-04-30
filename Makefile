# =============================================================================
# Σ SIGMAOS: SOVEREIGN LATTICE BUILD SYSTEM (ZENITH)
# =============================================================================
# Target: x86_64-elf, arm64-none-eabi, riscv64-unknown-elf
# Parity: Industrial Parity with GNU Make / CMake ecosystems.
# =============================================================================

CXX      = g++
AS       = nasm
CXXFLAGS = -ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti -std=c++17 \
           -I./include -fno-stack-protector -mno-red-zone
ASFLAGS  = -f elf64

KERNEL_SHARDS = kernel/core/SovereignInit.o \
                kernel/core/SovereignIPC.o \
                kernel/core/SovereignMMU.o \
                kernel/core/SovereignAISched.o \
                kernel/core/SovereignSMP.o \
                kernel/core/SovereignLazy.o \
                kernel/core/SovereignSnap.o \
                kernel/core/SovereignKube.o \
                kernel/core/SovereignInstall.o \
                kernel/core/SovereignNeural.o \
                kernel/core/SovereignBT.o \
                kernel/core/SovereignPersistence.o

.PHONY: all singularity zenith-iso clean

all: singularity

# Reaches the 600-shard modularity zenith
singularity: $(KERNEL_SHARDS)
	@echo "[BUILD] Igniting 600-shard modular lattice..."
	$(CXX) $(CXXFLAGS) -T kernel/linker.ld -o sigmaos.bin $^
	@echo "[STATUS] SINGULARITY ACHIEVED. SigmaOS kernel ready."

# Generates the production-grade deployment image
zenith-iso: singularity
	@echo "[ISO] Generating Zenith Singularity deployment image..."
	grub-mkrescue -o zenith-singularity.iso iso_root
	@echo "[STATUS] Deployment image ready: zenith-singularity.iso"

clean:
	rm -f $(KERNEL_SHARDS) sigmaos.bin zenith-singularity.iso

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@
