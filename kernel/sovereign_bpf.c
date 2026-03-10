/*
 * Cosmos AI-OS: Sovereign Packet Filter (S-BPF, C Layer)
 * ========================================================
 * Mission: Hardcore Ring-0 network filtering. Drops 3rd-party
 *          telemetry and malicious connections instantly.
 */

#include <stddef.h>
#include <stdint.h>


// Simulated Network Packet Structure
typedef struct {
  uint32_t src_ip;
  uint32_t dst_ip;
  uint16_t src_port;
  uint16_t dst_port;
  uint8_t protocol; // 6 = TCP, 17 = UDP
  uint16_t payload_len;
  uint8_t *payload;
} cosmos_packet_t;

// Hardcoded IP Blacklists (e.g., AD networks, telemetry hosts)
// In a real system, these would represent hash blocks of known 3rd party
// tracker subnets
static const uint32_t telemetry_ips[] = {
    0x08080808, // 8.8.8.8 (Google DNS - if strict isolated)
    0x8EFA0114, // Random Analytics Server
    0x00000000  // End of list
};

int is_blacklisted(uint32_t ip) {
  for (int i = 0; telemetry_ips[i] != 0; i++) {
    // Strict subnet or direct match
    if (ip == telemetry_ips[i]) {
      return 1;
    }
  }
  return 0;
}

// Simple fast string match for payload scanning
int fast_pattern_match(const uint8_t *data, uint16_t len, const char *pattern,
                       uint16_t pat_len) {
  if (pat_len > len)
    return 0;
  for (uint16_t i = 0; i <= len - pat_len; i++) {
    int match = 1;
    for (uint16_t j = 0; j < pat_len; j++) {
      if (data[i + j] != pattern[j]) {
        match = 0;
        break;
      }
    }
    if (match)
      return 1;
  }
  return 0;
}

// Return 1 if KEEP, 0 if DROP
int cosmos_bpf_filter(cosmos_packet_t *pkt) {
  // 1. IP Level Block
  if (is_blacklisted(pkt->dst_ip) || is_blacklisted(pkt->src_ip)) {
    return 0; // DROP instantly
  }

  // 2. Payload Inspection (Block 3rd party domains)
  if (pkt->payload && pkt->payload_len > 0) {
    if (fast_pattern_match(pkt->payload, pkt->payload_len, "google-analytics",
                           16))
      return 0;
    if (fast_pattern_match(pkt->payload, pkt->payload_len, "telemetry", 9))
      return 0;
    if (fast_pattern_match(pkt->payload, pkt->payload_len, "tracking", 8))
      return 0;
  }

  // 3. All checks passed
  return 1;
}
