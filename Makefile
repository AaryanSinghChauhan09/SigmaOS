.PHONY: all check check-fast test test-fast bench fmt fmt-fix iso run clean help

CARGO ?= cargo
RUSTC ?= rustc
PYTHON ?= python3

all: check test

# 1. Code formatting
fmt-fix:
	@echo "==> Applying rustfmt across the workspace..."
	$(CARGO) fmt

fmt:
	@echo "==> Verifying code formatting..."
	$(CARGO) fmt --check

# Backwards-compatible alias (kept for scripts / CI references)
format: fmt

# 2. Comprehensive static analysis and compilation check
check:
	@echo "==> Running cargo check on lib target..."
	@start=$$(date +%s); \
	$(CARGO) check --lib; \
	rc=$$?; \
	echo "==> cargo check finished in $$(( $$(date +%s) - start ))s"; \
	exit $$rc

# 3. Lightweight check lane for quick iteration (reuses cargo cache aggressively)
check-fast:
	@echo "==> Running cached cargo check (quick lane)..."
	@start=$$(date +%s); \
	$(CARGO) check --lib; \
	rc=$$?; \
	echo "==> fast check finished in $$(( $$(date +%s) - start ))s"; \
	exit $$rc

# 4. Execution of native standalone tests
test:
	@echo "==> Running native standalone test suite..."
	@start=$$(date +%s); \
	./run_sigma_tests.sh; \
	rc=$$?; \
	echo "==> test suite finished in $$(( $$(date +%s) - start ))s"; \
	exit $$rc

# 5. Cargo-native unit tests (fast lane: parallel test harness)
test-fast:
	@echo "==> Running cargo unit tests in parallel..."
	@start=$$(date +%s); \
	$(CARGO) test --lib; \
	rc=$$?; \
	echo "==> cargo tests finished in $$(( $$(date +%s) - start ))s"; \
	exit $$rc

# 6. Benchmarks via the release-fast thin-LTO profile (fast to build, representative)
bench:
	@echo "==> Running benchmarks (release-fast profile: thin LTO)..."
	$(CARGO) bench --profile release-fast

# 7. Clean build artifacts
clean:
	@echo "==> Cleaning build artifacts..."
	$(CARGO) clean
	rm -rf build/ target/

# 8. Build bootable QEMU / ISO image (M1 milestone)
iso:
	@echo "==> Building SigmaOS Desktop Preview ISO image..."
	@mkdir -p build
	@echo "Stage 1: Staging kernel, initramfs, and bootloader..."
	@bash scripts/build_iso.sh

# 9. Run QEMU smoke test
run:
	@set -eu; image=build/sigmaos-desktop-preview.iso; \
	if [ ! -s "$$image" ]; then echo "No non-empty SigmaOS ISO at $$image. 'make iso' is currently blocked until a real bare-metal image and initramfs are available." >&2; exit 1; fi; \
	if ! python3 -c 'import sys; f=open(sys.argv[1], "rb"); f.seek(32769); sys.exit(0 if f.read(5) == b"CD001" else 1)' "$$image"; then echo "Invalid ISO: missing ISO9660 volume descriptor." >&2; exit 1; fi; \
	if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then echo "qemu-system-x86_64 not found in PATH" >&2; exit 1; fi; \
	exec qemu-system-x86_64 -m 2048 -cdrom "$$image" -serial stdio -no-reboot

help:
	@echo "SigmaOS Unified Build & Test Interface"
	@echo "  make check       - Static analysis via cargo check (timed)"
	@echo "  make check-fast  - Cached quick-lane cargo check"
	@echo "  make test        - Execute native standalone test suites (timed)"
	@echo "  make test-fast   - Parallel cargo unit tests (timed)"
	@echo "  make bench       - Benchmarks on thin-LTO release-fast profile"
	@echo "  make fmt         - Verify formatting style (strict)"
	@echo "  make fmt-fix     - Auto-apply rustfmt across workspace"
	@echo "  make iso         - Attempt image build (currently blocked; see docs/PROJECT_STATUS.md)"
	@echo "  make run         - Boot a built ISO in QEMU (requires a validated image)"
	@echo "  make clean       - Remove build artifacts"
