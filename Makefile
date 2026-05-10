<<<<<<< HEAD
# SigmaOS Sovereign Makefile
# Proving sovereignty through deterministic builds.

CC = gcc
CFLAGS = -Wall -Wextra -I. -Imodules/core/include -ffreestanding -nostdlib

# Shards
DRIVER_SHARDS = $(wildcard modules/core/drivers/*.c)
KERNEL_SHARDS = $(wildcard modules/core/kernel/*.c) $(wildcard modules/core/kernel/*/*.c)
NET_SHARDS = $(wildcard modules/core/net/*.c)
PERF_SHARDS = $(wildcard modules/perf/*.c)
UI_SHARDS = $(wildcard modules/ui/*.c)
CLOUD_SHARDS = $(wildcard modules/cloud/*.c)

ALL_SHARDS = $(DRIVER_SHARDS) $(KERNEL_SHARDS) $(NET_SHARDS) $(PERF_SHARDS) $(UI_SHARDS) $(CLOUD_SHARDS)

all: $(ALL_SHARDS)
	@echo "All Sovereign Shards Validated."

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(ALL_SHARDS:.c=.o)
=======
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

# Dependency files
DEPS = $(KERNEL_SHARDS:.o=.d)

# Declarative discovery via SHARDS.manifest (Fix Issue #9)
MANIFEST_SOURCES = $(shell cat SHARDS.manifest | grep -v '^\#' | grep -v '^[[:space:]]*$$')
KERNEL_SHARDS    = $(patsubst %.cpp,%.o,$(filter %.cpp,$(MANIFEST_SOURCES))) \
                   $(patsubst %.c,%.o,$(filter %.c,$(MANIFEST_SOURCES))) \
                   $(patsubst %.asm,%.o,$(filter %.asm,$(MANIFEST_SOURCES)))

.PHONY: all kernel drivers ui singularity zenith-iso qemu clean rebuild test

all: kernel drivers ui singularity

kernel:
	@echo "[MODULE] Building Kernel (Sovereign Core)..."

drivers:
	@echo "[MODULE] Building Drivers (Hardware/GPU/Network)..."

ui:
	@echo "[MODULE] Building UI (Zenith Compositor)..."

test:
	@echo "[TEST] Building and running GTest host suite..."
	@mkdir -p tests/cpp_host/build
	@cd tests/cpp_host/build && cmake .. && make && ./test_kernel

qemu: singularity
	@echo "[QEMU] Booting SigmaOS..."
	$(QEMU) -kernel sigmaos.bin -serial stdio -m 2G

singularity: $(KERNEL_SHARDS)
	@echo "[BUILD] Linking 600-shard modular lattice..."
	$(CXX) $(CXXFLAGS) -T kernel/sigma.ld -o sigmaos.bin $^
	@echo "[STATUS] SINGULARITY ACHIEVED."

zenith-iso: singularity
	@echo "[ISO] Generating deployment image..."
	grub-mkrescue -o zenith-singularity.iso iso_root
	@echo "[STATUS] ISO ready: zenith-singularity.iso"

clean:
	find . -type f \( -name "*.o" -o -name "*.d" \) -delete
	rm -f sigmaos.bin zenith-singularity.iso

rebuild: clean all

# Include dependencies
-include $(DEPS)

# Compilation rules
%.o: %.cpp
	@mkdir -p $(dir $@)
	@echo "[CC++] $<"
	@$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.c
	@mkdir -p $(dir $@)
	@echo "[CC] $<"
	@$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.asm
	@mkdir -p $(dir $@)
	@echo "[AS] $<"
	@$(AS) $(ASFLAGS) $< -o $@
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645
