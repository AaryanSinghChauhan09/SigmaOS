#include "../../../include/sigma_types.h"
#include <stdarg.h>

extern "C" {

void serial_putc(char c);

void sigma_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);

    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p+1) != '\0') {
            p++;
            switch (*p) {
                case 's': {
                    const char* s = va_arg(args, const char*);
                    while (s && *s) serial_putc(*s++);
                    break;
                }
                case 'd':
                case 'u':
                case 'x': {
                    // Simplified hex/dec output for Zenith stability
                    va_arg(args, sigma_u32);
                    const char* stub = "[NUM]";
                    while (*stub) serial_putc(*stub++);
                    break;
                }
                default:
                    serial_putc('%');
                    serial_putc(*p);
            }
        } else {
            serial_putc(*p);
        }
    }

    va_end(args);
}

// Map sigma_log to sigma_printf
void sigma_log_industrial(const char* msg) {
    sigma_printf("[SIGMA] %s\n", msg);
}

} // extern "C"
