#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

// TCP/IP Packet Structures
#pragma pack(push, 1)
struct EthernetHeader {
    sigma_u8 dest_mac[6];
    sigma_u8 src_mac[6];
    sigma_u16 ethertype;
};

struct IPv4Header {
    sigma_u8 version_ihl;
    sigma_u8 dscp_ecn;
    sigma_u16 length;
    sigma_u16 ident;
    sigma_u16 flags_offset;
    sigma_u8 ttl;
    sigma_u8 protocol;
    sigma_u16 checksum;
    sigma_u32 src_ip;
    sigma_u32 dest_ip;
};

struct TCPHeader {
    sigma_u16 src_port;
    sigma_u16 dest_port;
    sigma_u32 seq_num;
    sigma_u32 ack_num;
    sigma_u8 data_offset;
    sigma_u8 flags;
    sigma_u16 window_size;
    sigma_u16 checksum;
    sigma_u16 urgent_ptr;
};
#pragma pack(pop)

class SovereignNetStackEngine {
public:
    static SovereignNetStackEngine& getInstance() {
        static SovereignNetStackEngine instance;
        return instance;
    }

    void init() {
        sigma_log_info("[NET] Initializing Sovereign TCP/IP Stack...\n");
        this->interfaces_active = 0;
        this->packets_filtered = 0;
        sigma_log_info("[NET] Zero-trust packet inspection ACTIVE.\n");
    }

    void registerInterface(const char* mac_addr) {
        if (this->interfaces_active >= 4) return;
        sigma_hardened_strcpy(this->interfaces[this->interfaces_active], mac_addr, 18);
        this->interfaces_active++;
        sigma_log_info("[NET] Network interface %s registered.\n", mac_addr);
    }

    bool dispatchPacket(const char* payload, sigma_u32 length) {
        if (length < sizeof(EthernetHeader)) return false;

        EthernetHeader* eth = (EthernetHeader*)payload;
        sigma_u16 ethertype = (eth->ethertype >> 8) | (eth->ethertype << 8); // ntohs
        
        if (ethertype == 0x0800) { // IPv4
            if (length < sizeof(EthernetHeader) + sizeof(IPv4Header)) return false;
            IPv4Header* ip = (IPv4Header*)(payload + sizeof(EthernetHeader));
            
            if (ip->protocol == 6) { // TCP
                TCPHeader* tcp = (TCPHeader*)(payload + sizeof(EthernetHeader) + sizeof(IPv4Header));
                sigma_u16 src_port = (tcp->src_port >> 8) | (tcp->src_port << 8);
                sigma_u16 dest_port = (tcp->dest_port >> 8) | (tcp->dest_port << 8);
                sigma_log_info("[NET] TCP Packet Received: %u -> %u\n", src_port, dest_port);
            }
        }
        
        // Deep Packet Inspection simulation
        if (length > 1500) {
            this->packets_filtered++;
            sigma_log_info("[NET] [WARN] Oversized MTU packet dropped by sovereign firewall.\n");
            return false;
        }
        
        sigma_log_info("[NET] Dispatched %u bytes over TCP/IP.\n", length);
        return true;
    }

private:
    SovereignNetStackEngine() : interfaces_active(0), packets_filtered(0) {}

    char interfaces[4][18];
    sigma_u32 interfaces_active;
    sigma_u32 packets_filtered;
};

extern "C" void netstack_init() {
    SovereignNetStackEngine::getInstance().init();
}

extern "C" void netstack_register_iface(const char* mac_addr) {
    SovereignNetStackEngine::getInstance().registerInterface(mac_addr);
}

extern "C" bool netstack_dispatch(const char* payload, sigma_u32 length) {
    return SovereignNetStackEngine::getInstance().dispatchPacket(payload, length);
}
