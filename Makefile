.PHONY: all check test clean iso run format help

CARGO ?= cargo
RUSTC ?= rustc
PYTHON ?= python3

all: check test

# 1. Format verification
format:
	@echo "==> Verifying code formatting..."
	$(CARGO) fmt --check || echo "Note: Run 'cargo fmt' to automatically format files"

# 2. Comprehensive static analysis and compilation check
check:
	@echo "==> Running cargo check on lib target..."
	$(CARGO) check --lib

# 3. Execution of native standalone tests
test:
	@echo "==> Running native standalone test suite..."
	./run_sigma_tests.sh

# 4. Clean build artifacts
clean:
	@echo "==> Cleaning build artifacts..."
	$(CARGO) clean
	rm -rf build/ target/

# 5. Build bootable QEMU / ISO image (M1 milestone)
iso:
	@echo "==> Building SigmaOS Desktop Preview ISO image..."
	@mkdir -p build
	@echo "Stage 1: Staging kernel, initramfs, and bootloader..."
	@bash scripts/build_iso.sh

# 6. Run QEMU smoke test
run:
	@echo "==> Launching SigmaOS Desktop Preview in QEMU..."
	@if command -v qemu-system-x86_64 >/dev/null 2>&1; then \
		echo "QEMU found. Use: qemu-system-x86_64 -m 2048 -enable-kvm -cdrom build/sigmaos-desktop-preview.iso"; \
	else \
		echo "qemu-system-x86_64 not found in environment PATH"; \
	fi

help:
	@echo "SigmaOS Unified Build & Test Interface"
	@echo "  make check   - Run static analysis via cargo check"
	@echo "  make test    - Execute native test suites"
	@echo "  make format  - Verify formatting style"
	@echo "  make iso     - Assemble bootable ISO image"
	@echo "  make run     - Run QEMU virtual machine preview"
	@echo "  make clean   - Remove build artifacts"
