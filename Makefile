# SigmaOS Makefile
# Build system for SigmaOS operating system

.PHONY: all clean build kernel drivers userspace test test-unit test-integration test-qemu help

# Default target
all: build

# Build profiles
PROFILE ?= standalone
ARCH ?= x86_64
DEBUG ?= 0
RELEASE ?= 0

# Compiler settings
RUSTC ?= rustc
CARGO ?= cargo
NASM ?= nasm
QEMU ?= qemu-system-x86_64

# Build directories
BUILD_DIR ?= build
TARGET_DIR ?= target

# Help target
help:
	@echo "SigmaOS Build System"
	@echo "==================="
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
	@echo "  V=1           - Verbose build output"
	@echo ""
	@echo "Examples:"
	@echo "  make PROFILE=standalone all"
	@echo "  make ARCH=aarch64 all"
	@echo "  make DEBUG=1 test"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(BUILD_DIR)
	@cargo clean
	@echo "Clean complete."

# Distclean - remove all generated files
distclean: clean
	@echo "Removing all generated files..."
	@rm -rf $(TARGET_DIR)
	@rm -f Cargo.lock
	@echo "Distclean complete."

# Mrproper - remove everything including config
mrproper: distclean
	@echo "Removing configuration..."
	@rm -f Config.sigma
	@rm -f .config
	@echo "Mrproper complete."

# Create build directories
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

# Build complete system
build: $(BUILD_DIR)
	@echo "Building SigmaOS (Profile: $(PROFILE), Arch: $(ARCH))..."
	@cargo build --profile $(if $(filter 1,$(RELEASE)),release,dev)
	@echo "Build complete."

# Build kernel only
kernel: $(BUILD_DIR)
	@echo "Building SigmaOS kernel..."
	@cargo build --bin sigma_kernel
	@echo "Kernel build complete."

# Build drivers only
drivers: $(BUILD_DIR)
	@echo "Building SigmaOS drivers..."
	@cargo build --bin sigma_drivers
	@echo "Drivers build complete."

# Build userspace only
userspace: $(BUILD_DIR)
	@echo "Building SigmaOS userspace..."
	@cargo build --bin sigma_userspace
	@echo "Userspace build complete."

# Build standalone profile
standalone: $(BUILD_DIR)
	@echo "Building standalone profile..."
	@cargo build --release --features "desktop drivers ai"
	@echo "Standalone build complete."

# Build microkernel profile
microkernel: $(BUILD_DIR)
	@echo "Building microkernel profile..."
	@cargo build --release --features "microkernel core-shards"
	@echo "Microkernel build complete."

# Build RTOS profile
rtos: $(BUILD_DIR)
	@echo "Building RTOS profile..."
	@cargo build --release --features "rtos realtime"
	@echo "RTOS build complete."

# Build cloud profile
cloud: $(BUILD_DIR)
	@echo "Building cloud profile..."
	@cargo build --release --features "cloud cloud-init"
	@echo "Cloud build complete."

# Build browser profile
browser: $(BUILD_DIR)
	@echo "Building browser profile..."
	@cargo build --release --features "wasm browser"
	@wasm-pack build --target web
	@echo "Browser build complete."

# Run all tests
test:
	@echo "Running all tests..."
	@cargo test
	@echo "All tests complete."

# Run unit tests only
test-unit:
	@echo "Running unit tests..."
	@cargo test --lib
	@echo "Unit tests complete."

# Run integration tests
test-integration:
	@echo "Running integration tests..."
	@cargo test --test '*'
	@echo "Integration tests complete."

# Run QEMU boot test
test-qemu: build
	@echo "Running QEMU boot test..."
	@$(QEMU) -cdrom $(BUILD_DIR)/sigmaos.iso -m 2G -serial stdio -no-reboot -display none
	@echo "QEMU boot test complete."

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt
	@echo "Formatting complete."

# Lint code
lint:
	@echo "Linting code..."
	@cargo clippy -- -D warnings
	@echo "Linting complete."

# Check code
check:
	@echo "Checking code..."
	@cargo check
	@echo "Code check complete."

# Build documentation
docs:
	@echo "Building documentation..."
	@cargo doc --no-deps
	@echo "Documentation build complete."

# Install dependencies
deps:
	@echo "Installing dependencies..."
	@cargo fetch
	@echo "Dependencies installed."

# Update dependencies
update:
	@echo "Updating dependencies..."
	@cargo update
	@echo "Dependencies updated."

# Profile-specific builds
ifeq ($(PROFILE),standalone)
build: standalone
else ifeq ($(PROFILE),microkernel)
build: microkernel
else ifeq ($(PROFILE),rtos)
build: rtos
else ifeq ($(PROFILE),cloud)
build: cloud
else ifeq ($(PROFILE),browser)
build: browser
endif

# Architecture-specific settings
ifeq ($(ARCH),aarch64)
CROSS_COMPILE ?= aarch64-linux-gnu-
else ifeq ($(ARCH),riscv64)
CROSS_COMPILE ?= riscv64-linux-gnu-
endif

# Debug/Release settings
ifeq ($(DEBUG),1)
CARGO_FLAGS += --debug
endif

ifeq ($(RELEASE),1)
CARGO_FLAGS += --release
endif

# Verbose output
ifeq ($(V),1)
CARGO_FLAGS += --verbose
endif
