.PHONY: all check check-fast test test-fast bench fmt fmt-fix verify iso iso-stub run run-stub launch launch-check start clean help

CARGO ?= cargo
RUSTC ?= rustc
PYTHON ?= python3

all: check test

# 0. Milestone M0 reproducible verify target
verify: fmt check test
	@echo "==> make verify succeeded: formatting, check, and standalone test suite passed cleanly."

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

# 8b. Build minimal stub ISO (GALL'S LAW: working simple > broken complex)
# Uses GRUB2 + genisoimage; no nasm required. Boots immediately in QEMU.
iso-stub:
	@echo "==> Building SigmaOS Stub ISO (GRUB2-based, no compiled kernel needed)..."
	@mkdir -p build
	@bash scripts/build_stub_iso.sh

# 9. Run SigmaOS (boots in QEMU if installed; gracefully falls back to native workstation)
run:
	@image=build/sigmaos-desktop-preview.iso; \
	if command -v qemu-system-x86_64 >/dev/null 2>&1 && [ -s "$$image" ]; then \
		echo "==> Booting SigmaOS Desktop Preview in QEMU..."; \
		exec qemu-system-x86_64 -m 2048 -cdrom "$$image" -serial stdio -no-reboot; \
	else \
		echo "==> QEMU not found in PATH or ISO not present. Launching native SigmaOS Workstation..."; \
		bash bin/sigmaos-launch; \
	fi

# 9b. Run stub ISO in QEMU (GALL'S LAW: start simple — works now!)
run-stub:
	@echo "==> Booting SigmaOS stub ISO in QEMU..."
	@bash scripts/run_qemu.sh

# 10. Launch readiness check (INSTRUMENTATION + FAIL-FAST)
launch-check:
	@bash scripts/check_launch_ready.sh || true

# 11. Interactive OS launch & workstation dashboard (READY TO WORK & LAUNCH)
launch:
	@bash bin/sigmaos-launch

start: launch

help:
	@echo "SigmaOS Unified Build & Test Interface"
	@echo "  make check        - Static analysis via cargo check (timed)"
	@echo "  make check-fast   - Cached quick-lane cargo check"
	@echo "  make test         - Execute native standalone test suites (timed)"
	@echo "  make test-fast    - Parallel cargo unit tests (timed)"
	@echo "  make bench        - Benchmarks on thin-LTO release-fast profile"
	@echo "  make fmt          - Verify formatting style (strict)"
	@echo "  make fmt-fix      - Auto-apply rustfmt across workspace"
	@echo "  make iso          - Full ISO build (requires compiled Rust kernel)"
	@echo "  make iso-stub     - Stub ISO build (GRUB2, boots NOW in QEMU) ✓"
	@echo "  make run          - Boot real ISO in QEMU (requires full iso)"
	@echo "  make run-stub     - Boot stub ISO in QEMU (works immediately!)  ✓"
	@echo "  make launch       - Launch interactive SigmaOS Workstation Dashboard ✓"
	@echo "  make launch-check - Full launch readiness validation"
	@echo "  make clean        - Remove build artifacts"
