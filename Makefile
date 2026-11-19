.PHONY: all check check-fast test test-fast bench fmt fmt-fix verify registry-audit consolidation-audit iso iso-stub run run-stub launch launch-check start mint arch omarchy gui desktop native tray install diagnose doctor fonts install-deps sandbox dotfiles settings boot-img clean help hub hub-list hub-status hub-audit hub-create hub-upgrade hub-sync hub-search hub-info hub-depends hub-families hub-consolidate hub-test hub-health hub-dupes kernel

CARGO ?= cargo
RUSTC ?= rustc
PYTHON ?= python3

all: check test

# 0. Milestone M0 reproducible verify target
verify: fmt check test registry-audit consolidation-audit
	@echo "==> make verify succeeded: formatting, check, tests, and registry audit passed cleanly."

# Read-only inventory of all repositories declared in COMPONENTS.toml.
registry-audit:
	@$(PYTHON) scripts/audit_component_fleet.py

# Read-only validation of canonical suite mappings and unresolved constituents.
consolidation-audit:
	@$(PYTHON) scripts/audit_consolidation.py

# Component-repository hub: this repo is the connector for the SigmaOS-*
# fleet. See scripts/sigma_hub.sh --help for the full command set.
hub:
	@bash scripts/sigma_hub.sh help

hub-list:
	@bash scripts/sigma_hub.sh list $(FILTER)

hub-status:
	@bash scripts/sigma_hub.sh status $(NAME)

hub-audit:
	@bash scripts/sigma_hub.sh audit

hub-create:
	@bash scripts/sigma_hub.sh create $(NAME) $(ARGS)

hub-upgrade:
	@bash scripts/sigma_hub.sh upgrade $(NAME) $(ARGS)

hub-sync:
	@bash scripts/sigma_hub.sh sync $(ARGS)

hub-search:
	@bash scripts/sigma_hub.sh search $(ARGS)

hub-info:
	@bash scripts/sigma_hub.sh info $(NAME)

hub-depends:
	@bash scripts/sigma_hub.sh depends $(NAME)

hub-families:
	@bash scripts/sigma_hub.sh families $(ARGS)

hub-consolidate:
	@bash scripts/sigma_hub.sh consolidate $(ARGS)

hub-test:
	@bash scripts/sigma_hub.sh test $(ARGS)

hub-health:
	@bash scripts/sigma_hub.sh health

hub-dupes:
	@bash scripts/sigma_hub.sh dupes

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
# NOTE: CARGO_BUILD_JOBS=8 prevents OOM on 16 GiB machines for the ~1M-LOC
# codebase. The profile.test in Cargo.toml forces codegen-units=1 for safety.
test-fast:
	@echo "==> Running cargo unit tests (memory-safe: CARGO_BUILD_JOBS=8)..."
	@start=$$(date +%s); \
	CARGO_BUILD_JOBS=8 $(CARGO) test --lib; \
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
	rm -rf build/ target/ boot/sigma_kernel.elf bin/sigmaos

# 7b. Build standalone Multiboot kernel
kernel:
	@echo "==> Compiling SigmaOS Sovereign Multiboot Kernel (Multiboot1 & Multiboot2)..."
	@gcc -m32 -ffreestanding -static -no-pie -fno-stack-protector -nostdlib -O2 -T boot/kernel.ld boot/sigma_kernel_core.c -o boot/sigma_kernel.elf
	@echo "==> Kernel built successfully: boot/sigma_kernel.elf"

# 7c. Build native workstation binary
bin/sigmaos: src/bin/sigmaos.rs
	@echo "==> Building native SigmaOS Workstation binary..."
	@rustc -O src/bin/sigmaos.rs -o bin/sigmaos

# 7d. Build native launcher controller
bin/sigmaos-launch: src/bin/sigmaos_launch.rs
	@echo "==> Building native SigmaOS Launch controller..."
	@rustc -O src/bin/sigmaos_launch.rs -o bin/sigmaos-launch

# 8. Build bootable QEMU / ISO image (M1 milestone)
iso: kernel
	@echo "==> Building SigmaOS Desktop Preview ISO image..."
	@mkdir -p build
	@bash scripts/build_iso.sh build/sigmaos-desktop-preview.iso

