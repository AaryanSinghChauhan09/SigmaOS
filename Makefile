# SigmaOS Makefile
# Build system for SigmaOS operating system (Linux Kernel & Distro Inspired)

# Set standard reproducible build epoch timestamp
export SOURCE_DATE_EPOCH ?= 1716000000

# ==============================================================================
# VERBOSE & SILENT BUILD SYSTEM CONFIGURATION (Linux Kernel Kbuild inspired)
# ==============================================================================
ifeq ($(V),1)
  cmd = $(2)
  Q =
else
  cmd = @$(if $(quiet_cmd_$(1)),echo $(quiet_cmd_$(1));) $(2)
  Q = @
endif

# Aligned log status printing helpers
quiet_cmd_rustc = "  RUSTC   $(1)"
quiet_cmd_gen   = "  GEN     $(1)"
quiet_cmd_clean = "  CLEAN   $(1)"
quiet_cmd_cc    = "  CC      $(1)"
quiet_cmd_ld    = "  LD      $(1)"

# ==============================================================================
# TARGET ARCHITECTURE DETECTORS
# ==============================================================================
ifndef ARCH
  UNAME_M := $(shell uname -m)
  ifeq ($(UNAME_M),x86_64)
    ARCH = x86_64
  else ifeq ($(UNAME_M),aarch64)
    ARCH = aarch64
  else ifeq ($(UNAME_M),arm64)
    ARCH = aarch64
  else
    ARCH = riscv64
  endif
endif

.PHONY: all clean build kernel drivers userspace test test-unit test-integration test-qemu help defconfig menuconfig

# Default target
all: build

# Build profiles
PROFILE ?= standalone
DEBUG ?= 0
RELEASE ?= 0

# Compiler settings
RUSTC = rustc
CARGO = cargo
NASM = nasm
QEMU = qemu-system-x86_64

# Determine Cargo Flags and Features dynamically based on PROFILE
CARGO_FLAGS =

ifeq ($(RELEASE),1)
CARGO_FLAGS += --release
endif

ifeq ($(PROFILE),standalone)
CARGO_FLAGS += --release --features "desktop drivers ai"
else ifeq ($(PROFILE),microkernel)
CARGO_FLAGS += --release --features "microkernel core-shards"
else ifeq ($(PROFILE),rtos)
CARGO_FLAGS += --release --features "rtos realtime"
else ifeq ($(PROFILE),cloud)
CARGO_FLAGS += --release --features "cloud cloud-init"
else ifeq ($(PROFILE),browser)
CARGO_FLAGS += --release --features "wasm browser"
else
CARGO_FLAGS += --profile $(if $(filter 1,$(RELEASE)),release,dev)
endif

# Verbose output flag passing to Cargo
ifeq ($(V),1)
CARGO_FLAGS += --verbose
endif

# ==============================================================================
# CONFIGURATION MANAGEMENT TARGETS
# ==============================================================================
defconfig:
	$(call cmd,gen,echo "Generating standard Config.sigma layout...")
	$(Q)echo "CONFIG_PROFILE=standalone" > Config.sigma
	$(Q)echo "CONFIG_ARCH=$(ARCH)" >> Config.sigma
	$(Q)echo "CONFIG_DEBUG=0" >> Config.sigma
	$(Q)cp Config.sigma .config

menuconfig:
	$(Q)if [ -f "Config.sigma" ]; then \
		echo "Active Config.sigma values:"; \
		cat Config.sigma; \
	else \
		echo "No active Config.sigma found. Run 'make defconfig' to generate defaults."; \
	fi

# ==============================================================================
# COMPILATION & ASSEMBLING PIPELINES
# ==============================================================================

# Clean build artifacts
clean:
	$(call cmd,clean,rm -rf build)
	$(Q)$(CARGO) clean
	@echo "Clean complete."

# Mock target for Singularity image and container validation pipelines
singularity:
	$(call cmd,gen,mkdir -p build)
	@echo "[CI] Mock Singularity verification image built successfully."

# Distclean - remove all generated files
distclean: clean
	$(call cmd,clean,rm -rf target Cargo.lock)
	@echo "Distclean complete."

# Mrproper - remove everything including config
mrproper: distclean
	$(call cmd,clean,rm -f Config.sigma .config)
	@echo "Mrproper complete."

# Build complete system (Unified profile routing with no circular warnings)
build:
	$(call cmd,rustc,echo "Building SigmaOS (Profile: $(PROFILE) - Arch: $(ARCH))...")
	$(Q)mkdir -p build
	$(Q)$(CARGO) build $(CARGO_FLAGS)
ifeq ($(PROFILE),browser)
	$(call cmd,rustc,echo "wasm-pack build --target web...")
	$(Q)wasm-pack build --target web
endif
	$(call cmd,gen,echo "Generating final bootable image build/sigmaos.iso...")
	$(Q)bash ./scripts/build-iso.sh
	@echo "Build complete."

