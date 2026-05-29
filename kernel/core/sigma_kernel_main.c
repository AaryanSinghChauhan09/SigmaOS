#include "sigma_kernel_types.h"
#include "sigma_slab.h"
#include "sigma_mlfq.h"
#include "sigma_pit.h"
#include "sigma_keyboard.h"

// Externally defined functions in existing HAL/drivers
extern void vga_putc(char c, sigma_u8 color);
extern void vga_scroll(void);
extern void serial_init(void);
extern void serial_puts(const char* s);
extern void idt_init(void);

// Our new PIC init
extern void sigma_pic_init(int offset1, int offset2);

// Boot resilience hooks
extern int sigma_rollback_check_fallback();
extern void sigma_rollback_mark_boot_successful();
extern void sigma_resilient_fallback_entry(const char* panic_reason);

static void vga_puts(const char* s) {
    while (*s) {
        vga_putc(*s++, 0x07); // 0x07 is light grey on black
    }
}

// Dummy tasks for testing the scheduler
void task_a() {
    while(1) {
        serial_puts("Task A running\n");
        sigma_sleep_ms(500);
    }
}

void task_b() {
    while(1) {
        serial_puts("Task B running\n");
        sigma_sleep_ms(500);
    }
}

static uint8_t stack_a[4096];
static uint8_t stack_b[4096];

void sigma_kernel_main(void* multiboot_info, uint32_t magic) {
    // 1. Initialize serial debugging
    serial_init();
    serial_puts("Î£ SigmaOS Zenith Booting...\n");

    // 1.1 Rollback gate: if repeated boots failed, force resilient mode.
    if (sigma_rollback_check_fallback() != 0) {
        serial_puts("[BOOT] Rollback requested. Entering resilient safe mode.\n");
        sigma_resilient_fallback_entry("Rollback gate requested safe mode");
        while (1) { __asm__ volatile("hlt"); }
    }

    // 2. Clear VGA (optional, assuming handled or we just print)
    vga_puts("Î£ SigmaOS Zenith Kernel Initializing\n");
    
    // 3. Init Interrupts (IDT)
    idt_init();
    
    // 4. Remap PIC to avoid conflict with CPU exceptions (remap to 32+)
    sigma_pic_init(32, 40);
    serial_puts("[HAL] PIC remapped\n");
    
    // 5. Init memory
    sigma_slab_init();
    serial_puts("[MEM] Slab Allocator Initialized\n");
    
    // 6. Init PIT (timer)
    sigma_pit_init(1000);
    serial_puts("[HAL] PIT Timer Initialized at 1000Hz\n");
    
    // 7. Init keyboard
    sigma_keyboard_init();
    serial_puts("[HAL] PS/2 Keyboard Initialized\n");
    
#if defined(SIGMA_MINIMAL_MODE) && (SIGMA_MINIMAL_MODE != 0)
    serial_puts("[BOOT] SIGMA_MINIMAL_MODE active. Skipping scheduler/tasks.\n");
    sigma_rollback_mark_boot_successful();
#else
    // 8. Init MLFQ Scheduler
    sigma_sched_init();
    serial_puts("[SCHED] MLFQ Scheduler Initialized\n");
    
    // Add dummy tasks
    sigma_sched_add_task((void*)task_a, stack_a + 4096);
    sigma_sched_add_task((void*)task_b, stack_b + 4096);
    
    // Enable interrupts
    __asm__ volatile("sti");
    serial_puts("[SYS] Interrupts Enabled\n");
    vga_puts("System Ready. Waiting for input...\n");

    // Mark this boot as stable enough to be "known-good".
    sigma_rollback_mark_boot_successful();
#endif

    // Main idle loop (reading keyboard input and echoing it)
    while (1) {
        char c = sigma_keyboard_read();
        if (c) {
            vga_putc(c, 0x0A); // Echo in green
            serial_puts("Key pressed\n");
        }
        __asm__ volatile("hlt");
    }
}
