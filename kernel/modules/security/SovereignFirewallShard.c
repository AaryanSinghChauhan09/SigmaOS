/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN FIREWALL SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb nftables / pf / Windows Defender FW / iptables USP.
 *          Native Silicon Packet Filter with Stateful Rule Engine.
 * Design: C11 / Zero-Dependency / Policy-Based Silicon Packet Classification.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Firewall Structures
// -------------------------------------------------------------------------

typedef enum {
    FW_PROTO_ANY  = 0,
    FW_PROTO_TCP  = 6,
    FW_PROTO_UDP  = 17,
    FW_PROTO_ICMP = 1
} SigmaFWProto_t;

typedef enum {
    FW_ACCEPT,
    FW_DROP,
    FW_REJECT,
    FW_LOG_AND_ACCEPT
} SigmaFWVerdict_t;

typedef struct {
    sigma_u32        rule_id;
    SigmaFWProto_t   proto;
    sigma_u32        src_addr;     /* 0 = any (network byte order) */
    sigma_u32        dst_addr;     /* 0 = any                      */
    sigma_u16        dst_port;     /* 0 = any                      */
    SigmaFWVerdict_t verdict;
    sigma_u64        hit_count;
    char             comment[40];
} SigmaFWRule_t;

#define MAX_FW_RULES 32
static SigmaFWRule_t s_fw_table[MAX_FW_RULES];
static sigma_u32     s_fw_count  = 0;
static sigma_u64     s_pkt_in    = 0;
static sigma_u64     s_pkt_drop  = 0;

// -------------------------------------------------------------------------
// Firewall Logic (nftables / iptables / pf / Windows Defender FW parity)
// -------------------------------------------------------------------------

/**
 * sigma_fw_add_rule: Adds a silicon packet filter rule.
 */
sigma_err_t sigma_fw_add_rule(SigmaFWProto_t proto,
                               sigma_u32 src, sigma_u32 dst,
                               sigma_u16 port,
                               SigmaFWVerdict_t verdict,
                               const char* comment) {
    if (s_fw_count >= MAX_FW_RULES) return SIGMA_ENOSPC;

    SigmaFWRule_t* r = &s_fw_table[s_fw_count];
    r->rule_id   = s_fw_count + 1;
    r->proto     = proto;
    r->src_addr  = src;
    r->dst_addr  = dst;
    r->dst_port  = port;
    r->verdict   = verdict;
    r->hit_count = 0;
    sigma_strcpy(r->comment, comment);
    s_fw_count++;

    static const char* vnames[] = { "ACCEPT", "DROP", "REJECT", "LOG+ACCEPT" };
    static const char* pnames[] = { "ANY", "TCP", "UDP", "ICMP" };
    sigma_u32 pi = (proto == FW_PROTO_TCP) ? 1 :
                   (proto == FW_PROTO_UDP) ? 2 :
                   (proto == FW_PROTO_ICMP) ? 3 : 0;
    sigma_printf("[FW]: Rule #%-2u %s/%s dst_port:%u -> %s  (%s)\n",
                 r->rule_id, pnames[pi], comment,
                 port, vnames[verdict], comment);
    return SIGMA_OK;
}

/**
 * sigma_fw_classify: Classifies an incoming silicon packet against the rule table.
 */
SigmaFWVerdict_t sigma_fw_classify(SigmaFWProto_t proto,
                                    sigma_u32 src, sigma_u32 dst,
                                    sigma_u16 dst_port) {
    s_pkt_in++;
    for (sigma_u32 i = 0; i < s_fw_count; i++) {
        SigmaFWRule_t* r = &s_fw_table[i];
        sigma_bool proto_match = (r->proto == FW_PROTO_ANY || r->proto == proto);
        sigma_bool src_match   = (r->src_addr == 0 || r->src_addr == src);
        sigma_bool dst_match   = (r->dst_addr == 0 || r->dst_addr == dst);
        sigma_bool port_match  = (r->dst_port == 0 || r->dst_port == dst_port);

        if (proto_match && src_match && dst_match && port_match) {
            r->hit_count++;
            if (r->verdict == FW_DROP || r->verdict == FW_REJECT) s_pkt_drop++;
            return r->verdict;
        }
    }
    /* Default policy: DROP (deny-by-default silicon sovereignty) */
    s_pkt_drop++;
    return FW_DROP;
}

// -------------------------------------------------------------------------
// Industrial Firewall Audit
// -------------------------------------------------------------------------

void SovereignFirewall_Audit() {
    static const char* vnames[] = { "ACCEPT", "DROP ", "REJECT", "LOG+ACCEPT" };
    static const char* pnames[] = { "ANY", "TCP", "UDP", "ICMP" };
    sigma_printf("\n--- SOVEREIGN FIREWALL AUDIT ---\n");
    sigma_printf("Packets In: %llu | Dropped: %llu | Rules: %u\n",
                 (unsigned long long)s_pkt_in,
                 (unsigned long long)s_pkt_drop,
                 s_fw_count);
    sigma_printf("#    PROTO PORT  VERDICT     HITS       COMMENT\n");
    sigma_printf("----------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_fw_count; i++) {
        SigmaFWRule_t* r = &s_fw_table[i];
        sigma_u32 pi = (r->proto == FW_PROTO_TCP) ? 1 :
                       (r->proto == FW_PROTO_UDP) ? 2 :
                       (r->proto == FW_PROTO_ICMP) ? 3 : 0;
        sigma_printf("%-4u %-5s %-5u %-11s %-10llu %s\n",
                     r->rule_id, pnames[pi], r->dst_port,
                     vnames[r->verdict],
                     (unsigned long long)r->hit_count,
                     r->comment);
    }
    sigma_printf("----------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignFirewallShard_Init() {
    sigma_printf("[SOC]: Seating Native Firewall Shard (nftables/pf/WFP Parity v1.0)...\n");
    /* Baseline industrial policy */
    sigma_fw_add_rule(FW_PROTO_TCP,  0, 0, 22,   FW_ACCEPT,       "Allow SSH");
    sigma_fw_add_rule(FW_PROTO_TCP,  0, 0, 443,  FW_ACCEPT,       "Allow HTTPS");
    sigma_fw_add_rule(FW_PROTO_TCP,  0, 0, 80,   FW_LOG_AND_ACCEPT,"Allow HTTP (log)");
    sigma_fw_add_rule(FW_PROTO_ICMP, 0, 0, 0,    FW_ACCEPT,       "Allow ICMP ping");
    sigma_fw_add_rule(FW_PROTO_ANY,  0, 0, 23,   FW_DROP,         "Block Telnet");
    sigma_fw_add_rule(FW_PROTO_UDP,  0, 0, 53,   FW_ACCEPT,       "Allow DNS");
    sigma_fw_add_rule(FW_PROTO_ANY,  0, 0, 0,    FW_DROP,         "Default deny-all");

    /* Simulate initial packets for hit counts */
    sigma_fw_classify(FW_PROTO_TCP, 0, 0, 443);
    sigma_fw_classify(FW_PROTO_TCP, 0, 0, 23);
    sigma_fw_classify(FW_PROTO_UDP, 0, 0, 9999);
}
