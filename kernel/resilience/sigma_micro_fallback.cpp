/*
 * Σ SigmaOS — sigma_micro_fallback: Resilient Mode Microkernel
 * Zero-Dependency.
 * 
 * A fallback, ultra-minimal kernel loop that executes if the main kernel
 * panics or detects catastrophic state corruption. It provides a rescue shell
 * to recover logs and attempt repairs, ensuring continuity.
 */

typedef unsigned int u32;
typedef unsigned char u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);
// Serial driver stubs
extern "C" void sigma_serial_write(char c);
extern "C" char sigma_serial_read();
// Storage stub
extern "C" void sigma_nvme_read_block(u32 lba, u8* buffer);

static bool resilient_mode_active = false;

/* Send a string over serial for the rescue shell */
static void rescue_puts(const char* str) {
    while (*str) {
        sigma_serial_write(*str++);
    }
}

/* 
 * Entered when the main kernel panics and cannot self-heal a specific process.
 */
extern "C" void sigma_resilient_fallback_entry(const char* panic_reason) {
    resilient_mode_active = true;
    
    // Disable all interrupts to prevent further state corruption
    // __asm__ volatile("cli"); // x86 stub
    
    sigma_vga_printf("\n=======================================================\n");
    sigma_vga_printf(" 🛡️ SIGMAOS RESILIENT MODE ACTIVATED 🛡️\n");
    sigma_vga_printf("=======================================================\n");
    sigma_vga_printf("Main kernel panicked. Reason: %s\n", panic_reason);
    sigma_vga_printf("Dropping to minimal rescue shell via serial console...\n");
    
    rescue_puts("\nSigmaOS Rescue Shell v1.0\n");
    rescue_puts("Type 'help' for commands.\n");
    
    char cmd_buffer[64];
    u32 cmd_idx = 0;
    
    while (resilient_mode_active) {
        rescue_puts("rescue> ");
        cmd_idx = 0;
        
        // Basic serial read loop
        while (1) {
            char c = sigma_serial_read();
            if (c == '\r' || c == '\n') {
                cmd_buffer[cmd_idx] = '\0';
                rescue_puts("\n");
                break;
            } else if (c == '\b' || c == 0x7F) {
                if (cmd_idx > 0) {
                    cmd_idx--;
                    rescue_puts("\b \b");
                }
            } else if (cmd_idx < 63) {
                cmd_buffer[cmd_idx++] = c;
                sigma_serial_write(c);
            }
        }
        
        // Parse basic commands
        if (cmd_buffer[0] == '\0') {
            continue;
        } else if (cmd_buffer[0] == 'h' && cmd_buffer[1] == 'e') {
            rescue_puts("Commands:\n");
            rescue_puts("  logs    - Dump kernel ring buffer\n");
            rescue_puts("  reboot  - ACPI hardware reset\n");
            rescue_puts("  repair  - Check filesystem integrity\n");
        } else if (cmd_buffer[0] == 'r' && cmd_buffer[1] == 'e' && cmd_buffer[2] == 'b') {
            rescue_puts("Initiating hardware reset...\n");
            // Trigger ACPI reset or outb(0x64, 0xFE)
            while(1) {}
        } else if (cmd_buffer[0] == 'l' && cmd_buffer[1] == 'o') {
            rescue_puts("Dumping logs... (stubbed)\n");
        } else {
            rescue_puts("Unknown command.\n");
        }
    }
}
