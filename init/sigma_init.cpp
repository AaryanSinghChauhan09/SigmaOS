/*
 * Σ SigmaOS Zenith — Sovereign Init System (S01_Genesis)
 * Absorbs: systemd init concept, BusyBox init simplicity, Arch Linux philosophy
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 * 
 * This is the sovereign init process (PID 1). It replaces systemd/sysvinit/runit.
 * Unlike systemd (2M LOC + 100 dependencies), S01_Genesis is < 200 lines
 * and brings up every subsystem deterministically.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── External Kernel Services ─────────────── */
/* Resolved by the linker from respective shard .cpp files */
extern "C" void sigma_vga_init();
extern "C" void sigma_vga_puts(const char* str);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_set_color(int fg, int bg);
extern "C" void sigma_slab_init();
extern "C" void sigma_kbd_init();
extern "C" void sigma_mouse_init();
extern "C" bool sigma_ata_read_sector(u32 lba, u8* buffer);
extern "C" bool sigma_fat32_mount(u32 start_lba);
extern "C" bool sigma_e1000_init(u64 mmio_base);
extern "C" void sigma_rt_scheduler_init();
extern "C" void sigma_registry_init();
extern "C" void sigma_sh_run();

/* ─────────────── Boot Stage Definitions ─────────────── */
typedef bool (*InitFunc)();

struct BootStage {
    const char* name;
    InitFunc    func;
    bool        critical;  /* If true, boot halts on failure */
};

/* ─────────────── Init Wrappers (adapt void→bool for uniform dispatch) ─────────────── */
static bool init_vga() {
    sigma_vga_init();
    return true;
}

static bool init_slab() {
    sigma_slab_init();
    return true;
}

static bool init_kbd() {
    sigma_kbd_init();
    return true;
}

static bool init_mouse() {
    sigma_mouse_init();
    return true;
}

static bool init_fs() {
    return sigma_fat32_mount(0);
}

static bool init_net() {
    /* PCI enumeration would resolve this dynamically */
    /* Using placeholder MMIO base; real boot would probe PCI BAR */
    return sigma_e1000_init(0xFEBC0000ULL);
}

static bool init_scheduler() {
    sigma_rt_scheduler_init();
    return true;
}

static bool init_registry() {
    sigma_registry_init();
    return true;
}

/* ─────────────── Boot Sequence (Deterministic, Ordered) ─────────────── */
static struct BootStage boot_sequence[] = {
    { "VGA Display",            init_vga,       true  },
    { "Slab Allocator",         init_slab,      true  },
    { "PS/2 Keyboard",          init_kbd,       true  },
    { "PS/2 Mouse",             init_mouse,     false },
    { "FAT32 Filesystem",       init_fs,        false },
    { "Intel e1000 NIC",        init_net,       false },
    { "RT Scheduler",           init_scheduler, true  },
    { "Configuration Registry", init_registry,  false },
};

#define BOOT_STAGE_COUNT (sizeof(boot_sequence) / sizeof(boot_sequence[0]))

/* ─────────────── Sovereign Panic ─────────────── */
static void sovereign_panic(const char* message) {
    sigma_vga_set_color(12, 0); /* Red on Black */
    sigma_vga_puts("\n!!! SOVEREIGN KERNEL PANIC !!!\n");
    sigma_vga_puts(message);
    sigma_vga_puts("\nSystem halted.\n");
    __asm__ volatile ("cli; hlt");
}

/* ─────────────── API: Boot Entry Point (PID 1) ─────────────── */
extern "C" void sigma_init() {
    /* Stage 0: VGA must come first so we can print */
    sigma_vga_init();

    /* Banner */
    sigma_vga_set_color(11, 0); /* Light Cyan on Black */
    sigma_vga_puts("\n");
    sigma_vga_puts("  ========================================\n");
    sigma_vga_puts("    Sigma SigmaOS Zenith v15.2\n");
    sigma_vga_puts("    S01_Genesis Init System (PID 1)\n");
    sigma_vga_puts("  ========================================\n\n");

    sigma_vga_set_color(7, 0); /* Light Grey on Black */

    /* Execute boot stages sequentially */
    u32 passed = 0;
    u32 failed = 0;

    for (u32 i = 0; i < BOOT_STAGE_COUNT; i++) {
        sigma_vga_printf("  [%u/%u] Initializing %s... ",
                         i + 1, (u32)BOOT_STAGE_COUNT, boot_sequence[i].name);

        bool result = boot_sequence[i].func();

        if (result) {
            sigma_vga_set_color(10, 0); /* Green */
            sigma_vga_puts("[OK]\n");
            passed++;
        } else {
            if (boot_sequence[i].critical) {
                sigma_vga_set_color(12, 0); /* Red */
                sigma_vga_puts("[FAIL - CRITICAL]\n");
                sovereign_panic(boot_sequence[i].name);
            } else {
                sigma_vga_set_color(14, 0); /* Yellow */
                sigma_vga_puts("[WARN - SKIPPED]\n");
                failed++;
            }
        }
        sigma_vga_set_color(7, 0); /* Reset to grey */
    }

    /* Boot summary */
    sigma_vga_puts("\n");
    sigma_vga_set_color(11, 0);
    sigma_vga_printf("  Boot complete: %u passed, %u skipped\n\n", passed, failed);
    sigma_vga_set_color(7, 0);

    /* Drop into Sovereign Shell */
    sigma_vga_puts("  Dropping to sigma-sh...\n");
    sigma_sh_run();

    /* If shell ever exits */
    sovereign_panic("sigma_sh returned unexpectedly");
}