# 8a. QEMU Boot Smoke Test (Vertical Slice Stage 1)
qemu-smoke: kernel
	@mkdir -p build
	@bash scripts/build_stub_iso.sh build/sigmaos.iso
	@bash scripts/ci/qemu_boot_smoke.sh

# 8b. Build minimal stub ISO (GALL'S LAW: working simple > broken complex)
# Uses GRUB2 + genisoimage; no nasm required. Boots immediately in QEMU.
iso-stub: kernel
	@echo "==> Building SigmaOS Stub ISO (GRUB2-based, with Multiboot kernel)..."
	@mkdir -p build
	@bash scripts/build_stub_iso.sh

# 8c. Build standalone UEFI boot image (FAT32 EFI System Partition)
boot-img:
	@echo "==> Synthesizing SigmaOS Bare-Metal UEFI Boot Image..."
	@bash scripts/build_standalone_boot_image.sh

# 9. Run SigmaOS (boots in QEMU if installed; gracefully falls back to native workstation)
run: iso-stub bin/sigmaos bin/sigmaos-launch
	@image=build/sigmaos-desktop-preview.iso; \
	if command -v qemu-system-x86_64 >/dev/null 2>&1 && [ -s "$$image" ]; then \
		echo "==> Booting SigmaOS Desktop Preview in QEMU..."; \
		exec qemu-system-x86_64 -m 2048 -cdrom "$$image" -serial stdio -no-reboot; \
	else \
		echo "==> QEMU not found in PATH or running headlessly. Launching native SigmaOS Workstation..."; \
		./bin/sigmaos-launch; \
	fi

# 9b. Run stub ISO in QEMU (GALL'S LAW: start simple — works now!)
run-stub: iso-stub
	@echo "==> Booting SigmaOS stub ISO in QEMU..."
	@bash scripts/run_qemu.sh

# 10. Launch readiness check (INSTRUMENTATION + FAIL-FAST)
launch-check:
	@bash scripts/check_launch_ready.sh

# 11. Interactive OS launch & workstation dashboard (READY TO WORK & LAUNCH)
launch: bin/sigmaos bin/sigmaos-launch
	@./bin/sigmaos-launch

start: launch

# 12. Distro personas
mint:
	@bash scripts/launch_mint_persona.sh

arch:
	@bash scripts/launch_arch_persona.sh

omarchy:
	@bash scripts/launch_omarchy_persona.sh

# 13. Graphical Desktop Launcher
gui:
	@python3 scripts/sigmaos_gui.py

desktop: gui

# 13a. System tray / status indicator
tray:
	@python3 scripts/sigmaos_tray.py &

# 13b. Pure GTK3 Native Desktop
native:
	@python3 scripts/sigmaos_native_app.py

# 14. Install SigmaOS desktop entry (.desktop file + icon in ~/.local)
install:
	@bash scripts/install_desktop_entry.sh

# 15. Run startup diagnostics
diagnose:
	@python3 scripts/sigmaos_startup_diagnostics.py

# 15a. Omarchy System Doctor (39-point comprehensive audit)
doctor:
	@bash scripts/sigmaos_doctor.sh

# 16. Install system fonts
fonts:
	@mkdir -p $(HOME)/.local/share/fonts
	@if [ ! -f $(HOME)/.local/share/fonts/JetBrainsMono-Regular.ttf ]; then \
		echo "==> Fetching JetBrains Mono font..."; \
		curl -Lfs "https://github.com/JetBrains/JetBrainsMono/raw/master/fonts/ttf/JetBrainsMono-Regular.ttf" -o $(HOME)/.local/share/fonts/JetBrainsMono-Regular.ttf 2>/dev/null || true; \
	fi
	@if [ ! -f $(HOME)/.local/share/fonts/SymbolsNerdFont-Regular.ttf ]; then \
		echo "==> Fetching Symbols Nerd Font..."; \
		curl -Lfs "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf" -o $(HOME)/.local/share/fonts/SymbolsNerdFont-Regular.ttf 2>/dev/null || true; \
	fi
	@echo "==> Updating font cache..."
	@fc-cache -fv $(HOME)/.local/share/fonts 2>/dev/null || true

# 16a. Launch Isolated Sandbox Container (Bubblewrap GUI passthrough)
sandbox:
	@bash scripts/sigmaos_sandbox_container.sh $(ARGS)

