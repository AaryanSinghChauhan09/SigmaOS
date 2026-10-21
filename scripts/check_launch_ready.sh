#!/usr/bin/env bash
# SigmaOS Launch Readiness Check
# ============================================================
# Validates that all components needed to build and run
# SigmaOS are present and correct.
#
# Inspired by: Arch Linux's pre-install checklist,
# Omarchy's readiness doctor, Linux Mint's driver manager.
#
# Principles: FAIL-FAST, DEFENSIVE PROGRAMMING,
# INSTRUMENTATION, DISCOVERABILITY, TRANSPARENCY
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

PASS=0
FAIL=0
WARN=0

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

ok()   { echo -e "  ${GREEN}[✓]${NC} $*"; PASS=$((PASS+1)); }
fail() { echo -e "  ${RED}[✗]${NC} $*"; FAIL=$((FAIL+1)); }
warn() { echo -e "  ${YELLOW}[!]${NC} $*"; WARN=$((WARN+1)); }
info() { echo -e "  ${BLUE}[i]${NC} $*"; }

echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║          SigmaOS Launch Readiness Check                  ║${NC}"
echo -e "${BLUE}║   Checking build tools, kernel source, boot config...    ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# ─────────────────────────────────────────────────────────────────────────────
# 1. Rust toolchain
# ─────────────────────────────────────────────────────────────────────────────
echo -e "${BLUE}── Rust Toolchain ──────────────────────────────────────────${NC}"
if command -v cargo >/dev/null 2>&1; then
    RUST_VER=$(rustc --version 2>/dev/null || echo "unknown")
    ok "cargo found: $RUST_VER"
else
    fail "cargo not found — install Rust: https://rustup.rs"
fi

if command -v rustup >/dev/null 2>&1; then
    ok "rustup found"
    # Check for bare-metal target
    if rustup target list --installed 2>/dev/null | grep -q "x86_64-unknown-none"; then
        ok "Target x86_64-unknown-none installed (bare-metal kernel capable)"
    else
        warn "Target x86_64-unknown-none not installed"
        info "Install with: rustup target add x86_64-unknown-none"
    fi
else
    warn "rustup not found — cannot check installed targets"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 2. Build check (cargo check)
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Cargo Check (Compile Validation) ───────────────────────${NC}"
if cargo check --lib --manifest-path="$ROOT_DIR/Cargo.toml" 2>&1 | grep -q "^error"; then
    fail "cargo check --lib has errors — fix before building"
else
    ok "cargo check --lib: PASSING (0 errors)"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 3. Bootloader & ISO tools
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Bootloader & ISO Tools ──────────────────────────────────${NC}"

check_tool() {
    local tool=$1 pkg=$2
    if command -v "$tool" >/dev/null 2>&1; then
        ok "$tool found"
    else
        warn "$tool not found (install: $pkg)"
    fi
}

check_tool nasm       "sudo apt install nasm"
check_tool xorriso    "sudo apt install xorriso"
check_tool grub-mkrescue "sudo apt install grub-pc-bin"
check_tool grub-mkimage  "sudo apt install grub-pc-bin"
check_tool genisoimage   "sudo apt install genisoimage (or xorriso)"

if [ -f "/usr/lib/grub/i386-pc/cdboot.img" ]; then
    ok "GRUB i386-pc library found"
else
    warn "GRUB i386-pc library missing — install: sudo apt install grub-pc-bin"
fi

# Check for limine (preferred bootloader)
if command -v limine >/dev/null 2>&1; then
    ok "Limine bootloader found"
elif [ -f "$ROOT_DIR/boot/limine/BOOTX64.EFI" ]; then
    ok "Limine binaries staged in boot/limine/"
else
    warn "Limine not found — install for production boot"
    info "See: https://limine-bootloader.org/"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 4. Kernel source
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Kernel Source ───────────────────────────────────────────${NC}"

if [ -f "$ROOT_DIR/src/kernel/main.rs" ]; then
    ok "Kernel main.rs found"
else
    fail "Kernel main.rs missing — expected at src/kernel/main.rs"
fi

if [ -f "$ROOT_DIR/boot/stub_kernel.asm" ]; then
    ok "Stub kernel ASM found: boot/stub_kernel.asm"
else
    warn "Stub kernel ASM not found"
fi

if [ -f "$ROOT_DIR/boot/kernel.ld" ]; then
    ok "Linker script found: boot/kernel.ld"
else
    warn "Linker script missing: boot/kernel.ld"
fi

