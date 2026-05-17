/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TRANSPORT LAYER (UDP)
 * =========================================================================
 */
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace Networking {
namespace Transport {

struct UDPHeader {
    sigma_u16 src_port;
    sigma_u16 dst_port;
    sigma_u16 length;
    sigma_u16 checksum;
};

class SovereignUDP {
public:
    void init() {
        sigma_log_info("[NET-TRANS] Initializing Sovereign UDP Protocol...");
    }

    sigma_status send(sigma_u16 src_port, sigma_u16 dst_port, sigma_u8* payload, sigma_size_t payload_len) {
        sigma_size_t total_len = sizeof(UDPHeader) + payload_len;
        sigma_log_info("[NET-TRANS] UDP TX: SRC=%d DST=%d LEN=%d", src_port, dst_port, total_len);
        
        // TODO: Construct packet, calculate checksum, pass down to IP layer
        return 0; // SIGMA_OK
    }

    void receive(sigma_u8* packet, sigma_size_t length) {
        if (length < sizeof(UDPHeader)) {
            sigma_log_error("[NET-TRANS] UDP RX Error: Packet too small");
            return;
        }
        
        UDPHeader* hdr = reinterpret_cast<UDPHeader*>(packet);
        sigma_log_info("[NET-TRANS] UDP RX: SRC=%d DST=%d LEN=%d", hdr->src_port, hdr->dst_port, hdr->length);
        
        // TODO: Demultiplex to listening sockets
    }
};

} // namespace Transport
} // namespace Networking
} // namespace SigmaOS
