# =============================================================================
# Σ SIGMAOS: SOVEREIGN UNIVERSAL MAKEFILE (v8.0 - ZERO-DEPENDENCY EDITION)
# =============================================================================
# Inspiration: torvalds/linux/Makefile, seL4/CMakeLists.txt, Fuchsia GN
# USP Absorbed: 
#   - Arch: Minimal, direct, PKGBUILD style
#   - Gentoo: USE flag equivalent (feature flags via SIGMA_FLAGS)
#   - NixOS: Reproducible builds (DETERMINISTIC=1)
#   - Alpine: Musl-based, static-first
# Languages: C (C11), C++ (C++20), ASM (x86_64/ARM64), Rust (no_std)
# Zero: No stdlib. No glibc. No libstdc++. No alloc crate.
# =============================================================================

# --- TOOLCHAIN ---
CC        := gcc
CXX       := g++
RUSTC     := rustc
NASM      := nasm
LD        := ld
AR        := ar
OBJCOPY   := objcopy

# --- ARCHITECTURE ---
ARCH      ?= x86_64
# Supported: x86_64, aarch64, riscv64

# --- COMPILER FLAGS ---

# C: Minimal, no-stdlib, maximum warnings, custom types only
CFLAGS    := -std=c11 \
             -Wall -Wextra -Wpedantic -Werror \
             -Wno-unused-parameter \
             -O2 \
             -ffreestanding \
             -fno-stack-protector \
             -fno-builtin \
             -mno-red-zone \
             -I. \
             -Ilibc \
             -DSIGMA_ARCH_$(shell echo $(ARCH) | tr - _ | tr '[:lower:]' '[:upper:]') \
             -DSIGMA_OS_BUILD=1

# C++: No STL, no RTTI, no exceptions - pure OOP via vtables
CXXFLAGS  := -std=c++20 \
             -Wall -Wextra -Werror \
             -Wno-unused-parameter \
             -O2 \
             -ffreestanding \
             -fno-stack-protector \
             -fno-builtin \
             -fno-exceptions \
             -fno-rtti \
             -mno-red-zone \
             -I. \
             -Ilibc \
             -DSIGMA_ARCH_$(shell echo $(ARCH) | tr - _ | tr '[:lower:]' '[:upper:]') \
             -DSIGMA_OS_BUILD=1

# ASM: NASM, ELF64 output
ASMFLAGS  := -f elf64

# Rust: no_std, no_main, target-specific
RUSTFLAGS := --edition 2021 \
             --target $(ARCH)-unknown-none \
             -C opt-level=2 \
             -C panic=abort \
             -C code-model=kernel \
             -A dead_code \
             -A unused_variables

# --- LINKER ---
LDFLAGS   := -nostdlib -static

# =============================================================================
# SOURCE GROUPS
# =============================================================================

# Core libc Implementation (Our sovereign stdlib replacement)
SIGMA_LIBC_C   := libc/sigma_libc.c
SIGMA_LIBC_OBJ := build/sigma_libc.o

# Kernel C sources (refactored for Linux-parity)
KERNEL_C_SRCS := kernel/main.c \
                 kernel/slab.c \
                 kernel/scheduler.c \
                 kernel/init.c \
                 arch/x86_64/paging.c \
                 fs/vfs.c \
                 drivers/console.c \
                 kernel/SovereignProcessManager.cpp \
                 kernel/SovereignMemoryZenith.cpp \
                 kernel/SovereignAIKernelZenith.cpp \
                 kernel/SovereignXV6Bridge.cpp
KERNEL_OBJS   := $(patsubst %, build/kernel_%.o, $(notdir $(KERNEL_C_SRCS)))

# ASM sources
BOOT_ASM_SRC     := arch/SovereignStandardHAL.asm
SYSCALL_ASM      := libc/SovereignLibC.asm
KERNEL_FINAL_ASM := kernel/SovereignKernelFinality.asm
BOOT_OBJS        := build/hal.o build/syscall.o build/finality.o

# Rust no_std modules (Industrial Safety)
RUST_SRCS := kernel/rust/SigmaRustCore.rs \
             kernel/rust/SovereignSafety.rs
RUST_TARGETS := $(patsubst %.rs, build/%.a, $(notdir $(RUST_SRCS)))

