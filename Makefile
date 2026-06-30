.PHONY: all build run clean

KERNEL_DIR = kernel

all: build

build:
	@echo "Building SigmaOS Kernel..."
	cd $(KERNEL_DIR) && cargo build --target x86_64-sigmaos.json

run: build
	@echo "Running SigmaOS in QEMU..."
	cd $(KERNEL_DIR) && cargo run

clean:
	@echo "Cleaning build artifacts..."
	cd $(KERNEL_DIR) && cargo clean
