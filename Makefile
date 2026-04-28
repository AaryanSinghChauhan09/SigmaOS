# ==============================================================================
# Σ SIGMAOS: INDUSTRIAL BUILD ORCHESTRATOR (v6.5 - MODULAR ZENITH)
# ==============================================================================

CC = gcc
CXX = g++
AS = nasm
LD = ld

CFLAGS = -m64 -ffreestanding -O2 -Wall -Wextra -Iinclude -nostdlib -fno-stack-protector
CXXFLAGS = $(CFLAGS) -fno-exceptions -fno-rtti
LDFLAGS = -T kernel/sigma.ld -m elf_x86_64

# Modular Directories
SRC_DIRS = kernel/core kernel/drivers kernel/orchestration kernel/shards userland
OBJ_DIR = obj

# Discover all source files
C_SRCS   = $(shell find $(SRC_DIRS) -name "*.c")
CXX_SRCS = $(shell find $(SRC_DIRS) -name "*.cpp")
ASM_SRCS = $(shell find $(SRC_DIRS) -name "*.asm")

# Generate object file paths
OBJS = $(C_SRCS:%.c=$(OBJ_DIR)/%.o) \
       $(CXX_SRCS:%.cpp=$(OBJ_DIR)/%.o) \
       $(ASM_SRCS:%.asm=$(OBJ_DIR)/%.o)

.PHONY: all clean kernel

all: kernel

kernel: $(OBJ_DIR)/sigma_os_master

$(OBJ_DIR)/sigma_os_master: $(OBJS)
	@echo "Σ [LINK]: Finalizing Sovereign Lattice..."
	$(LD) $(LDFLAGS) -o $@ $(OBJS)

$(OBJ_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@echo "Σ [C]: Compiling $<..."
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: %.cpp
	@mkdir -p $(dir $@)
	@echo "Σ [C++]: Compiling $<..."
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: %.asm
	@mkdir -p $(dir $@)
	@echo "Σ [ASM]: Assembling $<..."
	$(AS) -f elf64 $< -o $@

clean:
	rm -rf $(OBJ_DIR)
