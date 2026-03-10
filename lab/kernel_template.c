/*
 * Sovereign-Core v3.0: Minimal C Kernel
 * Targeting: i686-elf-gcc
 */

void kernel_main() {
    volatile char* vga_buffer = (volatile char*) 0xB8000;
    const char* str = "Sovereign-Core v3.0: Ring-0 Active.";
    
    // Clear Screen
    for(int i = 0; i < 80 * 25 * 2; i++) {
        vga_buffer[i] = 0;
    }
    
    // Print Hello
    for(int i = 0; str[i] != '\0'; i++) {
        vga_buffer[i*2] = str[i];
        vga_buffer[i*2+1] = 0x07; // Light Gray on Black
    }
    
    while(1); // Halt
}
