# SigmaOS Makefile
# Build system for SigmaOS operating system

# Set standard reproducible build epoch timestamp
export SOURCE_DATE_EPOCH ?= 1716000000

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

# Verbose output
ifeq ($(V),1)
CARGO_FLAGS += --verbose
endif

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
	@rm -rf build
	@cargo clean
	@echo "Clean complete."

# Distclean - remove all generated files
distclean: clean
	@echo "Removing all generated files..."
	@rm -rf target
	@rm -f Cargo.lock
	@echo "Distclean complete."

# Mrproper - remove everything including config
mrproper: distclean
	@echo "Removing configuration..."
	@rm -f Config.sigma
	@rm -f .config
	@echo "Mrproper complete."

# Build all networking, security, and processor compatibility tools from source
compat:
	@mkdir -p build
	@echo "Building SigmaOS networking, security, and processor compatibility tools..."
	@rustc --crate-type=lib tools/sigma_ssh_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_scp_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_nfs_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_samba_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_rsync_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_tcpdump_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_dns_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_secure_alloc_compat.rs --out-dir build/
	@rustc --crate-type=lib tools/sigma_cpu_compat.rs --out-dir build/
	@echo "Compatibility tools build complete."

# Build complete system (Unified profile routing with no circular warnings)
build: compat
	@mkdir -p build
	@echo "Building SigmaOS (Profile: $(PROFILE), Arch: $(ARCH))..."
	@cargo build $(CARGO_FLAGS)
ifeq ($(PROFILE),browser)
	@wasm-pack build --target web
endif
	@./scripts/build-iso.sh
	@echo "Build complete."

# Build kernel only
kernel:
	@mkdir -p build
	@echo "Building SigmaOS kernel..."
	@cargo build --bin sigma_kernel
	@echo "Kernel build complete."

# Build drivers only
drivers:
	@mkdir -p build
	@echo "Building SigmaOS drivers..."
	@cargo build --bin sigma_drivers
	@echo "Drivers build complete."

# Build userspace only
userspace:
	@mkdir -p build
	@echo "Building SigmaOS userspace..."
	@cargo build --bin sigma_userspace
	@echo "Userspace build complete."

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
	@qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio -no-reboot -display none
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

# Architecture-specific settings
ifeq ($(ARCH),aarch64)
CROSS_COMPILE ?= aarch64-linux-gnu-
else ifeq ($(ARCH),riscv64)
CROSS_COMPILE ?= riscv64-linux-gnu-
endif
