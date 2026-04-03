/* 
 Σ SIGMAOS ZENITH: BARE-METAL KERNEL ENTRY (v1500.0)
 Mission: Real Hardware Initialization & VGA Console Emit. 
*/

#include "SigmaSovereignInternal.h"

// Σ HARDWARE BINDING: VGA TEXT BUFFER
static volatile unsigned short* VGA_BUFFER = (unsigned short*)0xB8000;
static const int VGA_WIDTH = 80;
static const int VGA_HEIGHT = 25;

void sigma_printk(const char* s, unsigned char color) {
    static int cursor = 0;
    while (*s) {
        VGA_BUFFER[cursor++] = (unsigned short)((color << 8) | *s++);
        if (cursor >= VGA_WIDTH * VGA_HEIGHT) cursor = 0;
    }
}

// Σ THE BARE-METAL KERNEL ENTRY POINT
void kmain(void) {
    // 1. Clear Screen
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
        VGA_BUFFER[i] = (unsigned short)((0x07 << 8) | ' ');
    }

    // 2. Emit Sovereign Boot Logo
    sigma_printk("Σ SIGMAOS ZENITH : BARE-METAL SOVEREIGNTY ACHIEVED (v1500.0)\n", 0x0F);
    sigma_printk("Σ [BOOT]: GDT/IDT Validated (Stub)\n", 0x0A);
    sigma_printk("Σ [BOOT]: VGA Hardware Console Active\n", 0x0B);

    // 3. Infinite Sleep (Transition to Scheduler)
    while (1) {
        __asm__ volatile ("hlt");
    }
}
