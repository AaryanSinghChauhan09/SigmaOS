/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: BPF (BERKELEY PACKET FILTER) VIRTUAL MACHINE
 * =============================================================================
 * Inspired by: Linux kernel kernel/bpf/core.c
 *              FreeBSD sys/net/bpf_filter.c
 * =============================================================================
 * In-kernel sandbox VM for high-performance packet filtering and telemetry.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

/* Simplified BPF instruction format */
#define BPF_CLASS(code) ((code) & 0x07)
#define BPF_LD    0x00
#define BPF_ALU   0x04
#define BPF_JMP   0x05
#define BPF_RET   0x06

#define BPF_OP(code)    ((code) & 0xf0)
#define BPF_ADD   0x00
#define BPF_SUB   0x10
#define BPF_MUL   0x20
#define BPF_JEQ   0x10

#define BPF_SRC(code)   ((code) & 0x08)
#define BPF_K     0x00
#define BPF_X     0x08

typedef struct {
    sigma_u16 code;
    sigma_u8  jt;
    sigma_u8  jf;
    sigma_u32 k;
} sigma_bpf_insn_t;

void bpf_init(void) {
    sigma_printf("[bpf] Berkeley Packet Filter VM initialized\n");
}

/* 
 * Executes a BPF program against a packet buffer. 
 * Returns the amount of bytes to accept (0 = drop, >0 = accept).
 */
sigma_u32 bpf_filter(const sigma_bpf_insn_t* prog, sigma_u32 prog_len, const sigma_u8* pkt, sigma_u32 pkt_len) {
    sigma_u32 A = 0; /* Accumulator */
    sigma_u32 X = 0; /* Index Register */
    sigma_u32 pc = 0; /* Program Counter */
    
    if (!prog || prog_len == 0) return 0; /* Drop if no program */

    sigma_printf("[bpf] Executing BPF filter (%u instructions)...\n", prog_len);

    while (pc < prog_len) {
        const sigma_bpf_insn_t* insn = &prog[pc];
        sigma_u16 code = insn->code;
        
        switch (BPF_CLASS(code)) {
            case BPF_LD:
                /* Simplified: Load absolute 32-bit word from packet */
                if (insn->k + 3 < pkt_len) {
                    A = (pkt[insn->k] << 24) | (pkt[insn->k+1] << 16) | 
                        (pkt[insn->k+2] << 8) | pkt[insn->k+3];
                } else {
                    return 0; /* Out of bounds read */
                }
                break;
                
            case BPF_ALU:
                switch (BPF_OP(code)) {
                    case BPF_ADD:
                        A += (BPF_SRC(code) == BPF_K) ? insn->k : X;
                        break;
                    case BPF_SUB:
                        A -= (BPF_SRC(code) == BPF_K) ? insn->k : X;
                        break;
                }
                break;
                
            case BPF_JMP:
                switch (BPF_OP(code)) {
                    case BPF_JEQ:
                        if (A == ((BPF_SRC(code) == BPF_K) ? insn->k : X)) {
                            pc += insn->jt;
                        } else {
                            pc += insn->jf;
                        }
                        break;
                }
                break;
                
            case BPF_RET:
                return (BPF_SRC(code) == BPF_K) ? insn->k : A;
        }
        
        pc++;
        
        if (pc >= prog_len) {
            sigma_printf("[bpf] ERR: BPF execution ran off end of program\n");
            return 0;
        }
    }
    
    return 0; /* Drop by default if fell through */
}
