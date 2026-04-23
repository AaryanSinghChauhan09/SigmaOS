# ============================================================
# SigmaOS Sovereign Makefile
# Convenience wrapper around the Python build orchestrator.
# All targets are architecture-aware and capsule-driven.
# ============================================================

ARCH      ?= x86_64
PYTHON    := python3
BUILDER   := scripts/sovereign_builder.py
ISO_SCRIPT := scripts/build_iso.sh
BUILD_DIR := build/$(ARCH)

.PHONY: all x86_64 aarch64 riscv64 iso clean run test help

# Default target
all: x86_64

## ── Architecture Targets ──────────────────────────────────

x86_64:
	@echo "[*] Building SigmaOS for x86_64..."
	@$(PYTHON) $(BUILDER) x86_64

aarch64:
	@echo "[*] Building SigmaOS for ARM64..."
	@$(PYTHON) $(BUILDER) aarch64

riscv64:
	@echo "[*] Building SigmaOS for RISC-V 64..."
	@$(PYTHON) $(BUILDER) riscv64

## ── Image Packaging ──────────────────────────────────────

iso:
	@echo "[*] Building bootable ISO for $(ARCH)..."
	@bash $(ISO_SCRIPT) $(ARCH)

## ── QEMU Smoke Testing ───────────────────────────────────

run: iso
	@echo "[*] Booting SigmaOS in QEMU ($(ARCH))..."
	qemu-system-$(ARCH) \
		-cdrom $(BUILD_DIR)/sigmaos_$(ARCH).iso \
		-m 512M \
		-serial stdio \
		-no-reboot \
		-display none

## ── Module Scaffolding ───────────────────────────────────

scaffold:
	@echo "Usage: make scaffold-<module_name>-<type>"
scaffold-%:
	$(eval PARTS := $(subst -, ,$*))
	@bash scripts/scaffold_module.sh $(word 1,$(PARTS)) $(word 2,$(PARTS))

## ── Cleanup ──────────────────────────────────────────────

clean:
	@echo "[*] Cleaning build artifacts..."
	@rm -rf build/
	@find modules/ -name "*.o" -delete
	@echo "[+] Clean complete."

## ── Module Graph ─────────────────────────────────────────

graph:
	@echo "[*] Printing module dependency graph..."
	@$(PYTHON) -c "\
import json, os; \
[print(m['module'], '->', m.get('dependencies',[])) \
 for root,_,files in os.walk('modules') \
 if 'module.json' in files \
 for m in [json.load(open(os.path.join(root,'module.json')))]]"

## ── Help ─────────────────────────────────────────────────

help:
	@echo ""
	@echo "  SigmaOS Sovereign Build System"
	@echo "  ================================"
	@echo "  make                - Build for x86_64"
	@echo "  make aarch64        - Build for ARM64"
	@echo "  make riscv64        - Build for RISC-V"
	@echo "  make iso            - Package bootable ISO"
	@echo "  make run            - Boot in QEMU"
	@echo "  make scaffold-name-type - Scaffold new module"
	@echo "  make graph          - Show module dep graph"
	@echo "  make clean          - Remove build artifacts"
	@echo ""
