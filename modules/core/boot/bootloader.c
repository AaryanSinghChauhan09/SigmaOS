#include <stdint.h>

// ---------------------------------------------------------
// SigmaOS Minimal Bootloader Prototype
// ---------------------------------------------------------

// Magic number to identify SigmaOS bootable image
#define SIGMA_BOOT_MAGIC 0x5161A05

typedef struct {
    uint32_t magic;
    uint32_t version;
    void* kernel_entry_point;
    uint32_t mem_size_kb;
} boot_info_t;

// Extern reference to the kernel entry (from kernel.c)
extern void kernel_main(boot_info_t* info);

// The actual bootloader entry point (jumped to by BIOS/UEFI)
void _start_bootloader() {
    // 1. Initialize basic hardware (VGA text mode, simple serial)
    // 2. Detect memory map
    
    boot_info_t info;
    info.magic = SIGMA_BOOT_MAGIC;
    info.version = 1;
    info.mem_size_kb = 1048576; // 1GB mock memory
    
    // 3. Relocate kernel (mocked here)
    // 4. Jump to kernel
    kernel_main(&info);
    
    // Should never reach here
    while(1);
}
