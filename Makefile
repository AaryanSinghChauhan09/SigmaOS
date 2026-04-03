# =============================================================================
# Σ SIGMAOS: SOVEREIGN INDUSTRIAL MAKEFILE (v170.0 - PURE C11 MASTER)
# =============================================================================
# Mission: Absolute System Sovereignty. Zero-Dependency Build.
# Standard: C11 (ISO/IEC 9899:2011)
# Output: build/sigmaos_zenith
# New Shards: sigma_distro_absorber, sigma_tool_absorber, sigma_linux_usps
# =============================================================================

CC      := gcc
NASM    := nasm
LD      := ld
OBJCOPY := objcopy

# Industrial Compiler Flags (Linux)
CFLAGS  := -std=c11 -Wall -Wextra -Wpedantic -Werror \
           -Wno-unused-parameter -O3 -ffreestanding \
           -fno-stack-protector -fno-builtin -mno-red-zone \
           -I. -Ilibc -DSIGMA_INDUSTRIAL_BUILD=1

# Windows-compatible Compiler Flags (relaxed for Win32 toolchain)
CFLAGS_WIN := -std=c11 -Wall -O3 -I. -Ilibc -DSIGMA_INDUSTRIAL_BUILD=1 -DSIGMA_WIN32=1

ASMFLAGS := -f elf64
LDFLAGS  := -nostdlib -static -e main

# Directories
BUILD_DIR      := build
KERNEL_DIR     := kernel
LIBC_DIR       := libc
TOOLS_DIR      := sovereign_tools

# Source Discovery
KERNEL_SRCS    := $(wildcard $(KERNEL_DIR)/*.c)
LIBC_SRCS      := $(wildcard $(LIBC_DIR)/*.c)
ASM_SRCS       := $(wildcard $(KERNEL_DIR)/*.asm) $(wildcard $(LIBC_DIR)/*.asm)

# NEW: Sovereign Tool Shards
TOOL_SRCS      := $(TOOLS_DIR)/SovereignOmniCLI.c \
                  $(TOOLS_DIR)/SigmaCLI_Dispatcher.c \
                  $(TOOLS_DIR)/sigma_distro_absorber.c \
                  $(TOOLS_DIR)/sigma_tool_absorber.c \
                  $(TOOLS_DIR)/sigma_linux_usps.c

# NEW: Agent Userland Tools
USERLAND_DIR   := userland
AGENT_SRCS     := $(KERNEL_DIR)/SovereignOmniAgent.c $(USERLAND_DIR)/OmniCLI.c

# Object Mapping
KERNEL_OBJS    := $(patsubst $(KERNEL_DIR)/%.c, $(BUILD_DIR)/kernel_%.o, $(KERNEL_SRCS))
LIBC_OBJS      := $(patsubst $(LIBC_DIR)/%.c, $(BUILD_DIR)/libc_%.o, $(LIBC_SRCS))
ASM_OBJS       := $(patsubst %.asm, $(BUILD_DIR)/%.o, $(notdir $(ASM_SRCS)))
AGENT_OBJS     := $(patsubst $(KERNEL_DIR)/%.c, $(BUILD_DIR)/kernel_%.o, $(KERNEL_DIR)/SovereignOmniAgent.c) \
                  $(patsubst $(KERNEL_DIR)/%.c, $(BUILD_DIR)/kernel_%.o, $(KERNEL_DIR)/SovereignNetData.c) \
                  $(patsubst $(KERNEL_DIR)/%.c, $(BUILD_DIR)/kernel_%.o, $(KERNEL_DIR)/SovereignOrchestrator.c) \
                  $(patsubst $(KERNEL_DIR)/%.c, $(BUILD_DIR)/kernel_%.o, $(KERNEL_DIR)/SovereignMCP.c) \
                  $(patsubst $(USERLAND_DIR)/%.c, $(BUILD_DIR)/userland_%.o, $(USERLAND_DIR)/OmniCLI.c)

.PHONY: all clean dirs zenith sync win32 shards absorb-check

all: dirs zenith

dirs:
	@mkdir -p $(BUILD_DIR)

$(BUILD_DIR)/kernel_%.o: $(KERNEL_DIR)/%.c
	@echo "[SHARD-CC] $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/libc_%.o: $(LIBC_DIR)/%.c
	@echo "[LIBC-CC]  $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/userland_%.o: $(USERLAND_DIR)/%.c
	@echo "[USER-CC]  $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: $(KERNEL_DIR)/%.asm
	@echo "[SHARD-ASM] $<"
	@$(NASM) $(ASMFLAGS) $< -o $@

$(BUILD_DIR)/%.o: $(LIBC_DIR)/%.asm
	@echo "[LIBC-ASM]  $<"
	@$(NASM) $(ASMFLAGS) $< -o $@

zenith: $(KERNEL_OBJS) $(LIBC_OBJS) $(ASM_OBJS) $(AGENT_OBJS)
	@echo "[SHARD-LD] Linking Sovereign Zenith..."
	@$(LD) $(LDFLAGS) $^ -o $(BUILD_DIR)/sigmaos_zenith
	@echo "Σ SIGMAOS ZENITH BUILD SUCCESSFUL (PURE C11 SHARDED + OMNI-AGENT)"

sync:
	@echo "[SYNC] Pushing Sovereign Architecture to GitHub..."
	@git add .
	@git commit -m "Σ SIGMAOS: Finalizing Sovereign C11 Architecture - Zero Dependency Mastery"
	@git push origin main

clean:
	@rm -rf $(BUILD_DIR)
	@echo "[CLEAN] Build artifacts purged."
