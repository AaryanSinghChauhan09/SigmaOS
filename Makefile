# =========================================================================
# SIGMAOS: INDUSTRIAL KERNEL MAKEFILE (v15.0 - ZENITH)
# =========================================================================

CC = x86_64-linux-gnu-gcc
CXX = x86_64-linux-gnu-g++
LD = x86_64-linux-gnu-ld
ASM = nasm

# Clear-Linux-inspired: LTO enabled by default for cross-file inlining and dead-code elimination
CFLAGS = -Iinclude -Ilib/musl/include -Ilib/mesa/include -Ilib/linux-drivers/include -Ilib/wayland/src -ffreestanding -mno-red-zone -Wall -Wextra -O2 -fno-pie -flto
CXXFLAGS = $(CFLAGS) -fno-exceptions -fno-rtti -std=c++17
ASMFLAGS = -f elf64

BUILD_DIR = build
ISO_DIR = $(BUILD_DIR)/iso
KERNEL_BIN = $(BUILD_DIR)/sigmaos.bin
ISO_IMAGE = $(BUILD_DIR)/sigmaos.iso

# Directories to search for source files
SRC_DIRS := kernel/core kernel/core/drivers/input kernel/core/memory kernel/core/sched kernel/core/system kernel/core/syscall kernel/core/hal kernel/core/vulkan kernel/net kernel/storage kernel/telemetry tools usr init fs net lib/libc
C_SRCS := $(shell find $(SRC_DIRS) -name '*.c')
CXX_SRCS := $(shell find $(SRC_DIRS) -name '*.cpp')
ASM_SRCS := $(shell find $(SRC_DIRS) -name '*.asm')

# Object files
OBJS := $(patsubst %.c, $(BUILD_DIR)/%.o, $(C_SRCS)) \
        $(patsubst %.cpp, $(BUILD_DIR)/%.o, $(CXX_SRCS)) \
        $(patsubst %.asm, $(BUILD_DIR)/%.o, $(ASM_SRCS))

.PHONY: all clean iso qemu pgo-generate pgo-use

all: iso

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.asm
	@mkdir -p $(dir $@)
	$(ASM) $(ASMFLAGS) $< -o $@

$(KERNEL_BIN): $(OBJS)
	# Link using proper linker script
	$(LD) -n -T linker.ld -o $@ $^

iso: $(KERNEL_BIN)
	@mkdir -p $(ISO_DIR)/boot/grub
	@cp $(KERNEL_BIN) $(ISO_DIR)/boot/
	@echo "menuentry 'SigmaOS Zenith' {" > $(ISO_DIR)/boot/grub/grub.cfg
	@echo "    multiboot /boot/sigmaos.bin" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "    boot" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "}" >> $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO_IMAGE) $(ISO_DIR)

qemu: iso
	qemu-system-x86_64 -cdrom $(ISO_IMAGE) -serial stdio -m 2G

# =========================================================================
# RPi-Distro-inspired: ARM64 Embedded Scaling Target
# Usage: make arm64-rpi
# =========================================================================
arm64-rpi:
	@echo "[ARM64] Switching toolchain to aarch64-linux-gnu..."
	$(MAKE) CC=aarch64-linux-gnu-gcc CXX=aarch64-linux-gnu-g++ LD=aarch64-linux-gnu-ld \
	        CFLAGS="-Iinclude -ffreestanding -mcpu=cortex-a72 -Wall -Wextra -O2 -fno-pie" \
	        KERNEL_BIN=$(BUILD_DIR)/sigmaos_arm64.bin iso
	@echo "[ARM64] Successfully built SigmaOS for Raspberry Pi (embedded profile)."

# =========================================================================
# Debian-Edu-inspired: Specialized Editions
# =========================================================================
iso-iot:
	@echo "[EDITION: IoT] Stripping GUI, forcing MINIMAL_MODE..."
	$(MAKE) CFLAGS="$(CFLAGS) -DSIGMA_MINIMAL_MODE=1 -DSIGMA_STRIP_GUI=1" iso

iso-research:
	@echo "[EDITION: Research] Disabling strict sandboxing for compute clusters..."
	$(MAKE) CFLAGS="$(CFLAGS) -DSIGMA_RELAX_SANDBOX=1 -DSIGMA_COMPUTE_CLUSTER=1" iso

iso-secure:
	@echo "[EDITION: Secure] Enforcing strict isolation and Tor routing default..."
	$(MAKE) CFLAGS="$(CFLAGS) -DSIGMA_STRICT_ISOLATION=1 -DSIGMA_FORCE_TOR=1" iso

# =========================================================================
# Clear-Linux-inspired: Profile-Guided Optimization (PGO) skeleton
# Usage: make pgo-generate  ->  run workload  ->  make pgo-use
# =========================================================================
pgo-generate:
	@echo "[PGO] Phase 1: Compiling instrumented build for profiling..."
	$(MAKE) CFLAGS="$(CFLAGS) -fprofile-generate=./pgo-data" CXXFLAGS="$(CXXFLAGS) -fprofile-generate=./pgo-data" iso
	@echo "[PGO] Instrumented build ready. Run your benchmark/boot workload, then run: make pgo-use"

pgo-use:
	@echo "[PGO] Phase 2: Compiling optimized build using profile data..."
	$(MAKE) CFLAGS="$(CFLAGS) -fprofile-use=./pgo-data -fprofile-correction" CXXFLAGS="$(CXXFLAGS) -fprofile-use=./pgo-data -fprofile-correction" iso
	@echo "[PGO] Optimized PGO kernel built successfully."

clean:
	rm -rf $(BUILD_DIR)
CONFIG_MONOLITHIC_DRIVERS=0
CONFIG_MICROKERNEL=1
