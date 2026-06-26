# =========================================================================
# SIGMAOS: INDUSTRIAL KERNEL MAKEFILE (v15.0 - ZENITH)
# =========================================================================

# --- Reproducible builds (NixOS-inspired) ---------------------------------
# Stamp every build with SOURCE_DATE_EPOCH so binaries are bit-for-bit
# identical across machines when built from the same source tree.
ifdef SOURCE_DATE_EPOCH
  TIMESTAMP_FLAG = -DSIGMA_BUILD_TIMESTAMP=$(SOURCE_DATE_EPOCH)
else
  TIMESTAMP_FLAG =
endif

CC = x86_64-linux-gnu-gcc
CXX = x86_64-linux-gnu-g++
LD = x86_64-linux-gnu-ld
ASM = nasm

# --- Kernel flags (freestanding — no host libc, no stack protector in ring 0)
CFLAGS = -Iinclude -ffreestanding -mno-red-zone -mcmodel=kernel \
         -fno-stack-protector -fno-exceptions -fno-rtti \
         -Wall -Wextra -Werror=format-security \
         -O2 -fno-pie -nostdlib \
         $(TIMESTAMP_FLAG)
CXXFLAGS = $(CFLAGS) -std=c++17

# --- Userland / daemon hardening flags (Alpine-inspired) ------------------
SIGMA_USERLAND_FLAGS = \
  -fstack-protector-strong \
  -fPIE \
  -D_FORTIFY_SOURCE=2 \
  -Wformat \
  -Wformat-security \
  -Werror=format-security

SIGMA_USERLAND_LDFLAGS = \
  -Wl,-z,relro \
  -Wl,-z,now \
  -pie
ASMFLAGS = -f elf64

# --- SPDX: all source files should carry GPL-2.0-or-later headers ---------
# Enforced via CI; not a make rule, but documented here for contributors.

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

.PHONY: all clean iso qemu

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

clean:
	rm -rf $(BUILD_DIR)

# ── USE-flag-style feature toggles (Gentoo portage inspired) ─────────────────
# Override on the command line:  make SIGMA_USE_AI_ENGINE=0
# Or via a profile:              cmake -DCMAKE_TOOLCHAIN_FILE=profiles/iot-minimal.cmake
SIGMA_USE_HYPERVISOR   ?= 1
SIGMA_USE_AI_ENGINE    ?= 1
SIGMA_USE_ZENITH_DE    ?= 1
SIGMA_USE_CLUSTER      ?= 0
SIGMA_USE_BLUETOOTH    ?= 1
SIGMA_USE_WIFI         ?= 1
SIGMA_USE_CRYPTFS      ?= 1
SIGMA_USE_PQ_NET       ?= 0
SIGMA_USE_WASM         ?= 0

# Propagate USE flags as preprocessor defines into the kernel binary
ifeq ($(SIGMA_USE_HYPERVISOR),1)
  CFLAGS   += -DSIGMA_HAS_HYPERVISOR
  CXXFLAGS += -DSIGMA_HAS_HYPERVISOR
  SRC_DIRS += kernel/virt
endif
ifeq ($(SIGMA_USE_AI_ENGINE),1)
  CFLAGS   += -DSIGMA_HAS_AI
  CXXFLAGS += -DSIGMA_HAS_AI
endif
ifeq ($(SIGMA_USE_ZENITH_DE),0)
  # Headless/server profile — exclude all GUI sources
  CFLAGS   += -DSIGMA_HEADLESS
  CXXFLAGS += -DSIGMA_HEADLESS
endif
ifeq ($(SIGMA_USE_CRYPTFS),1)
  CFLAGS   += -DSIGMA_HAS_CRYPTFS
  CXXFLAGS += -DSIGMA_HAS_CRYPTFS
endif
ifeq ($(SIGMA_USE_PQ_NET),1)
  CFLAGS   += -DSIGMA_HAS_PQ_NET
  CXXFLAGS += -DSIGMA_HAS_PQ_NET
endif
ifeq ($(SIGMA_USE_WASM),1)
  CFLAGS   += -DSIGMA_HAS_WASM
  CXXFLAGS += -DSIGMA_HAS_WASM
endif
