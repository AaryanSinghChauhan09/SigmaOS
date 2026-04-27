# ==============================================================================
# Σ SIGMAOS: INDUSTRIAL BUILD ORCHESTRATOR (v6.4)
# ==============================================================================

CC = gcc
AS = nasm
LD = ld

CFLAGS = -m64 -ffreestanding -O2 -Wall -Wextra -Iinclude -nostdlib -fno-stack-protector
LDFLAGS = -T kernel/sigma.ld -m elf_x86_64

# Modular Directories
SRC_DIR  = kernel
CORE_DIR = kernel/core
DIAG_DIR = kernel/diagnostics
ORCH_DIR = kernel/orchestration
SHELL_DIR = kernel/shell
OBJ_DIR  = obj

# Objects
OBJS = $(OBJ_DIR)/main.o \
       $(OBJ_DIR)/scheduler.o \
       $(OBJ_DIR)/pmm.o \
       $(OBJ_DIR)/vmm.o \
       $(OBJ_DIR)/vmm_perf.o \
       $(OBJ_DIR)/idt.o \
       $(OBJ_DIR)/vfs.o \
       $(OBJ_DIR)/hotpatch.o \
       $(OBJ_DIR)/slab.o \
       $(OBJ_DIR)/input_queue.o \
       $(OBJ_DIR)/vga.o \
       $(OBJ_DIR)/keyboard.o \
       $(OBJ_DIR)/serial.o \
       $(OBJ_DIR)/multi_core.o \
       $(OBJ_DIR)/summoner.o \
       $(OBJ_DIR)/neural_lattice.o \
       $(OBJ_DIR)/zenith_gui.o \
       $(OBJ_DIR)/ids_shard.o \
       $(OBJ_DIR)/firewall_shard.o \
       $(OBJ_DIR)/net_buf.o \
       $(OBJ_DIR)/e1000.o \
       $(OBJ_DIR)/ide.o \
       $(OBJ_DIR)/sigmafs.o

all: sigmaos.bin

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(CORE_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(DIAG_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(ORCH_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(SHELL_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(DRV_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(FS_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

sigmaos.bin: $(OBJS)
	$(LD) $(LDFLAGS) $(OBJS) -o sigmaos.bin

clean:
	rm -rf $(OBJ_DIR) sigmaos.bin

# Verification
verify:
	@echo "Σ [BUILD]: Verifying Multiboot Compliance..."
	@grub-file --is-x86-multiboot sigmaos.bin && echo "Multiboot OK" || echo "Fail"