# Build kernel only
kernel:
	$(call cmd,rustc,echo "Building SigmaOS kernel...")
	$(Q)mkdir -p build
	$(Q)$(CARGO) build --bin sigma_kernel
	@echo "Kernel build complete."

# Build drivers only
drivers:
	$(call cmd,rustc,echo "Building SigmaOS drivers...")
	$(Q)mkdir -p build
	$(Q)$(CARGO) build --bin sigma_drivers
	@echo "Drivers build complete."

# Build userspace only
userspace:
	$(call cmd,rustc,echo "Building SigmaOS userspace...")
	$(Q)mkdir -p build
	$(Q)$(CARGO) build --bin sigma_userspace
	@echo "Userspace build complete."

# ==============================================================================
# QUALITY ASSURANCE & TESTING HARNESSES
# ==============================================================================

# Run all tests
test:
	$(call cmd,gen,echo "Running all tests...")
	$(Q)$(CARGO) test
	@echo "All tests complete."

# Run unit tests only
test-unit:
	$(call cmd,gen,echo "Running unit tests...")
	$(Q)$(CARGO) test --lib
	@echo "Unit tests complete."

# Run integration tests
test-integration:
	$(call cmd,gen,echo "Running integration tests...")
	$(Q)$(CARGO) test --test '*'
	@echo "Integration tests complete."

# Run QEMU boot test
test-qemu: build
	$(call cmd,gen,echo "Running QEMU boot test...")
	$(Q)python3 scripts/qemu_smoke_test.py --headless
	@echo "QEMU boot test complete."

# Format code
fmt:
	$(call cmd,gen,echo "Formatting code...")
	$(Q)$(CARGO) fmt
	@echo "Formatting complete."

# Lint code
lint:
	$(call cmd,gen,echo "Linting code...")
	$(Q)$(CARGO) clippy -- -D warnings
	@echo "Linting complete."

# Check code
check:
	$(call cmd,rustc,echo "Checking code...")
	$(Q)$(CARGO) check
	@echo "Code check complete."

# Build documentation
docs:
	$(call cmd,gen,echo "Building documentation...")
	$(Q)$(CARGO) doc --no-deps
	@echo "Documentation build complete."

# Install dependencies
deps:
	$(call cmd,gen,echo "Installing dependencies...")
	$(Q)$(CARGO) fetch
	@echo "Dependencies installed."

# Update dependencies
update:
	$(call cmd,gen,echo "Updating dependencies...")
	$(Q)$(CARGO) update
	@echo "Dependencies updated."

# Architecture-specific settings
ifeq ($(ARCH),aarch64)
CROSS_COMPILE ?= aarch64-linux-gnu-
else ifeq ($(ARCH),riscv64)
CROSS_COMPILE ?= riscv64-linux-gnu-
endif

# ==============================================================================
# HELP TERMINAL REFERENCE MANUAL
# ==============================================================================
help:
	@echo "SigmaOS Build System (Linux Kernel Inspired)"
	@echo "============================================="
	@echo ""
	@echo "Available targets:"
	@echo "  all              - Build complete system (default)"
	@echo "  build            - Build complete system"
	@echo "  kernel           - Build kernel only"
	@echo "  drivers          - Build drivers only"
	@echo "  userspace        - Build userspace only"
	@echo "  clean            - Remove build artifacts"
	@echo "  distclean        - Remove all generated files"
	@echo "  mrproper         - Remove everything including config"
	@echo "  defconfig        - Generate standard default Config.sigma templates"
	@echo "  menuconfig       - Display active build options"
	@echo "  test             - Run all tests"
	@echo "  test-unit        - Run unit tests"
	@echo "  test-integration - Run integration tests"
	@echo "  test-qemu        - Run QEMU boot test"
	@echo ""
	@echo "Build profiles:"
	@echo "  PROFILE=standalone - Full desktop ISO (default)"
	@echo "  PROFILE=microkernel - Minimal kernel, core shards"
	@echo "  PROFILE=rtos        - Hard real-time ELF"
	@echo "  PROFILE=cloud       - Headless cloud image"
	@echo "  PROFILE=browser     - WASM bundle"
	@echo ""
	@echo "Architecture:"
	@echo "  ARCH=x86_64   - Intel/AMD (default)"
	@echo "  ARCH=aarch64  - ARM64"
	@echo "  ARCH=riscv64  - RISC-V"
	@echo ""
	@echo "Build options:"
	@echo "  DEBUG=1       - Enable debug symbols"
	@echo "  RELEASE=1     - Enable release optimizations"
	@echo "  V=1           - Verbose build output (Kbuild style)"
	@echo ""
	@echo "Examples:"
	@echo "  make PROFILE=standalone all"
	@echo "  make ARCH=aarch64 all"
	@echo "  make V=1 build"
