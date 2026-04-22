/*
 * shards/virtualization/vm.c — Sovereign VM Core
 * Minimal opcode interpreter — no libc beyond write() syscall.
 * Implements a 16-register stack machine with DMA-mapped memory.
 */

#include <stdint.h>
#include <stddef.h>

/* ── Opcodes ─────────────────────────────────────────────────────────────────── */
#define OP_NOP   0x00
#define OP_HALT  0x01
#define OP_LOAD  0x02  /* LOAD reg, imm8 */
#define OP_STORE 0x03  /* STORE addr, reg */
#define OP_ADD   0x04  /* ADD  r0, r1, r2 */
#define OP_SUB   0x05  /* SUB  r0, r1, r2 */
#define OP_JMP   0x06  /* JMP  addr16 */
#define OP_JZ    0x07  /* JZ   r0, addr16 (jump if r0==0) */
#define OP_CALL  0x08  /* CALL addr16 — push PC, jump */
#define OP_RET   0x09  /* RET  — pop PC */
#define OP_PUSH  0x0A  /* PUSH reg */
#define OP_POP   0x0B  /* POP  reg */
#define OP_OUT   0x0C  /* OUT  reg — emit to IPC bus */
#define OP_AND   0x0D
#define OP_OR    0x0E
#define OP_XOR   0x0F

/* ── VM State ────────────────────────────────────────────────────────────────── */
#define VM_MEM_SIZE   65536
#define VM_STACK_SIZE 256
#define VM_REG_COUNT  16

typedef struct {
    uint8_t  mem[VM_MEM_SIZE];
    uint32_t reg[VM_REG_COUNT];
    uint32_t pc;
    uint32_t sp;       /* stack pointer into call_stack */
    uint32_t call_stack[VM_STACK_SIZE];
    int      halted;
    uint64_t cycles;
} SigmaVM;

/* ── Output sink ─────────────────────────────────────────────────────────────── */
static void vm_out(uint32_t val) {
    /* In a real system: write to the IPC ring buffer */
    (void)val;
}

/* ── VM Init ─────────────────────────────────────────────────────────────────── */
void sigma_vm_init(SigmaVM *vm) {
    for (size_t i = 0; i < VM_MEM_SIZE; i++) vm->mem[i] = 0;
    for (int i = 0; i < VM_REG_COUNT; i++) vm->reg[i] = 0;
    vm->pc = 0; vm->sp = 0; vm->halted = 0; vm->cycles = 0;
}

/* ── VM Load Program ──────────────────────────────────────────────────────────── */
int sigma_vm_load(SigmaVM *vm, const uint8_t *prog, size_t len) {
    if (len > VM_MEM_SIZE) return -1;
    for (size_t i = 0; i < len; i++) vm->mem[i] = prog[i];
    return 0;
}

/* ── VM Step (single instruction) ────────────────────────────────────────────── */
int sigma_vm_step(SigmaVM *vm) {
    if (vm->halted || vm->pc >= VM_MEM_SIZE) return -1;
    uint8_t op = vm->mem[vm->pc++];
    vm->cycles++;

    switch (op) {
        case OP_NOP: break;
        case OP_HALT: vm->halted = 1; return 1;
        case OP_LOAD: {
            uint8_t r = vm->mem[vm->pc++] & 0x0F;
            vm->reg[r] = vm->mem[vm->pc++];
            break;
        }
        case OP_STORE: {
            uint16_t addr = (uint16_t)(vm->mem[vm->pc] | (vm->mem[vm->pc+1] << 8));
            vm->pc += 2;
            uint8_t r = vm->mem[vm->pc++] & 0x0F;
            if (addr < VM_MEM_SIZE) vm->mem[addr] = (uint8_t)vm->reg[r];
            break;
        }
        case OP_ADD: {
            uint8_t dst = vm->mem[vm->pc++] & 0x0F;
            uint8_t r1  = vm->mem[vm->pc++] & 0x0F;
            uint8_t r2  = vm->mem[vm->pc++] & 0x0F;
            vm->reg[dst] = vm->reg[r1] + vm->reg[r2];
            break;
        }
        case OP_SUB: {
            uint8_t dst = vm->mem[vm->pc++] & 0x0F;
            uint8_t r1  = vm->mem[vm->pc++] & 0x0F;
            uint8_t r2  = vm->mem[vm->pc++] & 0x0F;
            vm->reg[dst] = vm->reg[r1] - vm->reg[r2];
            break;
        }
        case OP_JMP: {
            uint16_t addr = (uint16_t)(vm->mem[vm->pc] | (vm->mem[vm->pc+1] << 8));
            vm->pc = addr;
            break;
        }
        case OP_JZ: {
            uint8_t r = vm->mem[vm->pc++] & 0x0F;
            uint16_t addr = (uint16_t)(vm->mem[vm->pc] | (vm->mem[vm->pc+1] << 8));
            vm->pc += 2;
            if (vm->reg[r] == 0) vm->pc = addr;
            break;
        }
        case OP_CALL: {
            if (vm->sp < VM_STACK_SIZE) vm->call_stack[vm->sp++] = vm->pc + 2;
            uint16_t addr = (uint16_t)(vm->mem[vm->pc] | (vm->mem[vm->pc+1] << 8));
            vm->pc = addr;
            break;
        }
        case OP_RET: {
            if (vm->sp > 0) vm->pc = vm->call_stack[--vm->sp];
            break;
        }
        case OP_PUSH: {
            uint8_t r = vm->mem[vm->pc++] & 0x0F;
            if (vm->sp < VM_STACK_SIZE) vm->call_stack[vm->sp++] = vm->reg[r];
            break;
        }
        case OP_POP: {
            uint8_t r = vm->mem[vm->pc++] & 0x0F;
            if (vm->sp > 0) vm->reg[r] = vm->call_stack[--vm->sp];
            break;
        }
        case OP_OUT: {
            uint8_t r = vm->mem[vm->pc++] & 0x0F;
            vm_out(vm->reg[r]);
            break;
        }
        case OP_AND: case OP_OR: case OP_XOR: {
            uint8_t dst = vm->mem[vm->pc++] & 0x0F;
            uint8_t r1  = vm->mem[vm->pc++] & 0x0F;
            uint8_t r2  = vm->mem[vm->pc++] & 0x0F;
            if      (op == OP_AND) vm->reg[dst] = vm->reg[r1] & vm->reg[r2];
            else if (op == OP_OR)  vm->reg[dst] = vm->reg[r1] | vm->reg[r2];
            else                   vm->reg[dst] = vm->reg[r1] ^ vm->reg[r2];
            break;
        }
        default:
            return -2; /* unknown opcode */
    }
    return 0;
}

/* ── VM Run Loop ──────────────────────────────────────────────────────────────── */
void sigma_vm_run(SigmaVM *vm, uint64_t max_cycles) {
    while (!vm->halted && vm->cycles < max_cycles) {
        if (sigma_vm_step(vm) < 0) break;
    }
}
