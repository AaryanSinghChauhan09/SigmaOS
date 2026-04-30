#include "SovereignLibC.h"

#ifndef va_list
#define va_list __builtin_va_list
#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_arg(ap, type) __builtin_va_arg(ap, type)
#define va_end(ap) __builtin_va_end(ap)
#endif

void sigma_log(const char* msg) {
    sigma_print("[SIGMA_LOG]: ");
    sigma_print(msg);
    sigma_print("\n");
}

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
            buf[i--] = (char)((val % 10) + '0');
            val /= 10;
        }
    }
    sigma_print(&buf[i + 1]);
}

void sigma_print_pad_num(sigma_u64 val, int width) {
    char buf[32];
    int i = 30;
    buf[31] = '\0';
    for (int j = 0; j < width; j++) {
        buf[i--] = (char)((val % 10) + '0');
        val /= 10;
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
                case 's': sigma_print(va_arg(args, const char*)); break;
                case 'd': {
                    int v = va_arg(args, int);
                    if (v < 0) { sigma_write(1, "-", 1); v = -v; }
                    sigma_print_num((sigma_u64)v);
                    break;
                }
                case 'u': sigma_print_num((sigma_u64)va_arg(args, unsigned int)); break;
                case 'l':
                    if (*(p+1) == 'l' && *(p+2) == 'u') {
                        sigma_print_num(va_arg(args, sigma_u64));
                        p += 2;
                    } else if (*(p+1) == 'l' && *(p+2) == 'd') {
                        sigma_i64 v = va_arg(args, sigma_i64);
                        if (v < 0) { sigma_write(1, "-", 1); v = -v; }
                        sigma_print_num((sigma_u64)v);
                        p += 2;
                    }
                    break;
                case 'x':
                case 'p': sigma_print_hex((sigma_u64)va_arg(args, sigma_u64)); break;
                case 'c': {
                    char c = (char)va_arg(args, int);
                    sigma_write(1, &c, 1);
                    break;
                }
                case 'f': {
                    sigma_f64 fv = va_arg(args, sigma_f64);
                    if (fv < 0.0) { sigma_write(1, "-", 1); fv = -fv; }
                    sigma_u64 intpart = (sigma_u64)fv;
                    sigma_print_num(intpart);
                    sigma_write(1, ".", 1);
                    sigma_u64 frac = (sigma_u64)((fv - (sigma_f64)intpart) * 10000.0 + 0.5);
                    sigma_print_pad_num(frac, 4);
                    break;
                }
                default: sigma_write(1, p, 1); break;
            }
        } else {
            sigma_write(1, p, 1);
        }
    }
    va_end(args);
}

int sigma_snprintf(char* str, sigma_size_t size, const char* format, ...) {
    if (!str || size == 0) return 0;
    va_list args;
    va_start(args, format);
    sigma_size_t written = 0;
    for (const char* p = format; *p != '\0' && written < size - 1; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            if (*p == 's') {
                const char* s = va_arg(args, const char*);
                while (*s && written < size - 1) str[written++] = *s++;
            } else if (*p == 'd') {
                int v = va_arg(args, int);
                if (v < 0) {
                    if (written < size - 1) str[written++] = '-';
                    v = -v;
                }
                char buf[16];
                int i = 0;
                if (v == 0) buf[i++] = '0';
                else {
                    while (v > 0) { buf[i++] = (char)((v % 10) + '0'); v /= 10; }
                }
                for (int j = i - 1; j >= 0 && written < size - 1; j--) str[written++] = buf[j];
            } else {
                str[written++] = *p;
            }
        } else {
            str[written++] = *p;
        }
    }
    str[written] = '\0';
    va_end(args);
    return (int)written;
}
