// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_firewall.h — Stateful packet filter & NAT engine
 *
 * Inspired by OpenBSD pf, Linux nftables, and FreeBSD ipfw.
 *
 * Features:
 *   - Stateful TCP/UDP/ICMP connection tracking
 *   - Per-workload rule chains (INPUT / OUTPUT / FORWARD)
 *   - NAT masquerade + port forwarding (DNAT/SNAT)
 *   - Rate limiting (token-bucket per source IP)
 *   - GeoIP-based country blocking (blocklist via sigma-pkg)
 *   - Logging to sigma-journald with eBPF ring-buffer
 *   - Rules expressed as sigma-policy JSON (compiled to BPF bytecode)
 *
 * Integration: sigma_pledge() restricts syscalls AFTER firewall init;
 *              sigma_cgroup.cpp enforces bandwidth limits per cgroup.
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Protocol constants ──────────────────────────────────────────────────── */
#define SIGMA_FW_PROTO_TCP   6
#define SIGMA_FW_PROTO_UDP   17
#define SIGMA_FW_PROTO_ICMP  1
#define SIGMA_FW_PROTO_ANY   0xFF

/* ── Rule action ─────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_FW_ACCEPT = 0,
    SIGMA_FW_DROP   = 1,
    SIGMA_FW_REJECT = 2,   /* sends RST/ICMP unreachable */
    SIGMA_FW_LOG    = 3,   /* accept + log to journald   */
    SIGMA_FW_QUEUE  = 4,   /* defer to userspace daemon  */
} sigma_fw_action_t;

/* ── Chain direction ─────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_FW_INPUT   = 0,
    SIGMA_FW_OUTPUT  = 1,
    SIGMA_FW_FORWARD = 2,
} sigma_fw_chain_t;

/* ── Address match ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32 addr;    /* IPv4 in network byte order; 0 = any */
    sigma_u32 mask;
} sigma_fw_addr4_t;

/* ── Firewall rule ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_fw_chain_t  chain;
    sigma_fw_action_t action;
    sigma_u8          proto;          /* SIGMA_FW_PROTO_* or ANY            */
    sigma_fw_addr4_t  src, dst;
    sigma_u16         sport_lo, sport_hi;  /* port range, 0=any            */
    sigma_u16         dport_lo, dport_hi;
    char              iface[16];      /* "" = all interfaces               */
    bool              stateful;       /* track connection state             */
    sigma_u32         rate_pps;       /* packets/sec limit; 0 = unlimited  */
    char              label[32];      /* for logging                       */
    sigma_u32         priority;       /* lower number = evaluated first    */
} sigma_fw_rule_t;

/* ── NAT entry ───────────────────────────────────────────────────────────── */
typedef struct {
    sigma_fw_addr4_t  orig_src, orig_dst;
    sigma_u16         orig_sport, orig_dport;
    sigma_fw_addr4_t  xlat_src, xlat_dst;
    sigma_u16         xlat_sport, xlat_dport;
    sigma_u8          proto;
} sigma_nat_entry_t;

/* ── Connection track entry ──────────────────────────────────────────────── */
typedef struct {
    sigma_fw_addr4_t  src, dst;
    sigma_u16         sport, dport;
    sigma_u8          proto;
    sigma_u8          state;          /* TCP state machine / ESTABLISHED   */
    sigma_u64         last_seen_ns;
    sigma_u64         bytes_fwd, bytes_rev;
} sigma_conntrack_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

int  sigma_fw_init(void);
void sigma_fw_shutdown(void);

/* Rule management */
int  sigma_fw_add_rule(const sigma_fw_rule_t *rule);
int  sigma_fw_del_rule(sigma_u32 priority, sigma_fw_chain_t chain);
int  sigma_fw_flush(sigma_fw_chain_t chain);
int  sigma_fw_list_rules(sigma_fw_chain_t chain,
                          sigma_fw_rule_t *out, int max_rules);

/* NAT */
int  sigma_nat_add_masquerade(const char *iface);
int  sigma_nat_add_dnat(sigma_u16 ext_port, sigma_fw_addr4_t dst,
                         sigma_u16 dst_port, sigma_u8 proto);

/* Connection tracking */
int  sigma_conntrack_dump(sigma_conntrack_t *out, int max_entries);
void sigma_conntrack_flush_expired(void);

/* Packet verdict (called from network driver shard) */
sigma_fw_action_t sigma_fw_process_packet(const void *pkt_data,
                                           sigma_size_t pkt_len,
                                           sigma_fw_chain_t chain);

/* Stats */
typedef struct {
    sigma_u64 packets_accepted;
    sigma_u64 packets_dropped;
    sigma_u64 packets_rejected;
    sigma_u64 nat_translations;
    sigma_u64 conntrack_entries;
} sigma_fw_stats_t;
int sigma_fw_stats(sigma_fw_stats_t *out);
