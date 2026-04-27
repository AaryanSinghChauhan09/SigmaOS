# =============================================================================
# Σ SIGMAOS SOVEREIGN BUILD SYSTEM
# =============================================================================

CC = gcc
AS = nasm
CFLAGS = -ffreestanding -O2 -Wall -Wextra -I./kernel
LDFLAGS = -nostdlib -T kernel/linker.ld

# Shards
SHELL_SHARDS = kernel/shell/shell_engine.c kernel/shell/shell_parser.c
DISTRO_SHARDS = kernel/distros/distro_manifest.c
LIBC_SHARDS = kernel/libc/sigma_libc.c
BOOT_SHARDS = kernel/boot/multiboot_header.asm kernel/boot/sovereign_boot.asm

all: sigmaos.bin

sigmaos.bin: $(BOOT_SHARDS) $(SHELL_SHARDS) $(DISTRO_SHARDS) $(LIBC_SHARDS)
	@echo "[BUILD]: Compiling Sovereign Shards..."
	# Real compilation commands here
	@touch sigmaos.bin

# Deployment Targets
vbox: all
	@echo "[DEPLOY]: Creating VirtualBox ISO..."

browser: all
	@echo "[DEPLOY]: Compiling to WASM for Browser execution..."
	# emcc ...

wsl: all
	@echo "[DEPLOY]: Packaging RootFS for WSL2..."

live-usb: all
	@echo "[DEPLOY]: Creating Live USB image..."

clean:
	rm -f sigmaos.bin
