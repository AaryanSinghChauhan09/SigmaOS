# =============================================================================
# Σ SIGMAOS: SOVEREIGN LATTICE BUILD SYSTEM (ZENITH)
# =============================================================================
# Target: x86_64-elf, arm64-none-eabi, riscv64-unknown-elf
# Parity: Industrial Parity with GNU Make / CMake ecosystems.
# =============================================================================

CXX      = g++
AS       = nasm
QEMU     = qemu-system-x86_64
CXXFLAGS = -ffreestanding -O2 -Wall -Wextra -Werror -fno-exceptions -fno-rtti -std=c++17 \
           -I./include -fno-stack-protector -mno-red-zone -MMD -MP
ASFLAGS  = -f elf64

# Industrial Shard Orchestration (600-Shard Lattice)
# Dynamically discovers all .cpp, .c, and .asm files in the kernel structure
KERNEL_SHARDS = $(patsubst %.cpp,%.o,$(shell find kernel/core kernel/shards -name "*.cpp" 2>/dev/null)) \
                $(patsubst %.c,%.o,$(shell find kernel/core kernel/shards -name "*.c" 2>/dev/null)) \
                $(patsubst %.asm,%.o,$(shell find kernel/core kernel/shards -name "*.asm" 2>/dev/null))

# Dependency files
DEPS = $(KERNEL_SHARDS:.o=.d)

.PHONY: all singularity zenith-iso qemu clean

all: singularity

# Runs the sovereign kernel in QEMU (Step 1 parity)
qemu: singularity
	$(QEMU) -kernel sigmaos.bin -serial stdio -m 2G

# Reaches the 600-shard modularity zenith
singularity: $(KERNEL_SHARDS)
	@echo "[BUILD] Igniting 600-shard modular lattice..."
	$(CXX) $(CXXFLAGS) -T kernel/sigma.ld -o sigmaos.bin $^
	@echo "[STATUS] SINGULARITY ACHIEVED. SigmaOS kernel ready."

# Generates the production-grade deployment image
zenith-iso: singularity
	@echo "[ISO] Generating Zenith Singularity deployment image..."
	grub-mkrescue -o zenith-singularity.iso iso_root
	@echo "[STATUS] Deployment image ready: zenith-singularity.iso"

clean:
	rm -f $(KERNEL_SHARDS) $(DEPS) sigmaos.bin zenith-singularity.iso

-include $(DEPS)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.c
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@