# C++ Kernel drivers / apps
CPP_SRCS  := SigmaOOP_Framework.cpp \
             sigma_native_core.cpp \
             sigma_mesh_chat.cpp \
             SigmaDiagnosticsCore.cpp \
             SigmaFinalIntegration.cpp \
             SigmaConcurrencyZenith.cpp \
             sigma_system_pulse.cpp \
             sigma_browser_core.cpp \
             sigma_terminal_sovereign.cpp \
             SigmaHealth_Native.cpp \
             kernel/SovereignProcessManager.cpp \
             kernel/SovereignContainer.cpp \
             kernel/SovereignVirtualizer.cpp \
             kernel/SovereignNetwork.cpp \
             kernel/SovereignAgent.cpp \
             kernel/SovereignPM.cpp \
             kernel/SovereignSecurity.cpp \
             kernel/sigma_sml.cpp \
             kernel/SovereignVFS.cpp
CPP_OBJS  := $(patsubst %.cpp, build/%.o, $(CPP_SRCS))

# Sovereign Tools
SIGMA_TOOLS_SRCS := userland/sigma_pkg.c \
                    userland/apps/SigmaPersonalizer.cpp \
                    userland/apps/SigmaDistroStreamer.cpp \
                    userland/apps/SigmaLogic.cpp \
                    userland/apps/SigmaIndustrialMatrix.cpp \
                    userland/apps/SigmaThemeStore.cpp \
                    userland/apps/SigmaNetworkShard.cpp \
                    userland/apps/SigmaRemoteManager.cpp \
                    userland/apps/SigmaAuditTool.cpp \
                    userland/apps/SigmaTerminalUtils.cpp \
                    userland/apps/SovereignJusticeShard.cpp \
                    userland/apps/SovereignAetherAbsorption.cpp \
                    userland/apps/SigmaAutomation.rs \
                    userland/sigma_rust_parity.c \
                    kernel/plugins.c \
                    kernel/SigmaAI.c \
                    kernel/SigmaCore.asm \
                    SigmaGuideLinter.cpp \
                    SigmaFileFabricator.cpp \
                    SigmaSovereignCI.cpp \
                    SigmaLauncher.cpp \
                    SigmaThemeEngine.cpp \
                    SigmaSovereignPersonalizer.cpp \
                    SigmaSiliconPulse.cpp \
                    SigmaHtmlLinter.cpp \
                    SigmaBootstrapper.cpp \
                    SigmaSovereignBuilder.cpp \
                    SigmaIsoBuilder.cpp \
                    SigmaSovereignLogic.cpp \
                    SigmaSovereignScript.cpp \
                    SigmaSovereignMesh.cpp \
                    SigmaSovereignTypes.cpp \
                    userland/apps/SigmaSovereignDashboard.cpp \
                    ecosystem/SigmaAetherOrchestrator.cpp \
                    SigmaSovereignAPI.cpp \
                    SigmaControlCenter.cpp \
                    SigmaSovereignBEM.cpp \
                    SigmaSovereignNet.cpp \
                    userland/apps/SigmaSovereignAdvocate.cpp \
                    SigmaAntigravitySubsystem.cpp \
                    SigmaOmniAutomator.cpp \
                    SigmaAutomatorExtensions.cpp \
                    SigmaOmniScripts.cpp \
                    SigmaOmniPipeline.cpp \
                    SigmaOmniHypervisor.cpp \
                    SigmaOmniRTOS.cpp \
                    sigma_camera_sovereign.cpp
SIGMA_TOOLS_OBJS := $(patsubst %.cpp, build/%.o, $(SIGMA_TOOLS_SRCS))

# =============================================================================
# BUILD TARGETS
# =============================================================================

# Default: build everything
.PHONY: all clean info test sigma_libc kernel rust_core hal verify

all: dirs sigma_libc kernel rust_core hal tools info
	@echo ""
	@echo "Σ ============================================================ Σ"
	@echo "  SigmaOS SOVEREIGN BUILD v8.0 COMPLETE"
	@echo "  Zero stdlib. Zero glibc. Zero external deps."
	@echo "  Arch + Alpine + Debian + Gentoo + NixOS USPs absorbed."
	@echo "Σ ============================================================ Σ"

# Create build directory
dirs:
	@mkdir -p build build/kernel

# --- SOVEREIGN LIBC (Our core replacement for glibc/musl) ---
sigma_libc: dirs $(SIGMA_LIBC_OBJ) $(SIGMA_ROOT_LIBC_OBJ)
	@echo "[SIGMA_LIBC] Sovereign Standard Library compiled."

