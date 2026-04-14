# =========================================================================
# Σ SIGMAOS ZENITH: MASTER BUILD SYSTEM (v3250.0 — Phase 60)
# =========================================================================
# Target:        sigma_zenith.bin — Sovereign Zenith Supreme
# Architecture:  x86_64 (bare-metal, QEMU, Cloud VMs, USB flash)
# Compiler:      GCC 13+ / Clang 17+ (C11, -ffreestanding)
# Standard:      Zero HLL Dependency — Pure C11 / ASM Sovereign Shards
# =========================================================================

CC      = gcc
AS      = nasm
LD      = ld
OBJCOPY = objcopy

# Default target
all: help

# Sovereign build help
help:
	@echo "Σ SIGMAOS MASTER BUILD SYSTEM"
	@echo "=============================="
	@echo "make all         : Show this help"
	@echo "make bin         : Build the sovereign kernel binary"
	@echo "make iso         : Build the bootable QEMU ISO"
	@echo "make lint        : Run static analysis"
	@echo "make shard-list  : Show sovereign shard inventory"
	@echo "make test        : Run the high-fidelity audit pipeline"
	@echo "make clean       : Purge all build artifacts"

# ---------------------------------------------------------------------------
# Compiler flags
# ---------------------------------------------------------------------------
CFLAGS  = -std=c11                  \
           -m64                      \
           -ffreestanding            \
           -O2                       \
           -Wall -Wextra             \
           -Wno-unused-parameter     \
           -fno-stack-protector      \
           -fno-pic                  \
           -nostdlib                 \
           -I./include               \
           -I./kernel/modules/core   \
           -I./kernel/core           \
           -I./kernel

ASFLAGS = -f elf64
LDFLAGS = -T kernel/sigma.ld -m elf_x86_64 -nostdlib

# ---------------------------------------------------------------------------
# === INTEGRATED SOVEREIGN SUITES (Industrial Tier - v2) ===
# ---------------------------------------------------------------------------

SUITE_ROOT := kernel/suites

C_SOURCES := $(shell find $(SUITE_ROOT) -name '*.c' 2>/dev/null)
ASM_SOURCES := $(shell find $(SUITE_ROOT) -name '*.asm' 2>/dev/null)

# Fallback specifically for core files if not already in suites
C_SOURCES += $(shell find kernel/core -name '*.c' 2>/dev/null)
ASM_SOURCES += $(shell find kernel -name 'boot.asm' 2>/dev/null)

# Aggregate all unique sources
C_SOURCES := $(sort $(C_SOURCES))
ASM_SOURCES := $(sort $(ASM_SOURCES))

SHARDS := $(C_SOURCES:.c=.o) $(ASM_SOURCES:.asm=.o)

# ---------------------------------------------------------------------------
# Build targets
# ---------------------------------------------------------------------------

.PHONY: all clean iso test shard-list lint check

all: sigma_zenith.bin
	@echo "Σ [BUILD]: sigma_zenith.bin ready — $(words $(C_SOURCES)) C11 shards compiled."

sigma_zenith.bin: kernel/boot.o $(SHARDS)
	$(LD) $(LDFLAGS) -o $@ kernel/boot.o $(SHARDS)

%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

%.o: %.asm
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

# ---------------------------------------------------------------------------
# Utility targets
# ---------------------------------------------------------------------------

clean:
	rm -rf kernel/boot.o $(SHARDS) sigma_zenith.bin iso/
	@echo "Σ [CLEAN]: All objects purged."

# Produce bootable ISO via GRUB
iso: sigma_zenith.bin
	mkdir -p iso/boot/grub
	cp sigma_zenith.bin iso/boot/
	@printf 'set timeout=0\nset default=0\nmenuentry "SigmaOS Zenith Supreme (v3250.4)" {\n  multiboot /boot/sigma_zenith.bin\n  boot\n}\n' > iso/boot/grub/grub.cfg
	grub-mkrescue -o SigmaOS_Zenith_v3250.iso iso/
	@echo "Σ [ISO]: SigmaOS_Zenith_v3250.iso created."

# Static analysis (cppcheck if available)
lint:
	@command -v cppcheck >/dev/null 2>&1 && \
	  cppcheck --enable=all --std=c11 -I./include $(C_SOURCES) || \
	  echo "Σ [LINT]: cppcheck not found — install for static analysis."

# Sovereign shard inventory (per-suite breakdown)
shard-list:
	@echo "Σ [MANIFEST]: Sovereign Shard Inventory"
	@echo "==========================================="
	@echo "S01 Genesis:       $$(find $(SUITE_ROOT)/S01_Genesis -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S02 ZenithUI:      $$(find $(SUITE_ROOT)/S02_ZenithUI -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S03 Distros:       $$(find $(SUITE_ROOT)/S03_Distros -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S04 HAL:           $$(find $(SUITE_ROOT)/S04_HAL -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S05 Memory:        $$(find $(SUITE_ROOT)/S05_Memory -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S06 Storage:       $$(find $(SUITE_ROOT)/S06_Storage -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S07 Network:       $$(find $(SUITE_ROOT)/S07_Network -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S08 Security:      $$(find $(SUITE_ROOT)/S08_Security -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S09 Tooling:       $$(find $(SUITE_ROOT)/S09_Tooling -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "S10 Orchestration: $$(find $(SUITE_ROOT)/S10_Orchestration -name '*.c' 2>/dev/null | wc -l) shards"
	@echo "-------------------------------------------"
	@echo "Total C Sources: $(words $(C_SOURCES))"
	@echo "ASM Sources:     $(words $(ASM_SOURCES))"

# Sovereign resilience audit (High-Fidelity)
test:
	gcc -std=c11 -O2 -I ./include tools/sovereign_test/sovereign_test_runner.c -o sigma-test && ./sigma-test


