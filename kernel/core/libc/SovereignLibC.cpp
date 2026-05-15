#include "../../../include/SovereignLibC.h"
#include "../../../include/core/sigma_types.h"
#include <stdarg.h>

extern "C" {

void serial_putc(char c);

int sigma_printf(const char* format, ...) {
    int count = 0;
    va_list args;
    va_start(args, format);

    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p+1) != '\0') {
            p++;
            switch (*p) {
                case 's': {
                    const char* s = va_arg(args, const char*);
                    while (s && *s) { serial_putc(*s++); count++; }
                    break;
                }
                case 'd':
                case 'u':
                case 'x': {
                    // Simplified hex/dec output for Zenith stability
                    va_arg(args, sigma_u32);
                    const char* stub = "[NUM]";
                    while (*stub) { serial_putc(*stub++); count++; }
                    break;
                }
                default:
                    serial_putc('%');
                    serial_putc(*p);
                    count += 2;
            }
        } else {
            serial_putc(*p);
            count++;
        }
    }

    va_end(args);
    return count;
}

// Map sigma_log to sigma_printf
void sigma_log_industrial(const char* msg) {
    sigma_printf("[SIGMA] %s\n", msg);
}

} // extern "C"
