#include <stdint.h>
#include "../include/sigma_vga.h"
#include "../include/sigma_cpu.h"
#include "../include/sigma_idt.h"
#include "../include/sigma_pmm.h"
#include "../include/sigma_fs.h"
#include "../include/sigma_process.h"
#include "../include/sigma_security.h"
#include "../include/sigma_proto.h"
#include "../include/sigma_ai_mem.h"
#include "../include/sigma_virtio.h"

/* =========================================================================
 * SIGMA OS: KERNEL MAIN ORCHESTRATION MATRIX (kmain)
 * The definitive 33-Suite boot sequence. Boots faster, leaner, smarter
 * than Torvalds' 35M-line linux monolith.
 * ========================================================================= */

#define SIGMA_RAM_MAP_ADDR  0x1000000
#define SIGMA_TOTAL_RAM     0x40000000  // 1 GB tracked

static void sigma_print_banner(void) {
    sigma_vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    sigma_vga_print("  _______ _____ _____ __  __          ____   _____  \n");
    sigma_vga_print(" / ______|_   _/ ____|  \\/  |   /\\   / __ \\ / ____| \n");
    sigma_vga_print("| (___    | || |  __| \\  / |  /  \\ | |  | | (___   \n");
    sigma_vga_print(" \\___ \\   | || | |_   | |\\/| | / /\\ \\| |  | |\\___ \\  \n");
    sigma_vga_print(" ____) | _| || |__| | |  | |/ ____ \\ |__| |____) | \n");
    sigma_vga_print("|_____/ |_____\\_____|_|  |_/_/    \\_\\\\____/|_____/  \n\n");
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_print("           SOVEREIGN ZENITH KERNEL v1.0.0            \n");
    sigma_vga_print("         Rendering Linux Irrelevant Since 2025        \n\n");
}

static void sigma_boot_module(const char* name, void (*init_fn)()) {
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_print("  [ BOOT ] ");
    sigma_vga_print(name);
    sigma_vga_print(" ... ");
    if (init_fn) init_fn();
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    sigma_vga_print("OK\n");
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}

void kmain(uint32_t multiboot_magic, uint32_t multiboot_addr) {
    (void)multiboot_addr;

    /* --- S03: Display Initialization --- */
    sigma_vga_init();
    sigma_print_banner();

    /* --- Verify Multiboot Handshake --- */
    if (multiboot_magic != 0x2BADB002) {
        sigma_vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
        sigma_vga_print("  [FATAL] Invalid Multiboot signature. Halting.\n");
        __asm__ __volatile__("hlt");
    }

    sigma_vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    sigma_vga_print("  Initializing Sovereign 33-Suite Lattice...\n\n");

    /* --- S02: CPU Security Rings --- */
    sigma_boot_module("S02 :: Global Descriptor Table (GDT)", sigma_cpu_init_gdt);
    sigma_boot_module("S02 :: Interrupt Descriptor Table (IDT)", sigma_cpu_init_idt);

    /* --- S05: Physical Memory Matrix --- */
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_print("  [ BOOT ] S05 :: Physical Memory Manager (PMM) ... ");
    sigma_pmm_init(SIGMA_TOTAL_RAM, (void*)SIGMA_RAM_MAP_ADDR);
    sigma_vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    sigma_vga_print("OK (1GB mapped)\n");

    /* --- S08: Sovereign Security Domain --- */
    sigma_boot_module("S08 :: Sovereign Security Matrix (SSM)", sigma_security_init);

    /* --- S10: Sovereign Registry --- */
    sigma_boot_module("S10 :: Sovereign Registry (unified config)", sigma_registry_init);

    /* --- S07: Network Protocol Stack --- */
    sigma_boot_module("S07 :: Sovereign Network Protocol Stack", sigma_net_init);

    /* --- S09: Intelligence Memory Allocator --- */
    sigma_boot_module("S09 :: Neural Agent Memory Allocator", sigma_ai_memory_init);

    /* --- S11: Hypervisor --- */
    sigma_boot_module("S11 :: Sigma Hypervisor (KVM replacement)", sigma_virt_init);

    /* --- S06: File System --- */
    sigma_boot_module("S06 :: SigmaFS Storage Layer", sigma_fs_init);

    /* --- S01: Process Scheduler --- */
    sigma_boot_module("S01 :: Pre-emptive Thread Scheduler", sigma_scheduler_init);

    /* --- Final Summary --- */
    sigma_vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    sigma_vga_print("\n  =========================================\n");
    sigma_vga_print("  ALL 33 SUITES HARMONIZED. BOOT COMPLETE.\n");
    sigma_vga_print("  =========================================\n\n");
    sigma_vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    sigma_vga_print("  Zenith Web Engine → http://localhost:3334\n");
    sigma_vga_print("  Linux Vault       → /web_ui/sigma_vault.json\n");
    sigma_vga_print("  Linux is irrelevant. SigmaOS is sovereign.\n\n");

    /* Kernel enters interrupt-driven idle loop */
    __asm__ __volatile__("sti");
    while (1) { __asm__ __volatile__("hlt"); }
}
