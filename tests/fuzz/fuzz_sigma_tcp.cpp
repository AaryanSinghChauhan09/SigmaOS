// SPDX-License-Identifier: GPL-2.0-or-later
// tests/fuzz/fuzz_sigma_tcp.cpp — libFuzzer harness for the TCP/IP stack
//
// Build:
//   clang++ -fsanitize=fuzzer,address -std=c++17 \
//     -Iinclude -Iklib/include \
//     tests/fuzz/fuzz_sigma_tcp.cpp \
//     kernel/net/sigma_tcpip.c -o fuzz_tcp
//
// Run (30 second budget):
//   ./fuzz_tcp -max_total_time=30 corpus/tcp/
//
// What this finds:
//   - Buffer overflows in packet parsing
//   - Integer overflows in length fields
//   - Use-after-free in connection tracking
//   - Infinite loops in state machine
//   - NULL dereferences on malformed options

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

// Forward declarations for the TCP stack under test
extern "C" {
    // Inject a raw IP packet into the TCP state machine
    // Returns: 0 = accepted, -1 = rejected (normal), -2 = error (bug)
    int sigma_tcp_inject_packet(const uint8_t *data, size_t len);

    // TCP option parser — target for malformed options
    int sigma_tcp_parse_options(const uint8_t *opts, size_t len,
                                 uint16_t *mss, uint8_t *wscale,
                                 uint32_t *sack_permitted);

    // Conntrack lookup — target for hash collision attacks
    int sigma_conntrack_lookup(uint32_t src_ip, uint32_t dst_ip,
                               uint16_t sport, uint16_t dport,
                               uint8_t proto);
}

// ── TCP header structure ──────────────────────────────────────────────────
struct __attribute__((packed)) tcp_hdr {
    uint16_t sport, dport;
    uint32_t seq, ack;
    uint8_t  doff_flags;
    uint8_t  flags;
    uint16_t window;
    uint16_t checksum;
    uint16_t urgent;
};

struct __attribute__((packed)) ip_hdr {
    uint8_t  version_ihl;
    uint8_t  tos;
    uint16_t tot_len;
    uint16_t id;
    uint16_t frag_off;
    uint8_t  ttl;
    uint8_t  protocol;   /* 6 = TCP */
    uint16_t checksum;
    uint32_t src, dst;
};

// ── Corpus-guided mutation helpers ───────────────────────────────────────
// libFuzzer will mutate these and discover edge cases automatically.

static const uint8_t SEED_SYN[] = {
    // IP header (20 bytes)
    0x45, 0x00, 0x00, 0x3c, 0x12, 0x34, 0x40, 0x00,
    0x40, 0x06, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x01,
    0x7f, 0x00, 0x00, 0x01,
    // TCP header (20 bytes) — SYN
    0x04, 0xd2, 0x00, 0x50, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x50, 0x02, 0x72, 0x10,
    0x00, 0x00, 0x00, 0x00,
};

// ── Main fuzzer entry point ───────────────────────────────────────────────
extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size < 1) return 0;

    // Test 1: Full packet injection (most coverage)
    if (size >= sizeof(ip_hdr) + sizeof(tcp_hdr)) {
        sigma_tcp_inject_packet(data, size);
    }

    // Test 2: TCP options parser (common attack surface)
    // Options live after the 20-byte TCP header
    if (size >= sizeof(tcp_hdr) + 4) {
        uint16_t mss = 0; uint8_t wscale = 0; uint32_t sack = 0;
        sigma_tcp_parse_options(data + sizeof(tcp_hdr),
                                size - sizeof(tcp_hdr),
                                &mss, &wscale, &sack);
    }

    // Test 3: Conntrack with fuzz-derived 5-tuple
    // Use first 13 bytes as (src_ip, dst_ip, sport, dport, proto)
    if (size >= 13) {
        uint32_t src_ip, dst_ip; uint16_t sport, dport; uint8_t proto;
        memcpy(&src_ip,  data,    4);
        memcpy(&dst_ip,  data+4,  4);
        memcpy(&sport,   data+8,  2);
        memcpy(&dport,   data+10, 2);
        proto = data[12];
        sigma_conntrack_lookup(src_ip, dst_ip, sport, dport, proto);
    }

    return 0;
}

// Provide initial seed corpus to guide the fuzzer
extern "C" size_t LLVMFuzzerCustomMutator(
    uint8_t *data, size_t size, size_t max_size, unsigned int seed)
{
    // 20% chance: start from a known-valid SYN packet
    if ((seed % 5) == 0 && max_size >= sizeof(SEED_SYN)) {
        size_t n = sizeof(SEED_SYN);
        memcpy(data, SEED_SYN, n);
        // Corrupt a random byte
        data[seed % n] ^= (uint8_t)(seed >> 8);
        return n;
    }
    return size; // fall through to libFuzzer's default mutator
}
