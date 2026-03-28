/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GUIDE LINTER (v6.0 - NO-DEP EDITION)
 * =========================================================================
 * Mission: Refactor fix_guide_lints.js into a native C++ utility.
 * Objective: Reduce dependency on Node.js and external JS runtimes.
 * Principle: Zero third-party librariies. Only SigmaLibC.
 * =========================================================================
 */

#include "SigmaLibC.h"

/* 
 * Helper functions for raw syscalls not in basic headers 
 * SYS_open = 2 (x86_64)
 * SYS_close = 3 (x86_64)
 * O_RDWR = 2 (standard)
 */
sigma_i64 sigma_open(const char* filename, sigma_i32 flags, sigma_i32 mode) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_i64 ret;
    __asm__ volatile (
        "syscall"
        : "=a"(ret)
        : "a"(2ULL), "D"(filename), "S"(flags), "d"(mode)
        : "rcx", "r11", "memory"
    );
    return ret;
#else
    return -1;
#endif
}

sigma_i64 sigma_close(sigma_i32 fd) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_i64 ret;
    __asm__ volatile (
        "syscall"
        : "=a"(ret)
        : "a"(3ULL), "D"(fd)
        : "rcx", "r11", "memory"
    );
    return ret;
#else
    return -1;
#endif
}

/* 
 * Linter logic: Processes OS_GUIDE.md and fixes layout issues.
 */
void process_buffer(char* buffer, sigma_usize size, sigma_i32 out_fd) {
    sigma_i32 h1Count = 0;
    sigma_usize i = 0;
    sigma_bool last_was_newline = SIGMA_TRUE;
    sigma_i32 newline_count = 0;

    sigma_printf("[LINTER]: Processing %llu bytes...\n", (unsigned long long)size);

    while (i < size) {
        char c = buffer[i];

        /* Check for multiple blank lines (\n{3,}) -> \n\n */
        if (c == '\n') {
            newline_count++;
            if (newline_count <= 2) {
                sigma_write(out_fd, "\n", 1);
            }
            i++;
            continue;
        } else {
            newline_count = 0;
        }

        /* Check for headings (# ) to ensure spacing and H1 count */
        if (c == '#' && (i == 0 || buffer[i-1] == '\n')) {
            /* We handle heading spacing briefly */
            sigma_usize h_level = 0;
            while (i < size && buffer[i] == '#') { h_level++; i++; }
            
            if (i < size && buffer[i] == ' ') {
                /* Spacing around heading found */
                if (h_level == 1) {
                    h1Count++;
                    if (h1Count > 1) {
                        /* Convert extra H1s to H2 */
                        sigma_write(out_fd, "## ", 3);
                    } else {
                        sigma_write(out_fd, "# ", 2);
                    }
                } else {
                    /* Print proper number of hashes */
                    for (sigma_usize k = 0; k < h_level; k++) sigma_write(out_fd, "#", 1);
                    sigma_write(out_fd, " ", 1);
                }
                i++; /* skip space */
            } else {
                /* If no space after #, restore it like it was or just keep moving */
                for (sigma_usize k = 0; k < h_level; k++) sigma_write(out_fd, "#", 1);
            }
            continue;
        }

        /* Normal char */
        sigma_write(out_fd, &c, 1);
        i++;
    }
}

int main() {
    sigma_printf("[SIGMA_LINTER]: Starting Sovereign Linter v6.0...\n");

    const char* filePath = "OS_GUIDE.md";
    const char* tmpPath = "OS_GUIDE.md.tmp";

    sigma_i32 in_fd = (sigma_i32)sigma_open(filePath, 0, 0); /* O_RDONLY */
    if (in_fd < 0) {
        sigma_printf("[FAIL]: Could not open OS_GUIDE.md for reading.\n");
        return 1;
    }

    /* We assume size for now or could stat, for simplicity just read in chunks */
    /* But for lints, we might need more buffer. Let's allocate 1MB if possible, 
       or just use a stack buffer of 128KB. OS_GUIDE.md is 600KB+ */
    
    /* We'll use a large heap buffer if mmap is available, otherwise just use a fixed 1MB buffer */
    char* data = (char*)SIGMA_NULL;
    /* Simulate memory allocation via syscall 9 (mmap) */
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile (
        "syscall"
        : "=a"(data)
        : "a"(9ULL), "D"(0ULL), "S"(2000000ULL), "d"(3ULL /* PROT_READ|WRITE */), "r"(0x22 /* MAP_PRIVATE|ANON */), "r"(-1ULL), "r"(0ULL)
        : "rcx", "r11", "memory"
    );
#endif

    if (data == (char*)SIGMA_NULL || (sigma_i64)data < 0) {
        sigma_printf("[FAIL]: Buffer allocation failed (mmap).\n");
        sigma_close(in_fd);
        return 1;
    }

    sigma_i64 bytesRead = sigma_read(in_fd, data, 2000000);
    sigma_close(in_fd);

    if (bytesRead <= 0) {
        sigma_printf("[FAIL]: File is empty or read failed.\n");
        return 1;
    }

    sigma_i32 out_fd = (sigma_i32)sigma_open(tmpPath, 65 /* O_WRONLY|O_CREAT|O_TRUNC */, 0644);
    if (out_fd < 0) {
        sigma_printf("[FAIL]: Could not open tmp file for writing.\n");
        return 1;
    }

    process_buffer(data, (sigma_usize)bytesRead, out_fd);
    sigma_close(out_fd);

    sigma_printf("[SUCCESS]: Bulk lints fixed in OS_GUIDE.md.tmp\n");
    sigma_printf("[SUCCESS]: Sovereignty level increased. Node.js dependency REDUCED.\n");

    return 0;
}

