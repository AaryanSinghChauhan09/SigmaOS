# Makefile for SigmaOS
# Builds the kernel, drivers, and runs tests

ARCH ?= x86_64
TARGET := $(ARCH)-unknown-none
BUILD_DIR := target/$(TARGET)/debug

.PHONY: all kernel drivers check-abi iso-ci bench clean

all: kernel drivers

kernel:
	cargo build --target $(TARGET)

drivers:
	cargo build --features "all-drivers"

check-abi:
	@echo "Checking ABI compatibility..."
	# STUB: Run abi-checker

iso-ci: kernel
	@echo "Building bootable ISO for CI..."
	# STUB: Run xorriso

bench:
	@echo "Running benchmarks..."
	cargo bench

clean:
	cargo clean
