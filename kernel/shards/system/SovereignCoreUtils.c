/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN CORE UTILS (v20.0 - PURE C11)
 * =========================================================================
 * Converted from C++ class/namespace to ISO C11.
 * Utility: Bare-metal string ops, hex dump, process info, timestamps.
 * Standard: C11 (ISO/IEC 9899:2011) â€ zero external symbols.
 * =========================================================================
 */

#include "SovereignLibC.h"

/* =========================================================================
 * Sovereign Hex Dump (replaces C++ class utility)
 * ========================================================================= */
void sigma_hexdump(const void* ptr, sigma_size_t len) {
    const sigma_u8* p = (const sigma_u8*)ptr;
    sigma_size_t i;
    sigma_printf("[HEXDUMP]: %llu bytes @ addr ", len);
    sigma_print_hex((sigma_u64)(sigma_size_t)ptr);
    sigma_print("\n");
    for (i = 0; i < len; i++) {
        if (i % 16 == 0) {
            sigma_print("  ");
            sigma_print_hex(i);
            sigma_print(": ");
        }
        sigma_print_hex(p[i] & 0xFF);
        sigma_print(" ");
        if ((i + 1) % 16 == 0) sigma_print("\n");
    }
    sigma_print("\n");
}

/* =========================================================================
 * Sovereign String Copy (pure C11 â€ no strncpy from libc)
 * ========================================================================= */
void sigma_strcpy(char* dest, const char* src, sigma_size_t maxlen) {
    sigma_size_t i = 0;
    while (i < maxlen - 1 && src[i] != '\0') {
        dest[i] = src[i];
        i++;
    }
    dest[i] = '\0';
}

/* =========================================================================
 * Sovereign Number-to-String (decimal â€ no sprintf)
 * ========================================================================= */
void sigma_u64_to_str(sigma_u64 val, char* buf, sigma_size_t buflen) {
    int i = (int)(buflen - 2);
    buf[buflen - 1] = '\0';
    if (val == 0) { buf[i--] = '0'; }
    else {
        while (val > 0 && i >= 0) {
            buf[i--] = (char)((val % 10) + '0');
            val /= 10;
        }
    }
    /* Shift to the front */
    sigma_size_t start = (sigma_size_t)(i + 1);
    sigma_size_t j = 0;
    while (buf[start] != '\0') { buf[j++] = buf[start++]; }
    buf[j] = '\0';
}

/* =========================================================================
 * Sovereign System Info Banner (replaces C++ class SovereignCoreUtils)
 * ========================================================================= */
typedef struct SovereignCoreUtils {
    const char* build_version;
    sigma_u64   uptime_ticks;
    sigma_u64   heap_used;
} SovereignCoreUtils;

static void coreutils_init(SovereignCoreUtils* cu) {
    cu->build_version = "v20.0-C11-SOVEREIGN";
    cu->uptime_ticks  = 0;
    cu->heap_used     = 0;
}

static void coreutils_tick(SovereignCoreUtils* cu) {
    /* Read hardware TSC for uptime tracking */
    sigma_u64 tsc;
    __asm__ __volatile__ (
        "rdtsc\n\t"
        "shl $32, %%rdx\n\t"
        "or  %%rdx, %%rax"
        : "=a"(tsc) :: "rdx");
    cu->uptime_ticks = tsc;
}

static void coreutils_banner(const SovereignCoreUtils* cu) {
    sigma_printf("\n");
    sigma_printf("Î£ ======================================================= Î£\n");
    sigma_printf("  SigmaOS Sovereign Core Utils %s\n", cu->build_version);
    sigma_printf("  TSC Uptime Ticks : %llu\n", cu->uptime_ticks);
    sigma_printf("  Language Stack   : C11 (98%) + ASM (0.7%) + Rust (0.3%)\n");
    sigma_printf("  libc             : SovereignLibC (zero glibc dependency)\n");
    sigma_printf("Î£ ======================================================= Î£\n");
}

/* =========================================================================
 * Entry Point
 * ========================================================================= */
int main(void) {
    SovereignCoreUtils cu;
    coreutils_init(&cu);
    coreutils_tick(&cu);
    coreutils_banner(&cu);

    /* Demo: hex dump a small buffer */
    sigma_u8 test_buf[16];
    sigma_u8 i;
    for (i = 0; i < 16; i++) test_buf[i] = i;
    sigma_hexdump(test_buf, 16);

    return 0;
}