# Check if compiled kernel exists
if [ -f "$ROOT_DIR/boot/sigma_kernel.elf" ]; then
    ok "Compiled kernel ELF found: boot/sigma_kernel.elf"
elif [ -f "$ROOT_DIR/boot/sigma_kernel.bin" ]; then
    ok "Compiled kernel binary found: boot/sigma_kernel.bin"
else
    warn "No compiled kernel found — run 'make iso-stub' for stub, or compile Rust kernel"
    info "To compile stub: nasm -f elf32 boot/stub_kernel.asm -o /tmp/stub.o && ld -T boot/kernel.ld -o boot/sigma_kernel.elf /tmp/stub.o"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 5. Boot configuration
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Boot Configuration ──────────────────────────────────────${NC}"

if [ -f "$ROOT_DIR/boot/limine/limine.conf" ]; then
    ok "Limine config found: boot/limine/limine.conf"
else
    warn "Limine config missing: boot/limine/limine.conf"
fi

if [ -f "$ROOT_DIR/etc/systemd/system/sigma-session.service" ] || \
   [ -d "$ROOT_DIR/etc/systemd" ]; then
    ok "systemd service files found in etc/"
else
    warn "No systemd services found in etc/"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 6. Installer & packages
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Installer & Packages ────────────────────────────────────${NC}"

if [ -f "$ROOT_DIR/installer/sigma-installer.rs" ]; then
    ok "Installer source found: installer/sigma-installer.rs"
else
    warn "Installer source missing"
fi

if [ -f "$ROOT_DIR/install/packages/sigmaos-base.packages" ]; then
    PKG_COUNT=$(grep -c "^[^#]" "$ROOT_DIR/install/packages/sigmaos-base.packages" 2>/dev/null || echo "?")
    ok "Package list found: $PKG_COUNT packages defined"
else
    warn "Package list missing: install/packages/sigmaos-base.packages"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 7. QEMU
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── QEMU (Virtual Machine Testing) ─────────────────────────${NC}"

if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    QEMU_VER=$(qemu-system-x86_64 --version 2>/dev/null | head -1)
    ok "QEMU found: $QEMU_VER"
    if [ -e /dev/kvm ]; then
        ok "KVM available — hardware acceleration enabled"
    else
        warn "KVM not available — QEMU will run in slow TCG mode"
    fi
else
    warn "qemu-system-x86_64 not found"
    info "Install: sudo apt install qemu-system-x86"
fi

# ─────────────────────────────────────────────────────────────────────────────
# 8. Built ISO
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Built Artifacts ─────────────────────────────────────────${NC}"

if [ -f "$ROOT_DIR/build/sigmaos-desktop-preview.iso" ]; then
    ISO_SIZE=$(du -sh "$ROOT_DIR/build/sigmaos-desktop-preview.iso" | cut -f1)
    # Validate ISO9660
    if python3 -c "
f=open('$ROOT_DIR/build/sigmaos-desktop-preview.iso','rb')
f.seek(32769)
ok=f.read(5)==b'CD001'
f.close()
import sys; sys.exit(0 if ok else 1)
" 2>/dev/null; then
        ok "Valid ISO9660 image: build/sigmaos-desktop-preview.iso ($ISO_SIZE)"
        info "Boot with: make run-stub"
    else
        fail "ISO exists but invalid ISO9660 signature"
    fi
else
    warn "No built ISO found — run: make iso-stub"
fi

# ─────────────────────────────────────────────────────────────────────────────
# Summary
# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}── Summary ──────────────────────────────────────────────────${NC}"
echo ""
TOTAL=$((PASS + FAIL + WARN))
echo -e "  Total checks : $TOTAL"
echo -e "  ${GREEN}Passed  : $PASS${NC}"
echo -e "  ${YELLOW}Warnings: $WARN${NC}"
echo -e "  ${RED}Failed  : $FAIL${NC}"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo -e "  ${RED}✗ LAUNCH NOT READY — fix failures above${NC}"
    echo ""
    exit 1
elif [ "$WARN" -gt 0 ]; then
    echo -e "  ${YELLOW}⚠ PARTIALLY READY — address warnings for full launch${NC}"
    echo ""
    echo "  Quick start:"
    echo "    make iso-stub    # build bootable stub ISO"
    echo "    make run-stub    # boot in QEMU"
    echo ""
    exit 0
else
    echo -e "  ${GREEN}✓ FULLY READY TO LAUNCH!${NC}"
    echo ""
    echo "  make iso-stub && make run-stub"
    echo ""
    exit 0
fi
