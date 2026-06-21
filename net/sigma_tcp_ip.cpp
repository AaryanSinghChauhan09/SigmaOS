/*
 * Σ SigmaOS — sigma_tcp_ip: TCP/IP Stack & Zero-Copy Packet Queue (ZCLN)
 * Zero-Dependency.
 */

typedef unsigned int u32;
typedef unsigned short u16;
typedef unsigned char u8;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

struct TcpHeader {
    u16 src_port;
    u16 dest_port;
    u32 seq_num;
    u32 ack_num;
    u16 flags;
    u16 window_size;
    u16 checksum;
    u16 urgent_ptr;
};

struct ZclnPacketDescriptor {
    u64 buffer_phys_addr;
    u32 packet_len;
    u32 flags;
};

#define ZCLN_QUEUE_SIZE 128
static ZclnPacketDescriptor rx_queue[ZCLN_QUEUE_SIZE];
static u32 rx_head = 0;
static u32 rx_tail = 0;

extern "C" int sigma_net_zcln_enqueue(u64 phys_addr, u32 len) {
    u32 next = (rx_head + 1) % ZCLN_QUEUE_SIZE;
    if (next == rx_tail) return -1; // Queue Full
    
    rx_queue[rx_head].buffer_phys_addr = phys_addr;
    rx_queue[rx_head].packet_len = len;
    rx_queue[rx_head].flags = 1; // Present/Valid
    rx_head = next;
    
    sigma_vga_printf("[ZCLN] Enqueued zero-copy packet of length %d at 0x%llx\n", len, phys_addr);
    return 0;
}

extern "C" void sigma_tcp_process_packet(const u8* data, u32 len) {
    if (len < sizeof(TcpHeader)) return;
    const TcpHeader* tcp = (const TcpHeader*)data;
    sigma_vga_printf("[TCP/IP] Processing TCP segment: SrcPort:%d -> DestPort:%d (Seq:%u)\n",
                     tcp->src_port, tcp->dest_port, tcp->seq_num);
}