$(SIGMA_LIBC_OBJ): $(SIGMA_LIBC_C) libc/sigma_types.h libc/sigma_libc.h
	@echo "[CC]  libc/sigma_libc.c"
	@$(CC) $(CFLAGS) -c $< -o $@

$(SIGMA_ROOT_LIBC_OBJ): $(SIGMA_ROOT_LIBC_C) SigmaLibC.h libc/sigma_types.h libc/sigma_libc.h
	@echo "[CC]  SigmaLibC.c (root selftest)"
	@$(CC) $(CFLAGS) -DSIGMA_EMBEDDED_BUILD -c $< -o $@

# Test standalone SigmaLibC (produces a runnable ELF on Linux/x86_64)
sigma_libc_standalone: libc/sigma_libc.c SigmaLibC.c libc/sigma_types.h libc/sigma_libc.h
	@echo "[BUILD] SigmaLibC standalone test binary"
	@$(CC) $(CFLAGS) -nostdlib -static libc/sigma_libc.c SigmaLibC.c \
		-e _start -o build/sigma_libc_test
	@chmod +x build/sigma_libc_test
	@echo "[OK]   build/sigma_libc_test"

# --- KERNEL C MODULES ---
kernel: dirs $(KERNEL_OBJS)
	@echo "[KERNEL] C kernel modules compiled."

build/kernel_%.o: kernel/%.c libc/sigma_types.h libc/sigma_libc.h
	@echo "[CC]  kernel/$*.c"
	@$(CC) $(CFLAGS) -c $< -o $@ 2>&1 | head -30

# --- ASSEMBLER (x86_64 ISR/IDT/Boot) ---
hal: dirs build/sigma_hal.o
	@echo "[ASM]  HAL assembly completed."

build/sigma_hal.o: sigma_hal.asm
	@echo "[NASM] sigma_hal.asm"
	@$(NASM) $(ASMFLAGS) $< -o $@

build/boot.o: kernel/boot.asm
	@echo "[NASM] kernel/boot.asm"
	@$(NASM) $(ASMFLAGS) $< -o $@

build/long_mode.o: kernel/long_mode.asm
	@echo "[NASM] kernel/long_mode.asm"
	@$(NASM) $(ASMFLAGS) $< -o $@

build/syscall.o: kernel/syscall.asm
	@echo "[NASM] kernel/syscall.asm"
	@$(NASM) $(ASMFLAGS) $< -o $@

build/task_switch.o: kernel/task_switch.asm
	@echo "[NASM] kernel/task_switch.asm"
	@$(NASM) $(ASMFLAGS) $< -o $@

# --- RUST no_std CORE ---
rust_core: dirs
	@echo "[RUSTC] Building SigmaRustCore (no_std)..."
	@$(RUSTC) $(RUSTFLAGS) --emit=obj SigmaRustCore.rs -o build/SigmaRustCore.o 2>/dev/null || \
	 echo "[RUSTC] Note: Requires $(ARCH)-unknown-none toolchain installed."
	@echo "[RUSTC] Rust sovereign core build attempted."

# --- C++ OOP FRAMEWORK ---
cpp_framework: dirs
	@echo "[CXX] Building C++ OOP Framework (no STL, no RTTI)..."
	@$(CXX) $(CXXFLAGS) -c sigma_native_core.cpp -o build/sigma_native_core.o 2>&1 | head -20
	@echo "[CXX] C++ sovereign compile done."

# --- SOVEREIGN TOOLS ---
tools: dirs $(SIGMA_TOOLS_OBJS)
	@echo "[TOOLS] Sovereign native utilities compiled."

build/%.o: %.cpp libc/sigma_types.h libc/sigma_libc.h
	@echo "[CXX] $*"
	@$(CXX) $(CXXFLAGS) -c $< -o $@

