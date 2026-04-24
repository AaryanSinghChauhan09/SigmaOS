#include "suites/S01_Genesis/shards/sigma_kernel.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

/* =========================================================================
 * SIGMA OS: KERNEL MAIN ORCHESTRATION MATRIX (kmain)
 * The definitive 33-Suite boot sequence. 
 * ========================================================================= */

void kmain(uint32_t multiboot_magic, uint32_t multiboot_addr) {
    (void)multiboot_addr;

    /* --- S03: Early Display Initialization --- */
    // sigma_vga_init(); // Should be part of Halo or Genesis

    /* --- Verify Multiboot Handshake --- */
    if (multiboot_magic != 0x2BADB002) {
        // Handle fatal boot error
        __asm__ __volatile__("hlt");
    }

    /* --- Sovereign 33-Suite Lattice Materialization --- */
    SovereignMaster_InitAll();

    // Materialize Industrial Subsystems
    s_audio_init();
    s_graphics_init();

    sigma_sigma_printf("\n  =========================================\n");
    sigma_sigma_printf("  ALL 33 SUITES HARMONIZED. BOOT COMPLETE.\n");
    sigma_sigma_printf("  =========================================\n\n");
    
    // Demonstrate Userland & System Parity
    s_security_audit_all();
    s_pkg_list();
    s_ls("/");

    sigma_sigma_printf("\n  Zenith Web Engine -> http://localhost:3334\n");
    sigma_sigma_printf("  Linux is irrelevant. SigmaOS is sovereign.\n\n");

    /* Kernel enters interrupt-driven idle loop */
    __asm__ __volatile__("sti");
    while (1) { __asm__ __volatile__("hlt"); }
}

