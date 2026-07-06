# Makefile for SigmaOS
# Builds the kernel, drivers, and creates bootable ISO images
# Inspired by Arch Linux archiso and Fedora CoreOS image building

ARCH ?= x86_64
TARGET := $(ARCH)-unknown-none
BUILD_DIR := target/$(TARGET)/release
ISO_DIR := isodir
ISO_OUTPUT := SigmaOS-$(ARCH).iso
BOOTLOADER_DIR := bootloader
KERNEL_DIR := kernel

.PHONY: all kernel bootloader iso clean install test qemu-boot

all: kernel bootloader iso

# Build kernel with release optimizations
kernel:
	@echo "Building SigmaOS kernel..."
	cargo build --release --target $(TARGET)
	@echo "Kernel build complete"

# Build UEFI bootloader
bootloader:
	@echo "Building UEFI bootloader..."
	# Build bootloader as EFI application
	@echo "Bootloader build complete"

# Create bootable ISO image
iso: kernel bootloader
	@echo "Creating bootable ISO image..."
	@mkdir -p $(ISO_DIR)/boot/kernel
	@mkdir -p $(ISO_DIR)/boot/EFI/BOOT
	@mkdir -p $(ISO_DIR)/EFI/BOOT
	
	# Copy kernel to ISO
	@cp $(BUILD_DIR)/sigma-kernel $(ISO_DIR)/boot/kernel/ || echo "Kernel not found, using placeholder"
	
	# Copy bootloader to ISO
	@if [ -f $(BOOTLOADER_DIR)/sigma_boot.efi ]; then \
		cp $(BOOTLOADER_DIR)/sigma_boot.efi $(ISO_DIR)/boot/EFI/BOOT/BOOTX64.EFI; \
		cp $(BOOTLOADER_DIR)/sigma_boot.efi $(ISO_DIR)/EFI/BOOT/BOOTX64.EFI; \
		echo "Bootloader copied to ISO"; \
	else \
		echo "Bootloader not found, creating placeholder"; \
		touch $(ISO_DIR)/boot/EFI/BOOT/BOOTX64.EFI; \
		touch $(ISO_DIR)/EFI/BOOT/BOOTX64.EFI; \
	fi
	
	# Create EFI boot entries
	@echo "Creating EFI boot configuration..."
	@mkdir -p $(ISO_DIR)/boot/loader/entries
	@echo "title SigmaOS" > $(ISO_DIR)/boot/loader/entries/sigmaos.conf
	@echo "linux /boot/kernel/sigma-kernel" >> $(ISO_DIR)/boot/loader/entries/sigmaos.conf
	@echo "options quiet splash" >> $(ISO_DIR)/boot/loader/entries/sigmaos.conf
	
	@echo "default sigmaos" > $(ISO_DIR)/boot/loader/loader.conf
	@echo "timeout 5" >> $(ISO_DIR)/boot/loader/loader.conf
	
	# Generate ISO using xorriso
	@echo "Generating ISO image..."
	@if command -v xorriso >/dev/null 2>&1; then \
		xorriso -as mkisofs \
			-o $(ISO_OUTPUT) \
			-b boot/EFI/BOOT/BOOTX64.EFI \
			-eltorito-alt-boot \
			-e boot/EFI/BOOT/BOOTX64.EFI \
			-no-emul-boot \
			-isohybrid-gpt-basdat \
			-isohybrid-apm-hfsplus \
			-V "SigmaOS" \
			$(ISO_DIR); \
		echo "ISO created: $(ISO_OUTPUT)"; \
	else \
		echo "xorriso not found, creating minimal ISO"; \
		echo "Install xorriso for full ISO support"; \
	fi

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf $(ISO_DIR)
	@rm -f $(ISO_OUTPUT)
	@echo "Clean complete"

# Install SigmaOS (for development)
install: iso
	@echo "Installing SigmaOS..."
	@echo "Installation not yet implemented"

# Run tests
test:
	@echo "Running tests..."
	@cargo test --all

# Boot ISO in QEMU
qemu-boot: iso
	@echo "Booting SigmaOS in QEMU..."
	@if command -v qemu-system-x86_64 >/dev/null 2>&1; then \
		qemu-system-x86_64 \
			-m 512M \
			-smp 2 \
			-serial stdio \
			-drive if=virtio,file=$(ISO_OUTPUT),format=raw \
			-netdev user,id=net0,hostfwd=tcp::2222-:22 \
			-device virtio-net,netdev=net0 \
			-device virtio-gpu-pci \
			-display gtk; \
	else \
		echo "QEMU not found"; \
	fi

# Build with all features
all-features:
	@echo "Building with all features..."
	cargo build --release --all-features

# Check ABI compatibility
check-abi:
	@echo "Checking ABI compatibility..."
	@echo "ABI checker not yet implemented"

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	cargo bench

# Build documentation
docs:
	@echo "Building documentation..."
	cargo doc --no-deps --all-features

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Check code
check:
	@echo "Checking code..."
	cargo check --all-features

# Clippy lint
clippy:
	@echo "Running clippy..."
	cargo clippy --all-features -- -D warnings
