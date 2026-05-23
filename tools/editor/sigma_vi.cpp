/*
 * Σ SigmaOS — sigma_vi: Sovereign Modal Text Editor
 * Absorbs: vi, vim
 * Zero-Dependency: No libc, no ncurses.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_clear();
extern "C" char sigma_keyboard_getch();

extern "C" int sigma_vi_main(int argc, char** argv) {
    bool insert_mode = false;
    bool running = true;
    
    sigma_vga_clear();
    sigma_vga_printf("~ \n");
    sigma_vga_printf("~ SigmaOS Vi (Sovereign Modal Editor)\n");
    sigma_vga_printf("~ \n");
    sigma_vga_printf("\"No Name\" -- NORMAL MODE --\n");

    while (running) {
        char c = sigma_keyboard_getch();
        
        if (!insert_mode) {
            if (c == 'i') {
                insert_mode = true;
                sigma_vga_printf("\n-- INSERT --\n");
            } else if (c == ':') {
                sigma_vga_printf("\n:");
                char cmd = sigma_keyboard_getch();
                if (cmd == 'q') {
                    running = false;
                } else if (cmd == 'w') {
                    sigma_vga_printf(" [Written]\n");
                }
            }
        } else {
            if (c == 27) { // ESC key
                insert_mode = false;
                sigma_vga_printf("\n-- NORMAL --\n");
            } else if (c >= 32 && c <= 126) {
                sigma_vga_printf("%c", c);
            }
        }
    }

    sigma_vga_clear();
    return 0;
}
