#include <stdint.h>
#include "../include/sigma_vga.h"
#include "../include/sigma_cpu.h"
#include "../include/sigma_idt.h"
#include "../include/sigma_pmm.h"

/* =========================================================================
 * SIGMA OS: ABSOLUTE KERNEL ENTRY MATRIX (kmain)
 * Synchronizes all low-level shards and hands off execution into the void.
 * ========================================================================= */

// We define a bare-metal placeholder memory block for our PMM bitmap.
// In reality, this would dynamically allocate post-GRUB e820 analysis.
#define SIMULATED_BITMAP_ADDR 0x1000000 
#define SIMULATED_TOTAL_RAM 0x40000000 // 1GB of tracked RAM

void kmain(uint32_t magic, uint32_t addr) {
    // 1. Initialize Display Subsystems
    sigma_vga_init();
    sigma_vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    sigma_vga_print("========================================================\n");
    sigma_vga_print("        [ SIGMA OS SOVEREIGN KERNEL BOOT SEQUENCE ]     \n");
    sigma_vga_print("========================================================\n\n");

    // 2. Load Core CPU Security Rings (Step 4)
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_print("[SYSTEM] Bootstrapping Global Descriptor Table (GDT)... ");
    sigma_cpu_init_gdt();
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    sigma_vga_print("OK\n");

    // 3. Load Hardware Interrupt Matrix (Step 4 Extended)
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_print("[SYSTEM] Loading Interrupt Descriptor Table (IDT)...    ");
    sigma_cpu_init_idt();
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    sigma_vga_print("OK\n");

    // 4. Initialize Silicon Memory Maps (Step 5)
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_print("[SYSTEM] Structuring Physical Memory Manager (PMM)...   ");
    sigma_pmm_init(SIMULATED_TOTAL_RAM, (void*)SIMULATED_BITMAP_ADDR);
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    sigma_vga_print("OK\n\n");

    // Final Hand-off Log
    sigma_vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    sigma_vga_print("[SIGMA] Kernel successfully deployed across bare-metal.\n");
    sigma_vga_print("[SIGMA] Awaiting user-land Web-Engine Orchestrator hooks...\n");

    // Infinite Yield state. OS runs off interrupts now.
    while (1) {
        __asm__ __volatile__("hlt");
    }
}
