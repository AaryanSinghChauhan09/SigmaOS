/*
 * Σ SigmaOS Zenith — dmesg (Kernel Ring Buffer) Utility
 * Absorbs: Linux dmesg, util-linux dmesg
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define KLOG_RING_SIZE 256
static char klog_ring[KLOG_RING_SIZE][128];
static u32  klog_head = 0;
static u32  klog_count = 0;

// Called by kernel subsystems to log events
extern "C" void sigma_dmesg_write(const char* msg) {
    int i = 0;
    while (msg[i] && i < 127) {
        klog_ring[klog_head % KLOG_RING_SIZE][i] = msg[i];
        i++;
    }
    klog_ring[klog_head % KLOG_RING_SIZE][i] = '\0';
    klog_head++;
    if (klog_count < KLOG_RING_SIZE) klog_count++;
}

extern "C" int sigma_dmesg_main(int argc, char** argv) {
    u32 start = (klog_count < KLOG_RING_SIZE) ? 0 : (klog_head % KLOG_RING_SIZE);
    for (u32 i = 0; i < klog_count; i++) {
        sigma_vga_printf("[%4u] %s\n", i, klog_ring[(start + i) % KLOG_RING_SIZE]);
    }
    return 0;
}
