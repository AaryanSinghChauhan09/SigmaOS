.PHONY: all build run clean iso

KERNEL_DIR = kernel
ISO_DIR = iso
BOOT_DIR = $(ISO_DIR)/boot
EFI_DIR = $(ISO_DIR)/EFI/BOOT

all: build

build:
	@echo "Building SigmaOS Kernel..."
	cd $(KERNEL_DIR) && cargo build --target x86_64-sigmaos.json

# Build UEFI bootloader
bootloader:
	@echo "Building UEFI bootloader..."
	cd sigma-boot && zig build-exe -target x86_64-uefi -femit-bin=BOOTX64.EFI -fno-strip

# Create bootable ISO
iso: build bootloader
	@echo "Creating bootable ISO..."
	mkdir -p $(BOOT_DIR)
	mkdir -p $(EFI_DIR)
	
	# Copy kernel to boot directory
	cp $(KERNEL_DIR)/target/x86_64-sigmaos/debug/sigmaos.elf $(BOOT_DIR)/sigma-kernel.elf
	
	# Copy UEFI bootloader
	cp sigma-boot/BOOTX64.EFI $(EFI_DIR)/BOOTX64.EFI
	
	# Create ISO with xorriso
	xorriso -as mkisofs \
		-r -J -b boot/grub/grub.cfg \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		-eltorito-alt-boot -e EFI/BOOT/BOOTX64.EFI -no-emul-boot \
		-o sigmaos.iso $(ISO_DIR)

run: build
	@echo "Running SigmaOS in QEMU..."
	./qemu-boot.sh standalone

run-iso: iso
	@echo "Running SigmaOS ISO in QEMU..."
	qemu-system-x86_64 -cdrom sigmaos.iso -m 2G -smp 2 -enable-kvm

clean:
	@echo "Cleaning build artifacts..."
	cd $(KERNEL_DIR) && cargo clean
	cd sigma-boot && zig clean
	rm -rf $(ISO_DIR)
	rm -f sigmaos.iso
