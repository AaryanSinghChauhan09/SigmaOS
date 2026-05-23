/*
 * Σ SigmaOS — sigma_browser: Minimal Text-Mode Browser
 * Zero-Dependency: No predefined libraries.
 * Parses raw HTML manually and outputs text directly.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" u32  sigma_fat32_read(const char* name, unsigned char* buf, u32 max);

/* Sovereign HTML Parser State */
enum ParseState {
    TEXT,
    TAG,
    COMMENT
};

/*
 * Parse HTML string and dump formatted text to screen
 */
static void render_html(const char* html_data) {
    ParseState state = TEXT;
    const char* ptr = html_data;

    while (*ptr) {
        if (state == TEXT) {
            if (*ptr == '<') {
                if (ptr[1] == '!' && ptr[2] == '-' && ptr[3] == '-') {
                    state = COMMENT;
                    ptr += 3;
                } else {
                    state = TAG;
                }
            } else {
                sigma_vga_putchar(*ptr);
            }
        } else if (state == TAG) {
            if (*ptr == '>') {
                state = TEXT;
            }
            /* Simplistic: ignore tag contents completely */
        } else if (state == COMMENT) {
            if (ptr[0] == '-' && ptr[1] == '-' && ptr[2] == '>') {
                state = TEXT;
                ptr += 2;
            }
        }
        ptr++;
    }
}

/* 
 * Minimal Browser Entry Point
 */
extern "C" int sigma_browser_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_puts("Usage: browser <file.html>\n");
        return 1;
    }

    const char* filename = argv[1];
    
    /* Allocate arbitrary chunk for the HTML document */
    unsigned char file_buffer[8192];
    u32 bytes_read = sigma_fat32_read(filename, file_buffer, sizeof(file_buffer) - 1);
    
    if (bytes_read == 0) {
        sigma_vga_puts("Error: Could not open or read HTML file.\n");
        return 1;
    }
    
    file_buffer[bytes_read] = '\0'; /* Ensure null-termination */

    sigma_vga_puts("--- SigmaBrowser Rendering ---\n");
    render_html((const char*)file_buffer);
    sigma_vga_puts("\n------------------------------\n");

    return 0;
}
