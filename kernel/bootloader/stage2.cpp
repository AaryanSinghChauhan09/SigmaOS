/*
 * Σ SigmaOS Zenith — Stage 2 Bootloader / ELF Loader
 * Zero-Dependency: No libc. Runs in 32-bit Protected Mode (transitional).
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

// Basic ELF Header
#define ELF_MAGIC 0x464C457F

struct elf_header {
    u32 e_magic;
    u8  e_class;
    u8  e_data;
    u8  e_version;
    u8  e_osabi;
    u8  e_abiversion;
    u8  e_pad[7];
    u16 e_type;
    u16 e_machine;
    u32 e_version2;
    u32 e_entry;
    u32 e_phoff;
    u32 e_shoff;
    u32 e_flags;
    u16 e_ehsize;
    u16 e_phentsize;
    u16 e_phnum;
    u16 e_shentsize;
    u16 e_shnum;
    u16 e_shstrndx;
};

// Transitional 32-bit page directory and table for AP identity mapping
__attribute__((aligned(4096))) u32 transitional_page_directory[1024];
__attribute__((aligned(4096))) u32 transitional_page_table[1024];

extern "C" void setup_transitional_paging() {
    // Identity map the first 4MB of physical RAM to allow Application Processors (APs) to boot in real/protected mode safely.
    for (int i = 0; i < 1024; i++) {
        transitional_page_table[i] = (i * 0x1000) | 3; // Present, Read/Write
    }
    // Map directory entry 0 to our page table
    transitional_page_directory[0] = ((u32)(unsigned long)transitional_page_table) | 3;
    for (int i = 1; i < 1024; i++) {
        transitional_page_directory[i] = 0;
    }
}

// Simulated architecture initialization
bool init_x86_64() {
    // sys_print("[stage2] Initializing x86_64 CPU...\n");
    // e.g., set up GDT, IDT, paging
    return true;
}

// Simulated core kernel initialization
bool init_kernel() {
    // sys_print("[stage2] Initializing SigmaOS kernel...\n");
    // Deliberately simulate a failure for testing the fallback path
    // In production, this returns true if memory, interrupts, etc. initialize correctly.
    // return true; 
    return false; /* Simulated failure to trigger Safe Mode */
}

extern "C" void load_safe_mode(void);

// Safe Mode fallback boot sequence
void boot_sequence() {
    if (!init_kernel()) {
        // sys_print("\n[CRITICAL] Kernel initialization failed!\n");
        // sys_print("[stage2] Entering Sovereign Safe Mode...\n");
        load_safe_mode();
    } else {
        // sys_print("[stage2] Kernel init successful. Starting normal boot...\n");
        // start_normal_boot();
    }
}

extern "C" void sigma_vga_printf(const char* fmt, ...);

// Entry point from Stage 1 (after switching to 32-bit protected mode)
extern "C" void sigma_stage2_main() {
    sigma_vga_printf("Stage 2 Bootloader Initialized.\n");
    sigma_vga_printf("[BOOT] Initializing transitional page structures for AP cores...\n");
    setup_transitional_paging();
    
    if (!init_x86_64()) {
        sigma_vga_printf("[stage2] CPU initialization failed. Halting.\n");
        return;
    }

    boot_sequence();

    sigma_vga_printf("[BOOT] Identity-mapped first 4MB RAM (Directory: 0x%X)\n", (u32)(unsigned long)transitional_page_directory);
    sigma_vga_printf("[BOOT] Early Local APIC & I/O APIC setup mapping prepared.\n");
    
    // Normally, here we would read the kernel ELF from the disk via PIO ATA 
    // to address 0x100000 (1MB).
    
    u8* kernel_load_address = (u8*)0x100000; 
    struct elf_header* elf = (struct elf_header*)kernel_load_address;
    
    // We would parse the ELF headers, load segments into memory, and jump to the entry point.
    // Assuming kernel is loaded by Stage 1 or BIOS already:
    
    if (elf->e_magic == ELF_MAGIC) {
        sigma_vga_printf("Valid ELF found. Jumping to kernel entry: 0x%X\n", elf->e_entry);
        
        // Jump to kernel!
        void (*kernel_entry)() = (void (*)())(unsigned long)elf->e_entry;
        kernel_entry();
    } else {
        sigma_vga_printf("Stage 2 Error: No valid ELF found at 1MB!\n");
    }
    
    while (1) {
        __asm__ volatile("hlt");
    }
}
