/*
 * Σ SigmaOS Zenith — ARP Protocol Shard
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

#define ARP_HW_TYPE_ETHERNET 1
#define ARP_PROTOCOL_IPV4    0x0800
#define ARP_OP_REQUEST       1
#define ARP_OP_REPLY         2

struct __attribute__((packed)) sigma_arp_header {
    u16 hw_type;
    u16 protocol_type;
    u8  hw_addr_len;
    u8  protocol_addr_len;
    u16 opcode;
    u8  sender_mac[6];
    u32 sender_ip;
    u8  target_mac[6];
    u32 target_ip;
};

// ARP Cache (Simple Table)
#define ARP_CACHE_SIZE 128
struct {
    u32 ip;
    u8  mac[6];
    bool valid;
} arp_cache[ARP_CACHE_SIZE];

extern "C" void sovereign_memcpy(void* dst, const void* src, u32 n);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" bool sigma_eth_send(u8* dest_mac, u16 ethertype, u8* payload, u32 len);

/* Handle Incoming ARP Packet */
extern "C" void sigma_arp_receive(u8* packet, u32 len) {
    if (len < sizeof(sigma_arp_header)) return;
    
    struct sigma_arp_header* arp = (struct sigma_arp_header*)packet;
    
    // Convert endianness (assuming little-endian host, network is big-endian)
    u16 opcode = (arp->opcode >> 8) | (arp->opcode << 8);
    
    if (opcode == ARP_OP_REPLY) {
        // Cache the sender's MAC/IP
        for (int i = 0; i < ARP_CACHE_SIZE; i++) {
            if (!arp_cache[i].valid || arp_cache[i].ip == arp->sender_ip) {
                arp_cache[i].ip = arp->sender_ip;
                sovereign_memcpy(arp_cache[i].mac, arp->sender_mac, 6);
                arp_cache[i].valid = true;
                sigma_vga_printf("ARP Cache Updated: IP %x -> MAC %x:%x:...\n", arp->sender_ip, arp->sender_mac[0], arp->sender_mac[1]);
                break;
            }
        }
    } else if (opcode == ARP_OP_REQUEST) {
        // Respond if it's asking for our IP
        // (Assuming our IP is known globally, left out for brevity)
        sigma_vga_printf("Received ARP Request for IP: %x\n", arp->target_ip);
    }
}
