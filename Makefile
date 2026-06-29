# =========================================================================
# SIGMAOS: INDUSTRIAL KERNEL MAKEFILE (v15.0 - ZENITH)
# Unified mainline with OS-specific build targets.
#
# Usage:
#   make TARGET_OS=sigma    (default — Native SigmaOS, POSIX-free)
#   make TARGET_OS=ubuntu   (Ubuntu / Linux compatibility target)
#   make TARGET_OS=bsd      (BSD-style target)
#
# The TARGET_OS value controls:
#   1. Preprocessor define:  -DTARGET_OS_SIGMA | _UBUNTU | _BSD
#   2. Driver source directory added to SRC_DIRS
#   3. OS profile path printed for reference (config/<target>.yaml)
# =========================================================================

# --- Reproducible builds (NixOS-inspired) ---------------------------------
# Stamp every build with SOURCE_DATE_EPOCH so binaries are bit-for-bit
# identical across machines when built from the same source tree.
ifdef SOURCE_DATE_EPOCH
  TIMESTAMP_FLAG = -DSIGMA_BUILD_TIMESTAMP=$(SOURCE_DATE_EPOCH)
else
  TIMESTAMP_FLAG =
endif

# ── TARGET_OS — OS-specific build target ──────────────────────────────────
# Selects the driver layer and preprocessor define.
# Override on the command line:  make TARGET_OS=ubuntu
TARGET_OS ?= sigma

ifeq ($(TARGET_OS),sigma)
  OS_DEFINE    := TARGET_OS_SIGMA
  OS_DRIVER_DIR:= drivers/sigma
  OS_PROFILE   := config/sigma.yaml
else ifeq ($(TARGET_OS),ubuntu)
  OS_DEFINE    := TARGET_OS_UBUNTU
  OS_DRIVER_DIR:= drivers/linux
  OS_PROFILE   := config/ubuntu.yaml
else ifeq ($(TARGET_OS),bsd)
  OS_DEFINE    := TARGET_OS_BSD
  OS_DRIVER_DIR:= drivers/bsd
  OS_PROFILE   := config/bsd.yaml
else
  $(error [SigmaOS] Unknown TARGET_OS='$(TARGET_OS)'. Choose: sigma | ubuntu | bsd)
endif

$(info [SigmaOS] TARGET_OS=$(TARGET_OS)  define=-D$(OS_DEFINE)  drivers=$(OS_DRIVER_DIR))
$(info [SigmaOS] OS profile : $(OS_PROFILE))

CC = x86_64-linux-gnu-gcc
CXX = x86_64-linux-gnu-g++
LD = x86_64-linux-gnu-ld
ASM = nasm

# --- Kernel flags (freestanding — no host libc, no stack protector in ring 0)
CFLAGS = -Iinclude -ffreestanding -mno-red-zone -mcmodel=kernel \
         -fno-stack-protector -fno-exceptions -fno-rtti \
         -Wall -Wextra -Werror=format-security \
         -O2 -fno-pie -nostdlib \
         -D$(OS_DEFINE) \
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
# OS_DRIVER_DIR is appended based on TARGET_OS (sigma | ubuntu | bsd)
SRC_DIRS := kernel/core kernel/core/drivers/input kernel/core/memory kernel/core/sched kernel/core/system kernel/core/syscall kernel/core/hal kernel/core/vulkan kernel/net kernel/storage kernel/telemetry tools usr init fs net lib/libc $(OS_DRIVER_DIR)
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

# ── Immutable root option (Bottlerocket-inspired) ────────────────────────────
# When ON: root is remounted read-only after pivot, sigma-pkg CLI is excluded.
SIGMA_IMMUTABLE_ROOT ?= 0
ifeq ($(SIGMA_IMMUTABLE_ROOT),1)
  CFLAGS   += -DSIGMA_READONLY_ROOT=1
  CXXFLAGS += -DSIGMA_READONLY_ROOT=1
  # Remove sigma-pkg CLI from install targets — meaningless on immutable root
  INSTALL_TARGETS := $(filter-out sigma-pkg-cli, $(INSTALL_TARGETS))
  $(info [sigma] SIGMA_IMMUTABLE_ROOT=1: root will be remounted read-only at boot)
endif

# ── BR2_BROKEN-style stub tracker (Buildroot-inspired) ───────────────────────
# Any subsystem listed here is a known stub. A warning is printed on every
# build. Release builds (SIGMA_RELEASE_BUILD=1) FAIL if any stubs are enabled.
#
# To suppress a warning while working on a stub: set SIGMA_USE_<NAME>=0
# To fix a stub: implement it and remove it from this list.
SIGMA_BROKEN_SUBSYSTEMS := \
  sigma-jail       "Only prints to console — no real namespace isolation"       \
  sigma-mac        "Always returns GRANTED — no policy evaluation"              \
  sigma-cryptfs    "derive_key() is a stub — encryption is never applied"       \
  sigma-rollback   "sigma_ostree replaces this — old file was 404"             \
  sigma-cluster    "No distributed consensus implemented yet"                   \
  kernel/core      "Directory is empty — scheduler/mm/syscall files missing"

define PRINT_BROKEN
  @echo "  [STUB] $(1): $(2)"
endef

.PHONY: check-stubs
check-stubs:
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════╗"
	@echo "║      SIGMA WARNING: STUB SUBSYSTEMS PRESENT IN BUILD        ║"
	@echo "╠══════════════════════════════════════════════════════════════╣"
	@echo "║  sigma-jail:    Only prints — no real namespace isolation   ║"
	@echo "║  sigma-mac:     Always GRANTED — no policy evaluation       ║"
	@echo "║  sigma-cryptfs: derive_key() stub — encryption not applied  ║"
	@echo "║  sigma-rollback:sigma_ostree replaces this                  ║"
	@echo "║  sigma-cluster: No distributed consensus implemented        ║"
	@echo "║  kernel/core:   Directory empty — core files missing        ║"
	@echo "╠══════════════════════════════════════════════════════════════╣"
	@echo "║  Fix or set SIGMA_USE_<SUBSYSTEM>=0 to suppress.            ║"
	@echo "╚══════════════════════════════════════════════════════════════╝"
	@echo ""
ifeq ($(SIGMA_RELEASE_BUILD),1)
	$(error Release build blocked: stub subsystems present. Implement them or set SIGMA_USE_<SUBSYSTEM>=0)
endif

# Run stub check before every build
all: check-stubs
