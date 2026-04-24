# ============================================================
# SigmaOS Sovereign Makefile
# Convenience wrapper around the Native build orchestrator.
# All targets are architecture-aware and capsule-driven.
# ============================================================

ARCH      ?= x86_64
BUILDER   := ./s-cli
ISO_SCRIPT := scripts/build_iso.sh
BUILD_DIR := build

.PHONY: all x86_64 aarch64 riscv64 iso clean run test help

# Default target
all: x86_64

## ── Architecture Targets ──────────────────────────────────

x86_64:
	@echo "[*] Building SigmaOS for x86_64..."
	@$(BUILDER) build x86_64

aarch64:
	@echo "[*] Building SigmaOS for ARM64..."
	@$(BUILDER) build aarch64

riscv64:
	@echo "[*] Building SigmaOS for RISC-V 64..."
	@$(BUILDER) build riscv64

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
	@$(BUILDER) clean
	@echo "[+] Clean complete."

## ── Module Graph ─────────────────────────────────────────

graph:
	@echo "[*] Printing module dependency graph..."
	@$(BUILDER) list

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
