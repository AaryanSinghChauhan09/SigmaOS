/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * vm.h — SigmaOS eBPF-style in-kernel VM (sigma-probe bytecode)
 *
 * A safe, sandboxed bytecode VM for kernel inspection without modifying
 * the kernel itself.  Programs are:
 *   1. Verified by the static analyser (bounded loops, no wild pointers)
 *   2. JIT-compiled to native x86_64 for performance
 *   3. Attached to hook points (IPC, syscall, block I/O, network)
 *
 * Instruction set is a subset of eBPF (64-bit register machine, 10 regs).
 * Programs communicate with userspace via maps (ring buffer, hash, array).
 *
 * Inspired by: Linux eBPF (kernel/bpf/), bpftrace, DPDK rte_bpf
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/* ── Registers ───────────────────────────────────────────────────────────── */
#define BPF_REG_MAX  11
#define BPF_REG_ARG1  1
#define BPF_REG_ARG2  2
#define BPF_REG_ARG3  3
#define BPF_REG_ARG4  4
#define BPF_REG_ARG5  5
#define BPF_REG_CTX   6   /* context pointer (read-only) */
#define BPF_REG_FP    10  /* frame pointer (read-only) */
#define BPF_REG_RET   0   /* return value */

/* ── Instruction format (8 bytes, identical to Linux eBPF) ─────────────── */

typedef struct __attribute__((packed)) sigma_bpf_insn {
    uint8_t  code;
    uint8_t  dst_reg : 4;
    uint8_t  src_reg : 4;
    int16_t  off;
    int32_t  imm;
} sigma_bpf_insn_t;

/* ── Opcodes ────────────────────────────────────────────────────────────── */

#define BPF_ALU64_REG(op, dst, src) /* arithmetic 64-bit reg */
#define BPF_ALU64_IMM(op, dst, imm) /* arithmetic 64-bit imm */
#define BPF_MOV64_REG(dst, src)     /* dst = src */
#define BPF_MOV64_IMM(dst, imm)     /* dst = imm */
#define BPF_LD_MAP_FD(dst, map_fd)  /* load map fd */
#define BPF_CALL(func_id)           /* kernel helper call */
#define BPF_EXIT()                  /* exit program */
#define BPF_JMP_REG(op, dst, src, off) /* conditional jump */
#define BPF_LDX_MEM(size, dst, src, off) /* load from memory */
#define BPF_STX_MEM(size, dst, src, off) /* store to memory */

/* ── Map types ──────────────────────────────────────────────────────────── */

typedef enum sigma_bpf_map_type {
    BPF_MAP_HASH      = 0,
    BPF_MAP_ARRAY     = 1,
    BPF_MAP_RINGBUF   = 2,
    BPF_MAP_PERF_EVT  = 3,
    BPF_MAP_PERCPU    = 4,
} sigma_bpf_map_type_t;

typedef struct sigma_bpf_map_spec {
    sigma_bpf_map_type_t type;
    uint32_t key_size;
    uint32_t val_size;
    uint32_t max_entries;
    uint32_t flags;
} sigma_bpf_map_spec_t;

/* ── Hook points ────────────────────────────────────────────────────────── */

typedef enum sigma_bpf_hook {
    BPF_HOOK_IPC_SEND       = 0,
    BPF_HOOK_IPC_RECV       = 1,
    BPF_HOOK_BLOCK_READ     = 2,
    BPF_HOOK_BLOCK_WRITE    = 3,
    BPF_HOOK_NET_RX         = 4,
    BPF_HOOK_NET_TX         = 5,
    BPF_HOOK_SYSCALL_ENTER  = 6,
    BPF_HOOK_SYSCALL_EXIT   = 7,
    BPF_HOOK_SCHED_SWITCH   = 8,
    BPF_HOOK_KPROBE         = 9,   /* attach to any kernel function */
    BPF_HOOK_UPROBE         = 10,  /* attach to any userland function */
    BPF_HOOK__COUNT
} sigma_bpf_hook_t;

/* ── Kernel helper functions (called from BPF programs) ─────────────────── */

typedef enum sigma_bpf_helper {
    BPF_HELPER_MAP_LOOKUP    = 1,
    BPF_HELPER_MAP_UPDATE    = 2,
    BPF_HELPER_MAP_DELETE    = 3,
    BPF_HELPER_RINGBUF_RESERVE = 4,
    BPF_HELPER_RINGBUF_SUBMIT  = 5,
    BPF_HELPER_GET_TIME_NS   = 6,
    BPF_HELPER_GET_SHARD_ID  = 7,
    BPF_HELPER_TRACE_PRINTK  = 8,
    BPF_HELPER_PERF_SUBMIT   = 9,
    BPF_HELPER_OVERRIDE_RETURN = 10, /* change syscall return value */
} sigma_bpf_helper_t;

/* ── Verifier result ─────────────────────────────────────────────────────── */

typedef struct sigma_bpf_verify_result {
    bool     ok;
    int      insn_idx;         /* instruction where error occurred */
    char     error_msg[256];
    uint32_t stack_depth;
    uint32_t insn_processed;
} sigma_bpf_verify_result_t;

/* ── BPF API ─────────────────────────────────────────────────────────────── */

/* Create a BPF map */
int  sigma_bpf_map_create (const sigma_bpf_map_spec_t *spec, int *out_fd);
void sigma_bpf_map_close  (int map_fd);
int  sigma_bpf_map_lookup (int map_fd, const void *key, void *val);
int  sigma_bpf_map_update (int map_fd, const void *key, const void *val);
int  sigma_bpf_map_delete (int map_fd, const void *key);

/* Load, verify, and optionally JIT-compile a BPF program */
int  sigma_bpf_prog_load  (const sigma_bpf_insn_t *insns, uint32_t insn_cnt,
                             sigma_bpf_hook_t hook, int *out_fd,
                             sigma_bpf_verify_result_t *out_verify);
void sigma_bpf_prog_close (int prog_fd);

/* Attach/detach to a hook point */
int  sigma_bpf_attach     (int prog_fd, sigma_bpf_hook_t hook,
                             const char *target); /* target = function name for kprobe */
int  sigma_bpf_detach     (int prog_fd);

/* Read from a ring buffer map */
int  sigma_bpf_ringbuf_read(int map_fd, void *buf, size_t len,
                              int timeout_ms);

/* List all loaded programs */
typedef struct sigma_bpf_prog_info {
    int              fd;
    sigma_bpf_hook_t hook;
    uint32_t         insn_cnt;
    char             tag[8];    /* SHA-256 prefix */
    uint64_t         run_count;
    uint64_t         run_time_ns;
} sigma_bpf_prog_info_t;

int  sigma_bpf_prog_list  (sigma_bpf_prog_info_t *out, uint32_t max,
                             uint32_t *count);
