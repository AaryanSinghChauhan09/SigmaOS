/*
 * Host-side smoke test for SigmaOS IPv4/ICMP header helpers (no kernel linkage).
 * Mirrors checksum + ethertype checks used in kernel/net/sigma_net.c.
 */
#include <stdint.h>
#include <stdio.h>
#include <string.h>

typedef uint8_t  sigma_u8;
typedef uint16_t sigma_u16;
typedef uint32_t sigma_u32;
typedef int      sigma_bool;
#define SIGMA_TRUE 1
#define SIGMA_FALSE 0

#pragma pack(push, 1)
typedef struct {
    sigma_u8 dest[6];
    sigma_u8 src[6];
    sigma_u16 ethertype;
} eth_hdr_t;

typedef struct {
    sigma_u8  ihl_version;
    sigma_u8  tos;
    sigma_u16 len;
    sigma_u16 id;
    sigma_u16 frag_offset;
    sigma_u8  ttl;
    sigma_u8  proto;
    sigma_u16 checksum;
    sigma_u8  src_ip[4];
    sigma_u8  dest_ip[4];
} ipv4_hdr_t;
#pragma pack(pop)

static sigma_u16 calculate_checksum(void* data, sigma_u32 bytes) {
    sigma_u16* p = (sigma_u16*)data;
    sigma_u32 sum = 0;
    while (bytes > 1) {
        sum += *p++;
        bytes -= 2;
    }
    if (bytes > 0) sum += *((sigma_u8*)p);
    while (sum >> 16) sum = (sum & 0xFFFF) + (sum >> 16);
    return (sigma_u16)(~sum);
}

int main(void) {
    eth_hdr_t eth;
    memset(&eth, 0, sizeof(eth));
    for (int i = 0; i < 6; i++) eth.dest[i] = 0xFF;
    eth.ethertype = 0x0008;

    ipv4_hdr_t ip;
    memset(&ip, 0, sizeof(ip));
    ip.ihl_version = 0x45;
    ip.proto = 1;
    ip.checksum = calculate_checksum(&ip, sizeof(ip));

    printf("sigma_net_host_test: ethertype=0x%04x ipv4_cksum=0x%04x\n",
           eth.ethertype, ip.checksum);
    return 0;
}
