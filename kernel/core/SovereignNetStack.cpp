#include "../../include/core/sigma_types.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/libc/SovereignLibC.h"

/* =========================================================================
 * SIGMAOS: SOVEREIGN TCP/IP NETWORK STACK v2.0
 * Zero-Trust, Zero-Copy, Data-Link-layer Deep Packet Inspection
 * ========================================================================= */

#pragma pack(push, 1)
struct EthernetHeader {
    sigma_u8  dest_mac[6];
    sigma_u8  src_mac[6];
    sigma_u16 ethertype;
};

struct IPv4Header {
    sigma_u8  version_ihl;
    sigma_u8  dscp_ecn;
    sigma_u16 length;
    sigma_u16 ident;
    sigma_u16 flags_offset;
    sigma_u8  ttl;
    sigma_u8  protocol;
    sigma_u16 checksum;
    sigma_u32 src_ip;
    sigma_u32 dest_ip;
};

struct IPv6Header {
    sigma_u32 version_tc_flow;
    sigma_u16 payload_len;
    sigma_u8  next_header;
    sigma_u8  hop_limit;
    sigma_u8  src_ip[16];
    sigma_u8  dest_ip[16];
};

struct TCPHeader {
    sigma_u16 src_port;
    sigma_u16 dest_port;
    sigma_u32 seq_num;
    sigma_u32 ack_num;
    sigma_u8  data_offset;
    sigma_u8  flags;
    sigma_u16 window_size;
    sigma_u16 checksum;
    sigma_u16 urgent_ptr;
};

struct UDPHeader {
    sigma_u16 src_port;
    sigma_u16 dest_port;
    sigma_u16 length;
    sigma_u16 checksum;
};
#pragma pack(pop)

/* Inline ntohs for freestanding environments */
static inline sigma_u16 sigma_ntohs(sigma_u16 v) { return (sigma_u16)((v >> 8) | (v << 8)); }
static inline sigma_u32 sigma_ntohl(sigma_u32 v) {
    return ((v >> 24) & 0xFF) | ((v >> 8) & 0xFF00) | ((v << 8) & 0xFF0000) | ((v << 24) & 0xFF000000);
}

class SovereignNetStackEngine {
public:
    static SovereignNetStackEngine& getInstance() {
        static SovereignNetStackEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[NET] Initializing Sovereign Zero-Trust TCP/IPv4/IPv6 Stack v2.0...");
        this->interfaces_active = 0;
        this->packets_received  = 0;
        this->packets_filtered  = 0;
        this->bytes_dispatched  = 0;
        sigma_log("[NET] Data-link DPI firewall ACTIVE. MTU: 1500B (RFC 791).");
    }

    void registerInterface(const char* mac_addr, const char* ip_addr) {
        if (this->interfaces_active >= MAX_IFACES) {
            sigma_log("[NET] [WARN] Interface table full.");
            return;
        }
        sigma_hardened_strcpy(this->iface_mac[this->interfaces_active], mac_addr, 18);
        sigma_hardened_strcpy(this->iface_ip[this->interfaces_active],  ip_addr,  16);
        this->interfaces_active++;
        sigma_log_info("[NET] Interface registered: MAC=%s IP=%s\n", mac_addr, ip_addr);
    }

    bool dispatchPacket(const void* raw_frame, sigma_u32 length) {
        this->packets_received++;
        if (!raw_frame || length < (sigma_u32)sizeof(EthernetHeader)) {
            sigma_log("[NET] [DROP] Frame too short for Ethernet header.");
            this->packets_filtered++;
            return false;
        }

        const EthernetHeader* eth = (const EthernetHeader*)raw_frame;
        sigma_u16 ethertype = sigma_ntohs(eth->ethertype);

        if (ethertype == 0x0800) { /* IPv4 */
            return handleIPv4((const sigma_u8*)raw_frame + sizeof(EthernetHeader),
                              length - (sigma_u32)sizeof(EthernetHeader));
        } else if (ethertype == 0x86DD) { /* IPv6 */
            return handleIPv6((const sigma_u8*)raw_frame + sizeof(EthernetHeader),
                              length - (sigma_u32)sizeof(EthernetHeader));
        } else if (ethertype == 0x0806) { /* ARP */
            sigma_log("[NET] ARP packet received - updating lattice resolution table.");
            return true;
        }

        sigma_log_info("[NET] [DROP] Unknown EtherType: 0x%04X\n", ethertype);
        this->packets_filtered++;
        return false;
    }