# 16b. Deploy Omarchy modular dotfiles to ~/.config
dotfiles:
	@bash scripts/setup_omarchy_dotfiles.sh $(ARGS)

# 16c. System Settings Manager
settings:
	@bash bin/sigmaos-settings $(ARGS)

# 17. Install GUI dependencies
install-deps:
	@echo "==> Ensuring GUI dependencies are installed..."
	@sudo apt-get install -y python3-gi python3-gi-cairo gir1.2-webkit2-4.1 gir1.2-gtk-3.0 fonts-jetbrains-mono 2>/dev/null || true

help:
	@echo "SigmaOS Unified Build & Test Interface"
	@echo "  make check        - Static analysis via cargo check (timed)"
	@echo "  make check-fast   - Cached quick-lane cargo check"
	@echo "  make test         - Execute native standalone test suites (timed)"
	@echo "  make test-fast    - Memory-safe cargo unit tests (timed)"
	@echo "  make bench        - Benchmarks on thin-LTO release-fast profile"
	@echo "  make fmt          - Verify formatting style (strict)"
	@echo "  make registry-audit - Audit managed component checkouts and metadata (read-only)"
	@echo "  make hub          - Component-repo hub help (connect/create/upgrade/sync)"
	@echo "  make hub-list     - List components + local git status (FILTER=--category X)"
	@echo "  make hub-status   - Fleet or per-repo status (NAME=SigmaOS-foo)"
	@echo "  make hub-audit    - Count COMPONENT.toml coverage / upgrade candidates"
	@echo "  make hub-create   - Scaffold new component repo (NAME=..., ARGS=--desc ... --dry-run)"
	@echo "  make hub-upgrade  - Bring repos up to component standard (NAME=--all)"
	@echo "  make hub-sync     - Pull/push component checkouts (ARGS=--push)"
	@echo "  make hub-search   - Search components (ARGS=<regex>)"
	@echo "  make hub-info     - Show component metadata (NAME=SigmaOS-foo)"
	@echo "  make hub-depends  - Show component deps (NAME=SigmaOS-foo)"
	@echo "  make hub-families - Scaffold-fleet consolidation analyzer"
	@echo "  make hub-consolidate - Dry-run merge plan (ARGS=<family>)"
	@echo "  make hub-test     - cargo test across fleet (ARGS=<name|--local>)"
	@echo "  make hub-health   - Fleet-health summary (coverage/scaffold/git)"
	@echo "  make fmt-fix      - Auto-apply rustfmt across workspace"
	@echo "  make iso          - Full Desktop ISO build (GRUB2 + initramfs + installer) ✓"
	@echo "  make iso-stub     - Stub ISO build (GRUB2, boots NOW in QEMU) ✓"
	@echo "  make boot-img     - Standalone UEFI ESP image (FAT32 BOOTX64.EFI) ✓"
	@echo "  make run          - Boot desktop ISO in QEMU / native workstation ✓"
	@echo "  make run-stub     - Boot stub ISO in QEMU ✓"
	@echo "  make launch       - Launch interactive SigmaOS Workstation Dashboard ✓"
	@echo "  make launch-check - Full launch readiness validation"
	@echo "  make mint         - Launch Linux Mint Ergonomic Persona ✓"
	@echo "  make arch         - Launch Arch Linux Minimalist Persona ✓"
	@echo "  make omarchy      - Launch Omarchy Linux Aesthetic Persona ✓"
	@echo "  make gui          - Launch SigmaOS Graphical Desktop (GTK3 + WebKit2) ✓"
	@echo "  make desktop      - Alias for 'make gui' ✓"
	@echo "  make native       - Launch SigmaOS Native GTK3 Desktop (pure GTK3) ✓"
	@echo "  make tray         - Launch SigmaOS System Tray indicator ✓"
	@echo "  make install      - Install .desktop entry + icon to ~/.local ✓"
	@echo "  make diagnose     - Run SigmaOS startup health diagnostics ✓"
	@echo "  make doctor       - Run Omarchy 39-point system health diagnostics ✓"
	@echo "  make fonts        - Update local font cache ✓"
	@echo "  make install-deps - Install GTK3/WebKit2 GUI dependencies ✓"
	@echo "  make clean        - Remove build artifacts"
