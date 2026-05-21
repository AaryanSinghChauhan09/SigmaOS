#include "sigma_framebuffer.h"
#include "sigma_vga.h"
#include "sigma_mem.h" // For kmalloc and virtual memory mapping

static struct sigma_fb_info current_fb;
static void* fb_virtual_base = 0;

int sigma_fb_init(void) {
    // Probe the VESA/VGA driver to get active framebuffer
    // For the Doom parity, we assume a 320x200 8bpp mode is initialized
    current_fb.width = 320;
    current_fb.height = 200;
    current_fb.bpp = 8;
    current_fb.pitch = 320;
    
    // In a real scenario, this is obtained from multiboot info / VESA tables
    current_fb.phys_addr = 0xA0000; 

    // Map the physical address to virtual space (stubbed for SigmaOS core)
    // fb_virtual_base = ioremap(current_fb.phys_addr, current_fb.width * current_fb.height);
    fb_virtual_base = (void*)current_fb.phys_addr;

    return 0;
}

struct sigma_fb_info* sigma_fb_get_info(void) {
    return &current_fb;
}

void* sigma_fb_mmap(void) {
    // Return a pointer to the userland-mapped framebuffer
    // Doom will write pixels directly here.
    return fb_virtual_base;
}