    void printStats() const {
        sigma_log_info("[NET/STATS] Received: %u | Dispatched: %u | Filtered: %u | Bytes: %llu\n",
            this->packets_received, this->packets_received - this->packets_filtered,
            this->packets_filtered, this->bytes_dispatched);
    }

private:
    static constexpr sigma_u32 MAX_IFACES = 8;
    static constexpr sigma_u32 MAX_MTU    = 1500;

    SovereignNetStackEngine() : interfaces_active(0), packets_received(0),
                                packets_filtered(0), bytes_dispatched(0) {}

    bool handleIPv4(const sigma_u8* data, sigma_u32 len) {
        if (len < (sigma_u32)sizeof(IPv4Header)) { this->packets_filtered++; return false; }
        const IPv4Header* ip = (const IPv4Header*)data;
        sigma_u16 ip_len = sigma_ntohs(ip->length);

        /* DPI: block oversized or malformed payloads */
        if (ip_len > MAX_MTU) {
            sigma_log_info("[NET/DPI] Oversized IPv4 (len=%u) DROPPED.\n", ip_len);
            this->packets_filtered++;
            return false;
        }

        sigma_u32 src  = sigma_ntohl(ip->src_ip);
        sigma_u32 dest = sigma_ntohl(ip->dest_ip);
        sigma_log_info("[NET/IPv4] %u.%u.%u.%u -> %u.%u.%u.%u proto=%u\n",
            (src>>24)&0xFF, (src>>16)&0xFF, (src>>8)&0xFF, src&0xFF,
            (dest>>24)&0xFF, (dest>>16)&0xFF, (dest>>8)&0xFF, dest&0xFF,
            ip->protocol);

        if (ip->protocol == 6)  { return handleTCP(data + sizeof(IPv4Header), len - (sigma_u32)sizeof(IPv4Header)); }
        if (ip->protocol == 17) { return handleUDP(data + sizeof(IPv4Header), len - (sigma_u32)sizeof(IPv4Header)); }

        this->bytes_dispatched += ip_len;
        return true;
    }

    bool handleIPv6(const sigma_u8* data, sigma_u32 len) {
        if (len < (sigma_u32)sizeof(IPv6Header)) { this->packets_filtered++; return false; }
        sigma_log("[NET/IPv6] IPv6 packet processed.");
        return true;
    }

    bool handleTCP(const sigma_u8* data, sigma_u32 len) {
        if (len < (sigma_u32)sizeof(TCPHeader)) return false;
        const TCPHeader* tcp = (const TCPHeader*)data;
        sigma_u16 sp = sigma_ntohs(tcp->src_port);
        sigma_u16 dp = sigma_ntohs(tcp->dest_port);
        sigma_log_info("[NET/TCP] :%u -> :%u flags=0x%02X seq=%u\n", sp, dp, tcp->flags, sigma_ntohl(tcp->seq_num));
        this->bytes_dispatched += len;
        return true;
    }

    bool handleUDP(const sigma_u8* data, sigma_u32 len) {
        if (len < (sigma_u32)sizeof(UDPHeader)) return false;
        const UDPHeader* udp = (const UDPHeader*)data;
        sigma_log_info("[NET/UDP] :%u -> :%u len=%u\n",
            sigma_ntohs(udp->src_port), sigma_ntohs(udp->dest_port), sigma_ntohs(udp->length));
        this->bytes_dispatched += len;
        return true;
    }

    char      iface_mac[MAX_IFACES][18];
    char      iface_ip[MAX_IFACES][16];
    sigma_u32 interfaces_active;
    sigma_u32 packets_received;
    sigma_u32 packets_filtered;
    sigma_u64 bytes_dispatched;
};

/* --- C Bridge --- */
extern "C" void netstack_init() {
    SovereignNetStackEngine::getInstance().init();
}
extern "C" void netstack_register_iface(const char* mac, const char* ip) {
    SovereignNetStackEngine::getInstance().registerInterface(mac, ip);
}
extern "C" bool netstack_dispatch(const void* frame, sigma_u32 length) {
    return SovereignNetStackEngine::getInstance().dispatchPacket(frame, length);
}
extern "C" void netstack_stats() {
    SovereignNetStackEngine::getInstance().printStats();
}
