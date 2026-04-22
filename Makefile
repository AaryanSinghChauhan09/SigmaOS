# =============================================================================
# Σ SIGMAOS: SOVEREIGN UNIVERSAL MAKEFILE (v9.0 - MODULAR LATTICE EDITION)
# =============================================================================
# Architecture: 33-Suite Sovereign Lattice
# =============================================================================

# --- TOOLCHAIN ---
CC        := gcc
CXX       := g++
NASM      := nasm
LD        := ld
OBJCOPY   := objcopy

# --- ARCHITECTURE ---
ARCH      ?= x86_64

# --- COMPILER FLAGS ---
CFLAGS    := -std=c11 -Wall -Wextra -O2 -ffreestanding -fno-stack-protector -mno-red-zone
CXXFLAGS  := -std=c++20 -Wall -Wextra -O2 -ffreestanding -fno-stack-protector -fno-exceptions -fno-rtti -mno-red-zone
ASMFLAGS  := -f elf64
LDFLAGS   := -nostdlib -static

# --- DIRECTORIES ---
SUITES_DIR := suites
BUILD_DIR  := build

# --- SOURCE DISCOVERY (Recursive suite search) ---
ALL_C_SRCS   := $(shell find $(SUITES_DIR) -name "*.c")
ALL_CPP_SRCS := $(shell find $(SUITES_DIR) -name "*.cpp")
ALL_ASM_SRCS := $(shell find $(SUITES_DIR) -name "*.asm")

# Include paths for all suites and core headers
INCLUDES     := -I. -Icore/include $(shell find $(SUITES_DIR) -type d | sed 's/^/-I/')

# --- OBJECTS ---
OBJS := $(patsubst %.c, $(BUILD_DIR)/%.o, $(notdir $(ALL_C_SRCS))) \
        $(patsubst %.cpp, $(BUILD_DIR)/%.o, $(notdir $(ALL_CPP_SRCS))) \
        $(patsubst %.asm, $(BUILD_DIR)/%.o, $(notdir $(ALL_ASM_SRCS)))

# =============================================================================
# TARGETS
# =============================================================================

.PHONY: all clean info verify

all: dirs kernel info

dirs:
	@mkdir -p $(BUILD_DIR)

kernel: $(OBJS)
	@echo "[LD] Linking Σ SIGMAOS SOVEREIGN LATTICE..."
	@$(LD) $(LDFLAGS) $(OBJS) -o $(BUILD_DIR)/sigmaos_zenith
	@echo "[OK] build/sigmaos_zenith is ready."

# Pattern rules for objects
$(BUILD_DIR)/%.o: **/%.c
	@echo "[CC]  $<"
	@$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: **/%.cpp
	@echo "[CXX] $<"
	@$(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: **/%.asm
	@echo "[ASM] $<"
	@$(NASM) $(ASMFLAGS) $< -o $@

clean:
	@rm -rf $(BUILD_DIR)
	@echo "[CLEAN] Done."

info:
	@echo ""
	@echo "Σ SigmaOS Sovereign Build v9.0"
	@echo "  Suites Indexed: 33"
	@echo "  Sources Found:  $(words $(ALL_C_SRCS) $(ALL_CPP_SRCS) $(ALL_ASM_SRCS))"
	@echo ""
