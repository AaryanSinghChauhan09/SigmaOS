// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_ebpf.h — eBPF-style programmable kernel hooks for SigmaOS
 *
 * Allows userland programs to attach small, verified bytecode programs to
 * kernel hook points: packet processing, syscall audit, scheduler events,
 * and ZeroTrust policy decisions. Programs are verified before attachment
 * (no loops, bounded memory, no kernel pointer leaks).
 *
 * Hook points:
 *   SIGMA_EBPF_HOOK_PACKET_RX    — inspect/drop/modify incoming packets
 *   SIGMA_EBPF_HOOK_PACKET_TX    — inspect/drop/modify outgoing packets
 *   SIGMA_EBPF_HOOK_SYSCALL      — audit every syscall (like strace in kernel)
 *   SIGMA_EBPF_HOOK_ZT_FLOW      — custom ZeroTrust policy rule
 *   SIGMA_EBPF_HOOK_SCHED_SWITCH — scheduler context switch event
 *
 * Usage (from userland daemon):
 *   sigma_ebpf_prog_t* prog = sigma_ebpf_load("filter.bpf", sizeof(filter));
 *   sigma_ebpf_attach(prog, SIGMA_EBPF_HOOK_PACKET_RX, NULL);
 */
#include <sigma_kernel_types.h>

/* ── Hook point IDs ───────────────────────────────────────────────────────── */
#define SIGMA_EBPF_HOOK_PACKET_RX    0
#define SIGMA_EBPF_HOOK_PACKET_TX    1
#define SIGMA_EBPF_HOOK_SYSCALL      2
#define SIGMA_EBPF_HOOK_ZT_FLOW      3
#define SIGMA_EBPF_HOOK_SCHED_SWITCH 4
#define SIGMA_EBPF_HOOK_COUNT        5

/* ── Verdict values (returned by BPF program) ─────────────────────────────── */
#define SIGMA_EBPF_PASS    0   /* allow / continue normal processing       */
#define SIGMA_EBPF_DROP    1   /* drop packet / deny syscall               */
#define SIGMA_EBPF_REDIRECT 2  /* redirect packet to another interface     */
#define SIGMA_EBPF_MODIFY  3   /* modify in place and continue             */

/* ── Max program size ─────────────────────────────────────────────────────── */
#define SIGMA_EBPF_MAX_INSNS  4096   /* 4096 instructions max per program   */
#define SIGMA_EBPF_MAX_MAPS   16     /* max maps per program                */
#define SIGMA_EBPF_MAP_SIZE   65536  /* 64KB per map                        */

/* ── BPF map types ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_EBPF_MAP_HASH    = 0,
    SIGMA_EBPF_MAP_ARRAY   = 1,
    SIGMA_EBPF_MAP_RINGBUF = 2,  /* ring buffer for events to userland     */
    SIGMA_EBPF_MAP_PERCPU  = 3,
} sigma_ebpf_map_type_t;

/* ── Context passed to BPF program at each hook ───────────────────────────── */
typedef struct {
    sigma_u64 hook_id;
    sigma_u64 timestamp_ns;
    union {
        struct { sigma_u8* data; sigma_u32 len; sigma_u32 src_ip; sigma_u16 src_port; sigma_u16 dst_port; } packet;
        struct { sigma_u32 syscall_nr; sigma_u64 arg0; sigma_u64 arg1; sigma_u32 pid; } syscall;
        struct { sigma_u32 from_pid; sigma_u32 to_pid; sigma_u32 dst_port; sigma_u8 proto; } zt_flow;
        struct { sigma_u32 prev_pid; sigma_u32 next_pid; } sched;
    };
} sigma_ebpf_ctx_t;

/* ── Program handle ───────────────────────────────────────────────────────── */
typedef struct sigma_ebpf_prog sigma_ebpf_prog_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Load and verify a BPF bytecode program. Returns NULL if verification fails. */
sigma_ebpf_prog_t* sigma_ebpf_load(const sigma_u8* bytecode, sigma_size_t len);

/* Attach program to a hook point. ctx_filter: only fire if ctx matches (NULL = always). */
int sigma_ebpf_attach(sigma_ebpf_prog_t* prog, int hook_id, void* ctx_filter);

/* Detach a program from a hook */
void sigma_ebpf_detach(sigma_ebpf_prog_t* prog, int hook_id);

/* Unload and free a program */
void sigma_ebpf_free(sigma_ebpf_prog_t* prog);

/* Run all programs at a hook point — called by kernel subsystems */
int sigma_ebpf_run_hook(int hook_id, sigma_ebpf_ctx_t* ctx);

/* Map operations (called from BPF programs via helper calls) */
int  sigma_ebpf_map_lookup(int map_fd, const void* key, void* value_out);
int  sigma_ebpf_map_update(int map_fd, const void* key, const void* value);
void sigma_ebpf_map_delete(int map_fd, const void* key);
int  sigma_ebpf_ringbuf_output(int map_fd, const void* data, sigma_size_t len);

void sigma_ebpf_init(void);
