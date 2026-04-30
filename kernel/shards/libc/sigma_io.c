/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN LIBC - I/O SHARD (v20.0)
 * =========================================================================
 */

#include "SovereignLibC.h"
#include <stdarg.h>

void sigma_print(const char* str) {
    if (!str) return;
    sigma_write(1, str, sigma_strlen(str));
}

void sigma_print_num(sigma_u64 val) {
    char buf[32];
    int i = 30;
    buf[31] = '\0';
    if (val == 0) {
        buf[i--] = '0';
    } else {
        while (val > 0 && i > 0) {
            buf[i--] = (val % 10) + '0';
            val /= 10;
        }
    }
    sigma_print(&buf[i + 1]);
}

void sigma_print_hex(sigma_u64 val) {
    char buf[32];
    int i = 30;
    const char* hex = "0123456789ABCDEF";
    buf[31] = '\0';
    if (val == 0) {
        buf[i--] = '0';
    } else {
        while (val > 0 && i > 0) {
            buf[i--] = hex[val % 16];
            val /= 16;
        }
    }
    sigma_print("0x");
    sigma_print(&buf[i + 1]);
}

void sigma_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);
    
    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            switch (*p) {
                case 's':
                    sigma_print(va_arg(args, const char*));
                    break;
                case 'd':
                case 'u':
                    sigma_print_num(va_arg(args, sigma_u64));
                    break;
                case 'x':
                case 'p':
                    sigma_print_hex(va_arg(args, sigma_u64));
                    break;
                case 'c': {
                    char c = (char)va_arg(args, int);
                    sigma_write(1, &c, 1);
                    break;
                }
                default:
                    sigma_write(1, p, 1);
            }
        } else {
            sigma_write(1, p, 1);
        }
    }
    va_end(args);
}

void sigma_log(const char* msg) {
    sigma_printf("[ZENITH-LOG]: %s\n", msg);
}
