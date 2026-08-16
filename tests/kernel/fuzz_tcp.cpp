// SPDX-License-Identifier: MIT
// =============================================================================
// SIGMAOS KERNEL FUZZ HARNESS: TCP PACKET PARSER
// =============================================================================
// LLVMFuzzer test harness for stress testing TCP/IP header parsing logic,
// data offset boundaries, and payload validation under arbitrary inputs.
// =============================================================================

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#define MIN_TCP_HEADER_SIZE 20
#define MAX_TCP_HEADER_SIZE 60

typedef struct {
    uint16_t src_port;
    uint16_t dst_port;
    uint32_t seq_num;
    uint32_t ack_num;
    uint8_t  header_len;
    uint8_t  flags;
    uint16_t window_size;
    uint16_t checksum;
    uint16_t urgent_ptr;
    const uint8_t *payload;
    size_t   payload_len;
} FuzzTcpPacket;

static bool fuzz_parse_tcp_header(const uint8_t *data, size_t size, FuzzTcpPacket *pkt) {
    if (data == NULL || size < MIN_TCP_HEADER_SIZE) {
        return false;
    }

    uint8_t data_offset_raw = data[12] >> 4;
    uint8_t header_len = data_offset_raw * 4;

    // Header length must be at least 20 bytes and at most 60 bytes, and cannot exceed packet size
    if (header_len < MIN_TCP_HEADER_SIZE || header_len > MAX_TCP_HEADER_SIZE || header_len > size) {
        return false;
    }

    if (pkt != NULL) {
        pkt->src_port   = ((uint16_t)data[0] << 8) | data[1];
        pkt->dst_port   = ((uint16_t)data[2] << 8) | data[3];
        pkt->seq_num    = ((uint32_t)data[4] << 24) | ((uint32_t)data[5] << 16) |
                          ((uint32_t)data[6] << 8)  | data[7];
        pkt->ack_num    = ((uint32_t)data[8] << 24) | ((uint32_t)data[9] << 16) |
                          ((uint32_t)data[10] << 8) | data[11];
        pkt->header_len  = header_len;
        pkt->flags       = data[13];
        pkt->window_size = ((uint16_t)data[14] << 8) | data[15];
        pkt->checksum    = ((uint16_t)data[16] << 8) | data[17];
        pkt->urgent_ptr  = ((uint16_t)data[18] << 8) | data[19];
        pkt->payload     = data + header_len;
        pkt->payload_len = size - header_len;
    }

    return true;
}

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    FuzzTcpPacket pkt = {0};
    fuzz_parse_tcp_header(data, size, &pkt);
    return 0;
}
