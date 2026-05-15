#include "../../../include/libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Encrypted Networking (Sovereign Stack) Prototype
// ---------------------------------------------------------

// A minimal symmetric encryption shim (XOR stream cipher - for prototype)
// In production, replace with ChaCha20-Poly1305 or AES-GCM
void stream_encrypt(uint8_t* data, size_t len, const uint8_t* key, size_t key_len) {
    for (size_t i = 0; i < len; i++) {
        data[i] ^= key[i % key_len];
    }
}

// Encrypted packet structure
typedef struct {
    uint8_t  version;        // Protocol version (Sovereign Packet v1)
    uint16_t payload_length; // Length of encrypted payload
    uint32_t session_id;     // Encrypted session token
    uint8_t  nonce[12];      // Randomized nonce for encryption
    uint8_t  tag[16];        // Authentication tag (AEAD)
    uint8_t  payload[];      // Encrypted payload (flexible array member)
} __attribute__((packed)) sovereign_packet_t;

// Send an encrypted packet
void sovereign_send(int sock, const uint8_t* plaintext, uint16_t len, const uint8_t* session_key) {
    uint8_t buffer[2048];
    sovereign_packet_t* pkt = (sovereign_packet_t*)buffer;
    
    pkt->version = 1;
    pkt->payload_length = len;

    // Copy plaintext into payload
    for (int i = 0; i < len; i++) pkt->payload[i] = plaintext[i];
    
    // Encrypt payload in-place using session key
    stream_encrypt(pkt->payload, len, session_key, 32);

    // send(sock, buffer, sizeof(sovereign_packet_t) + len, 0);
}

// Performance Monitor Prototype
typedef struct {
    uint32_t cpu_usage_percent;
    uint32_t mem_used_kb;
    uint32_t mem_total_kb;
    uint32_t net_rx_bytes;
    uint32_t net_tx_bytes;
    uint32_t disk_read_bytes;
    uint32_t disk_write_bytes;
} perf_stats_t;

static perf_stats_t current_stats;

void perf_update_cpu(uint32_t usage) { current_stats.cpu_usage_percent = usage; }
void perf_update_mem(uint32_t used, uint32_t total) {
    current_stats.mem_used_kb = used;
    current_stats.mem_total_kb = total;
}
void perf_get_stats(perf_stats_t* out) { *out = current_stats; }
