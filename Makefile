# =============================================================================
# Σ SIGMAOS: SOVEREIGN INDUSTRIAL MAKEFILE (v160.0 - PURE C11 MASTER)
# =============================================================================
# Mission: Absolute System Sovereignty. Zero-Dependency Build.
# Standard: C11 (ISO/IEC 9899:2011)
# Output: build/sigmaos_工业_zenith 
# =============================================================================

CC      := gcc
NASM    := nasm
LD      := ld
OBJCOPY := objcopy

# Industrial Compiler Flags
CFLAGS  := -std=c11 -Wall -Wextra -Wpedantic -Werror \
           -Wno-unused-parameter -O3 -ffreestanding \
           -fno-stack-protector -fno-builtin -mno-red-zone \
           -I. -Ilibc -DSIGMA_INDUSTRIAL_BUILD=1

ASMFLAGS := -f elf64
LDFLAGS  := -nostdlib -static -e main # Using main for the shell entry point in this simulation

# Directories
BUILD_DIR := build
KERNEL_DIR := kernel
LIBC_DIR := libc

# Source Discovery
KERNEL_SRCS := $(wildcard $(KERNEL_DIR)/*.c)
LIBC_SRCS   := $(wildcard $(LIBC_DIR)/*.c)
ASM_SRCS    := $(wildcard $(KERNEL_DIR)/*.asm) $(wildcard $(LIBC_DIR)/*.asm)

# Object Mapping
KERNEL_OBJS := $(patsubst $(KERNEL_DIR)/%.c, $(BUILD_DIR)/kernel_%.o, $(KERNEL_SRCS))
LIBC_OBJS   := $(patsubst $(LIBC_DIR)/%.c, $(BUILD_DIR)/libc_%.o, $(LIBC_SRCS))
ASM_OBJS    := $(patsubst %.asm, $(BUILD_DIR)/%.o, $(notdir $(ASM_SRCS)))

.PHONY: all clean dirs zenith sync

all: dirs zenith

dirs:
	@mkdir -p $(BUILD_DIR)

$(BUILD_DIR)/kernel_%.o: $(KERNEL_DIR)/%.c
	@echo "[SHARD-CC] $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/libc_%.o: $(LIBC_DIR)/%.c
	@echo "[LIBC-CC]  $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: $(KERNEL_DIR)/%.asm
	@echo "[SHARD-ASM] $<"
	@$(NASM) $(ASMFLAGS) $< -o $@

$(BUILD_DIR)/%.o: $(LIBC_DIR)/%.asm
	@echo "[LIBC-ASM]  $<"
	@$(NASM) $(ASMFLAGS) $< -o $@

zenith: $(KERNEL_OBJS) $(LIBC_OBJS) $(ASM_OBJS)
	@echo "[SHARD-LD] Linking Sovereign Zenith..."
	@$(LD) $(LDFLAGS) $^ -o $(BUILD_DIR)/sigmaos_zenith
	@echo "Σ SIGMAOS ZENITH BUILD SUCCESSFUL (PURE C11 SHARDED)"

sync:
	@echo "[SYNC] Pushing Sovereign Architecture to GitHub..."
	@git add .
	@git commit -m "Σ SIGMAOS: Finalizing Sovereign C11 Architecture - Zero Dependency Mastery"
	@git push origin main

clean:
	@rm -rf $(BUILD_DIR)
	@echo "[CLEAN] Build artifacts purged."