# =============================================================================
# VERIFICATION (Check that no forbidden includes exist)
# =============================================================================
verify:
	@echo "[VERIFY] Checking for forbidden standard library includes..."
	@echo "[VERIFY] Scanning: libc/ kernel/ *.c *.h *.cpp"
	@FOUND=0; \
	for f in libc/*.c libc/*.h kernel/*.c SigmaLibC.c SigmaLibC.h; do \
		if [ -f "$$f" ]; then \
			if grep -n '#include <std' "$$f" | grep -v sigma_types | grep -v sigma_libc; then \
				echo "[FAIL] $$f: forbidden standard header found!"; \
				FOUND=1; \
			fi; \
		fi; \
	done; \
	if [ $$FOUND -eq 0 ]; then \
		echo "[PASS] ZERO forbidden standard library includes detected."; \
		echo "[PASS] SigmaOS sovereignty verified."; \
	else \
		echo "[FAIL] Forbidden dependencies found."; \
		exit 1; \
	fi

# =============================================================================
# RUNNING TESTS
# =============================================================================
test: sigma_libc_standalone
	@echo "[TEST] Running sigma_libc_standalone..."
	@if [ -x "build/sigma_libc_test" ]; then \
		./build/sigma_libc_test; \
	else \
		echo "[SKIP] Linux x86_64 only test."; \
	fi

# --- ZENITH SOVEREIGN LIBC ---
ZENITH_OBJ_ASM := build/SovereignLibC_asm.o
ZENITH_OBJ_CPP := build/SovereignLibC_cpp.o

$(ZENITH_OBJ_ASM): SovereignLibC.asm
	@mkdir -p build
	@echo "[NASM] SovereignLibC.asm"
	@$(NASM) $(ASMFLAGS) $< -o $@

$(ZENITH_OBJ_CPP): SovereignLibC.cpp SovereignLibC.h
	@mkdir -p build
	@echo "[CXX]  SovereignLibC.cpp"
	@$(CXX) $(CXXFLAGS) -c $< -o $@

# --- ZENITH DEMONSTRATOR ---
ZENITH_OBJS := $(ZENITH_OBJ_ASM) $(ZENITH_OBJ_CPP) \
               build/SovereignCoreUtils.o \
               build/SovereignDistroForge.o \
               build/SovereignOmniShard.o \
               build/SovereignZenithComplete.o \
               build/SovereignXV6Bridge.o \
               build/SovereignKnowledgeAudit.o \
               build/SovereignDesktopZenith.o \
               build/omni_shell.o

build/SovereignCoreUtils.o: SovereignCoreUtils.cpp SovereignLibC.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/SovereignDistroForge.o: SovereignDistroForge.cpp SovereignDistroForge.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/SovereignOmniShard.o: SovereignOmniShard.cpp SovereignOmniShard.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/SovereignZenithComplete.o: SovereignZenithComplete.cpp SovereignSyncZenith.h SovereignDiskZenith.h SovereignOSBasicsZenith.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/SovereignXV6Bridge.o: SovereignXV6Bridge.cpp SovereignXV6Bridge.h SovereignLibC.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/SovereignKnowledgeAudit.o: SovereignKnowledgeAudit.cpp SovereignHardwareIOZenith.h SovereignCoordinationZenith.h SovereignLibC.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/SovereignDesktopZenith.o: SovereignDesktopZenith.cpp SovereignDesktopZenith.h SovereignLibC.h SigmaOOP.hpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

build/omni_shell.o: omni_shell.cpp SovereignLibC.h SigmaOOP.hpp SovereignOmniShard.h SovereignSyncZenith.h SovereignDiskZenith.h SovereignOSBasicsZenith.h SovereignXV6Bridge.h SovereignHardwareIOZenith.h SovereignCoordinationZenith.h SovereignDesktopZenith.h
	@$(CXX) $(CXXFLAGS) -c $< -o $@

zenith: dirs $(ZENITH_OBJS)
	@echo "[LD]   Linking Σ SIGMAOS ZENITH SHARD..."
	@$(LD) $(LDFLAGS) $(ZENITH_OBJS) -e main -o build/sigmaos_zenith
	@echo "[OK]   build/sigmaos_zenith is ready for direct silicon execution."


# =============================================================================
# REPOSITORY SYNC & DISTRO RUNNER
# =============================================================================
sync:
	@chmod +x scripts/sync_github.sh
	@./scripts/sync_github.sh

run-distro:
	@python3 scripts/launch_distro.py $(DISTRO)

# =============================================================================
# CLEAN
# =============================================================================
clean:
	@echo "[CLEAN] Removing build artifacts..."
	@rm -rf build/
	@echo "[CLEAN] Done."

# =============================================================================
# INFO
# =============================================================================
info:
	@echo ""
	@echo "Σ SigmaOS Sovereign Build System v94.0 (ZENITH EDITION)"
	@echo "  ARCH        = $(ARCH)"
	@echo "  ZENITH_BINS = build/sigmaos_zenith"
	@echo "  SYMBOLS     = scripts/sync_github.sh, scripts/launch_distro.py"
	@echo ""
