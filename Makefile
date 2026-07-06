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
	mkdir -p $(BOOT_DIR)/grub
	mkdir -p $(EFI_DIR)
	
	# Copy kernel to boot directory
	cp $(KERNEL_DIR)/target/x86_64-sigmaos/debug/sigmaos.elf $(BOOT_DIR)/sigma-kernel.elf
	
	# Copy UEFI bootloader
	cp sigma-boot/BOOTX64.EFI $(EFI_DIR)/BOOTX64.EFI
	
	# Create GRUB config
	@echo "Creating GRUB configuration..."
	@echo "set timeout=5" > $(BOOT_DIR)/grub/grub.cfg
	@echo "set default=0" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "menuentry \"SigmaOS v16.0 Apex (UEFI)\" {" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "    multiboot2 /boot/sigma-kernel.elf" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "    boot" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "}" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "menuentry \"SigmaOS v16.0 Apex (UEFI - Safe Mode)\" {" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "    multiboot2 /boot/sigma-kernel.elf safe_mode=1" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "    boot" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "}" >> $(BOOT_DIR)/grub/grub.cfg
	
	# Create ISO with xorriso (UEFI-only)
	xorriso -as mkisofs \
		-r -J \
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
