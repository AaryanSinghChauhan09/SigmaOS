# =============================================================================
# Σ SIGMAOS: SOVEREIGN UNIVERSAL MAKEFILE (v10.0 - PURE SILICON EDITION)
# =============================================================================
# Architecture: 33-Suite Sovereign Lattice
# Standards: Zero-Std, Freestanding, Pure ASM/C20
# =============================================================================

# --- TOOLCHAIN ---
GCC       := C:/msys64/mingw64/bin/gcc.exe
GXX       := C:/msys64/mingw64/bin/g++.exe
NASM      := nasm
LD        := ld
OBJCOPY   := objcopy

# --- COMPILER FLAGS (Sovereign Hardening) ---
# We eliminate all high-level dependencies and standard libraries.
COMMON_FLAGS := -ffreestanding -nostdlib -fno-stack-protector -mno-red-zone -O2 -Wall -Wextra
CFLAGS       := -std=c11 $(COMMON_FLAGS)
CXXFLAGS     := -std=c++20 -fno-exceptions -fno-rtti $(COMMON_FLAGS)
ASMFLAGS     := -f elf64
LDFLAGS      := -nostdlib -static -T suites/S01_Genesis/shards/sigma.ld

# --- DIRECTORIES ---
SUITES_DIR := suites
BUILD_DIR  := build

# --- SOURCE DISCOVERY (PowerShell powered for Windows compatibility) ---
# Using powershell to find all sources recursively (Suites + Core + CLI)
ALL_C_SRCS   := $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Filter *.c -Recurse -ErrorAction SilentlyContinue | ForEach-Object { \$$_.FullName }")
ALL_CPP_SRCS := $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Filter *.cpp -Recurse -ErrorAction SilentlyContinue | ForEach-Object { \$$_.FullName }")
ALL_ASM_SRCS := $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Filter *.asm -Recurse -ErrorAction SilentlyContinue | ForEach-Object { \$$_.FullName }")

# Include paths for all suites and core headers
INCLUDES     := -I. -Iinclude $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Directory -Recurse -ErrorAction SilentlyContinue | ForEach-Object { '-I' + \$$_.FullName }")

# --- OBJECTS ---
# We map filenames to the build directory to avoid deep path issues on Windows
OBJS := $(patsubst %.c, $(BUILD_DIR)/%.o, $(notdir $(ALL_C_SRCS))) \
        $(patsubst %.cpp, $(BUILD_DIR)/%.o, $(notdir $(ALL_CPP_SRCS))) \
        $(patsubst %.asm, $(BUILD_DIR)/%.o, $(notdir $(ALL_ASM_SRCS)))

# =============================================================================
# TARGETS
# =============================================================================

.PHONY: all clean info verify

all: dirs kernel info

dirs:
	@powershell -Command "if (!(Test-Path $(BUILD_DIR))) { New-Item -ItemType Directory -Path $(BUILD_DIR) }"

kernel: $(OBJS)
	@echo "[LD] Linking Σ SIGMAOS SOVEREIGN LATTICE (Pure Silicon)..."
	@$(LD) $(LDFLAGS) $(OBJS) -o $(BUILD_DIR)/sigmaos_zenith
	@echo "[OK] build/sigmaos_zenith is ready."

# Pattern rules for objects
# Note: These rules assume flat build dir. VPATH is needed if we use notdir above.
vpath %.c $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Directory -Recurse -ErrorAction SilentlyContinue | ForEach-Object { \$$_.FullName }")
vpath %.cpp $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Directory -Recurse -ErrorAction SilentlyContinue | ForEach-Object { \$$_.FullName }")
vpath %.asm $(shell powershell -Command "Get-ChildItem -Path $(SUITES_DIR), core, cli, userland -Directory -Recurse -ErrorAction SilentlyContinue | ForEach-Object { \$$_.FullName }")

$(BUILD_DIR)/%.o: %.c
	@echo "[CC]  $<"
	@$(GCC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: %.cpp
	@echo "[CXX] $<"
	@$(GXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: %.asm
	@echo "[ASM] $<"
	@$(NASM) $(ASMFLAGS) $< -o $@

clean:
	@powershell -Command "if (Test-Path $(BUILD_DIR)) { Remove-Item -Recurse -Force $(BUILD_DIR) }"
	@echo "[CLEAN] Done."

info:
	@echo ""
	@echo "Σ SigmaOS Sovereign Build v10.0 (Pure Silicon)"
	@echo "  Suites Indexed: 33"
	@echo "  Sources Found:  $(words $(ALL_C_SRCS) $(ALL_CPP_SRCS) $(ALL_ASM_SRCS))"
	@echo "  Dependency State: Zero-Std / Freestanding"
	@echo ""
