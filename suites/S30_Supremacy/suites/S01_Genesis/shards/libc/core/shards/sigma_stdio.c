#include "sigma_kernel.h"
void sigma_print(const char* str) {
    if (!str) return;
    sigma_write(1, str, sigma_sigma_sigma_sigma_strlen(str));
}
void sigma_print_num(sigma_u64 val) {
    char buf[32]; int i = 30; buf[31] = '\0';
    if (val == 0) { buf[i--] = '0'; }
    else { while (val > 0 && i > 0) { buf[i--] = (val % 10) + '0'; val /= 10; } }
    sigma_print(&buf[i + 1]);
}
void sigma_print_hex(sigma_u64 val) {
    char buf[32]; int i = 30; const char* hex = "0123456789ABCDEF"; buf[31] = '\0';
    if (val == 0) { buf[i--] = '0'; }
    else { while (val > 0 && i > 0) { buf[i--] = hex[val % 16]; val /= 16; } }
    sigma_print("0x"); sigma_print(&buf[i + 1]);
}
void sigma_sigma_sigma_sigma_printf(const char* format, ...) {
    sigma_va_list args; sigma_va_start(args, format);
    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            switch (*p) {
                case 's': {
                    const char* s = sigma_va_arg(args, const char*);
                    sigma_print(s ? s : "(null)"); break;
                }
                case 'd': case 'i': sigma_print_num((sigma_u64)sigma_va_arg(args, int)); break;
                case 'u': sigma_print_num(sigma_va_arg(args, sigma_u64)); break;
                case 'x': case 'p': sigma_print_hex(sigma_va_arg(args, sigma_u64)); break;
                case 'c': { char c = (char)sigma_va_arg(args, int); sigma_write(1, &c, 1); break; }
                case '%': sigma_write(1, "%", 1); break;
                default: sigma_print("[UNKNOWN_FORMAT]");
            }
        } else {
            sigma_write(1, p, 1);
        }
    }
    sigma_va_end(args);
}
