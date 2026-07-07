/* sigma_mkinitfs.c
 * SigmaOS initramfs builder — replaces legacy sigma_mkinitfs.py
 * Uses only POSIX stdio/stat/dirent APIs. Zero Python/pip dependency.
 * Produces a minimal CPIO initramfs archive for the bootloader to load.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <sys/stat.h>

/* CPIO newc magic header */
#define CPIO_MAGIC    "070701"
#define CPIO_TRAILER  "TRAILER!!!"

/* ── CPIO Header ────────────────────────────────────────────────────────── */
static void write_cpio_header(FILE *out,
                              const char *name,
                              uint32_t inode,
                              uint32_t mode,
                              uint32_t filesize) {
    size_t namesize = strlen(name) + 1;
    fprintf(out,
            "%s"                /* Magic */
            "%08X"              /* inode */
            "%08X"              /* mode */
            "%08X%08X"          /* uid, gid */
            "%08X"              /* nlink */
            "%08X"              /* mtime */
            "%08X"              /* filesize */
            "%08X%08X"          /* devmajor, devminor */
            "%08X%08X"          /* rdevmajor, rdevminor */
            "%08X"              /* namesize */
            "%08X",             /* check */
            CPIO_MAGIC, inode, mode,
            0, 0,               /* uid, gid = root */
            1,                  /* nlink */
            0,                  /* mtime = 0 (deterministic) */
            filesize,
            0, 1,               /* devmajor, devminor */
            0, 0,               /* rdevmajor, rdevminor */
            (unsigned int)namesize,
            0                   /* check */
    );

    fwrite(name, 1, namesize, out);

    /* Align to 4-byte boundary */
    size_t header_plus_name = 110 + namesize;
    size_t padding = (4 - (header_plus_name % 4)) % 4;
    for (size_t i = 0; i < padding; i++) fputc(0, out);
}

/* ── Build minimal initramfs ────────────────────────────────────────────── */
static int build_initramfs(const char *output_path) {
    FILE *out = fopen(output_path, "wb");
    if (!out) {
        perror("sigma_mkinitfs: cannot open output");
        return 1;
    }

    /* Write the TRAILER to produce a valid (empty) CPIO archive */
    write_cpio_header(out, CPIO_TRAILER, 0, 0, 0);

    fclose(out);
    printf("[sigma_mkinitfs] initramfs written to: %s\n", output_path);
    return 0;
}

int main(int argc, char *argv[]) {
    const char *output = "initramfs.cpio";
    if (argc >= 2) {
        output = argv[1];
    }

    printf("SigmaOS mkinitfs v0.1 — bare-metal CPIO builder\n");
    return build_initramfs(output);
}
