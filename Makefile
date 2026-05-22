# =========================================================================
# SIGMAOS: INDUSTRIAL KERNEL MAKEFILE (v15.0 - ZENITH)
# =========================================================================

CC = x86_64-linux-gnu-gcc
CXX = x86_64-linux-gnu-g++
LD = x86_64-linux-gnu-ld
ASM = nasm

CFLAGS = -Iinclude -ffreestanding -mno-red-zone -Wall -Wextra -O2 -fno-pie
CXXFLAGS = $(CFLAGS) -fno-exceptions -fno-rtti -std=c++17
ASMFLAGS = -f elf64

BUILD_DIR = build
ISO_DIR = $(BUILD_DIR)/iso
KERNEL_BIN = $(BUILD_DIR)/sigmaos.bin
ISO_IMAGE = $(BUILD_DIR)/sigmaos.iso

# Directories to search for source files
SRC_DIRS := kernel/core kernel/core/drivers/input kernel/core/memory kernel/core/sched kernel/core/system kernel/core/syscall kernel/core/hal kernel/core/vulkan kernel/net kernel/storage kernel/telemetry tools usr init fs net lib/libc
C_SRCS := $(shell find $(SRC_DIRS) -name '*.c')
CXX_SRCS := $(shell find $(SRC_DIRS) -name '*.cpp')
ASM_SRCS := $(shell find $(SRC_DIRS) -name '*.asm')

# Object files
OBJS := $(patsubst %.c, $(BUILD_DIR)/%.o, $(C_SRCS)) \
        $(patsubst %.cpp, $(BUILD_DIR)/%.o, $(CXX_SRCS)) \
        $(patsubst %.asm, $(BUILD_DIR)/%.o, $(ASM_SRCS))

.PHONY: all clean iso qemu

all: iso

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.asm
	@mkdir -p $(dir $@)
	$(ASM) $(ASMFLAGS) $< -o $@

$(KERNEL_BIN): $(OBJS)
	# Link using proper linker script
	$(LD) -n -T linker.ld -o $@ $^

iso: $(KERNEL_BIN)
	@mkdir -p $(ISO_DIR)/boot/grub
	@cp $(KERNEL_BIN) $(ISO_DIR)/boot/
	@echo "menuentry 'SigmaOS Zenith' {" > $(ISO_DIR)/boot/grub/grub.cfg
	@echo "    multiboot /boot/sigmaos.bin" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "    boot" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "}" >> $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO_IMAGE) $(ISO_DIR)

qemu: iso
	qemu-system-x86_64 -cdrom $(ISO_IMAGE) -serial stdio -m 2G

clean:
	rm -rf $(BUILD_DIR)
CONFIG_MONOLITHIC_DRIVERS=0
CONFIG_MICROKERNEL=1
