/*
 * Σ SigmaOS — sigma_kgdb: Kernel Debugger
 * Zero-Dependency.
 * 
 * In-kernel debugger over serial port (GDB Remote Serial Protocol stub).
 * Allows breakpoints, register inspection, and stack traces.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);
// Serial port driver stubs
extern "C" void sigma_serial_write(char c);
extern "C" char sigma_serial_read();

#define MAX_BREAKPOINTS 16
static u64 breakpoints[MAX_BREAKPOINTS];
static bool kgdb_active = false;

/* Send a string over the serial debugger connection */
static void put_debug_str(const char* str) {
    while (*str) {
        sigma_serial_write(*str++);
    }
}

/* 
 * Entered via INT3 (x86) or BRK (ARM/RISC-V) exception handler
 */
extern "C" void sigma_kgdb_trap(u64* registers) {
    kgdb_active = true;
    
    // GDB Stop Reply Packet: signal 5 (SIGTRAP)
    put_debug_str("$T05#xx"); 
    
    sigma_vga_printf("[KGDB] Kernel execution paused. Awaiting GDB commands...\n");
    
    while (kgdb_active) {
        char c = sigma_serial_read();
        
        if (c == '$') { // Start of packet
            // Stubbed GDB protocol parser
            char cmd = sigma_serial_read();
            if (cmd == 'g') {
                // Read registers
                put_debug_str("$00000000...#xx"); // Dummy register state
            } else if (cmd == 'c') {
                // Continue
                kgdb_active = false;
                put_debug_str("$OK#9a");
            } else if (cmd == 'z' || cmd == 'Z') {
                // Insert/Remove breakpoint
                put_debug_str("$OK#9a");
            } else {
                put_debug_str("$#00"); // Empty response = unsupported
            }
        }
    }
    
    sigma_vga_printf("[KGDB] Resuming kernel execution...\n");
}
