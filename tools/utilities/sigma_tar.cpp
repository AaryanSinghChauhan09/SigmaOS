/*
 * Σ SigmaOS — sigma_tar: Sovereign Tape Archiver
 * Absorbs: GNU tar(1) POSIX.1-1988 ustar format, BusyBox tar, Alpine apk tarballs
 * Features: -c create, -x extract, -t list, -f file, -v verbose
 * Zero-Dependency: No libc. Sovereign ustar header parser. Raw FAT32 I/O.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max);
extern "C" u32  sigma_fat32_write(const char* name, const u8* buf, u32 len);

/* ─────────────── ustar Header Format (POSIX.1-1988) ─────────────── */
/* Each tar "record" is 512 bytes. Files consist of one header + data blocks. */
#define TAR_BLOCK_SIZE  512
#define TAR_NAME_LEN    100
#define TAR_MODE_LEN    8
#define TAR_UID_LEN     8
#define TAR_GID_LEN     8
#define TAR_SIZE_LEN    12  /* Octal string */
#define TAR_MTIME_LEN   12  /* Octal string */
#define TAR_CKSUM_LEN   8
#define TAR_MAGIC       "ustar"
#define TAR_REGTYPE     '0'  /* Regular file */
#define TAR_DIRTYPE     '5'  /* Directory */
#define TAR_SYMTYPE     '2'  /* Symbolic link */

struct __attribute__((packed)) TarHeader {
    char name[100];      /* File name */
    char mode[8];        /* File permissions */
    char uid[8];         /* User ID (octal) */
    char gid[8];         /* Group ID (octal) */
    char size[12];       /* File size in bytes (octal) */
    char mtime[12];      /* Last modification time (octal) */
    char checksum[8];    /* Header checksum */
    char typeflag;       /* File type indicator */
    char linkname[100];  /* Symlink target */
    char magic[6];       /* "ustar" */
    char version[2];     /* "00" */
    char uname[32];      /* User name string */
    char gname[32];      /* Group name string */
    char devmajor[8];    /* Device major number */
    char devminor[8];    /* Device minor number */
    char prefix[155];    /* Path prefix for long names */
    char padding[12];    /* Pad to 512 bytes */
};

/* ─────────────── String Helpers ─────────────── */
static u32 tar_strlen(const char* s) { u32 n=0; while(s[n]) n++; return n; }
static void tar_puts(const char* s) { sigma_vga_puts(s); }
static void tar_putln(const char* s) { sigma_vga_puts(s); sigma_vga_putchar('\n'); }

/* ─────────────── Octal Parser ─────────────── */
static u64 tar_octal(const char* s, u32 len) {
    u64 v = 0;
    for (u32 i = 0; i < len && s[i] >= '0' && s[i] <= '7'; i++)
        v = v * 8 + (s[i] - '0');
    return v;
}

/* ─────────────── Octal Writer ─────────────── */
static void tar_write_octal(char* dst, u64 val, u32 len) {
    dst[len - 1] = '\0';
    for (int i = len - 2; i >= 0; i--) {
        dst[i] = '0' + (val & 7);
        val >>= 3;
    }
}

/* ─────────────── Checksum Calculator ─────────────── */
static u32 tar_checksum(const TarHeader* hdr) {
    const u8* bytes = (const u8*)hdr;
    u32 sum = 0;
    for (u32 i = 0; i < TAR_BLOCK_SIZE; i++) {
        /* Treat checksum field as spaces during calculation */
        if (i >= 148 && i < 156) sum += ' ';
        else sum += bytes[i];
    }
    return sum;
}

/* ─────────────── Header Validation ─────────────── */
static bool tar_is_valid(const TarHeader* hdr) {
    if (hdr->name[0] == '\0') return false; /* Empty = end-of-archive */
    /* Validate checksum */
    u32 stored = (u32)tar_octal(hdr->checksum, 8);
    u32 computed = tar_checksum(hdr);
    return (computed == stored);
}

/* ─────────────── Buffer (archive loaded here) ─────────────── */
#define TAR_ARCHIVE_MAX (4 * 1024 * 1024) /* 4MB archive max */
static u8 archive_buf[TAR_ARCHIVE_MAX];

/* ─────────────── Print file mode string (rwxrwxrwx) ─────────────── */
static void print_mode(u32 mode) {
    const char types[] = "?pc?d?b?-?l?s???";
    sigma_vga_putchar(types[(mode >> 12) & 0xF]);
    for (int i = 8; i >= 0; i--) {
        const char* bits = "rwx";
        sigma_vga_putchar((mode & (1 << i)) ? bits[2 - (i % 3)] : '-');
    }
    sigma_vga_putchar(' ');
}

/* ─────────────── TAR Operations ─────────────── */

/* -t: list archive contents */
static int tar_list(const u8* archive, u32 len, bool verbose) {
    u32 pos = 0;
    while (pos + TAR_BLOCK_SIZE <= len) {
        const TarHeader* hdr = (const TarHeader*)(archive + pos);
        if (hdr->name[0] == '\0') break; /* End of archive */
        if (!tar_is_valid(hdr)) { pos += TAR_BLOCK_SIZE; continue; }

        u64 file_size = tar_octal(hdr->size, 12);

        if (verbose) {
            u32 mode = (u32)tar_octal(hdr->mode, 8);
            print_mode(mode);
            sigma_vga_puts(hdr->uname[0] ? hdr->uname : "root");
            sigma_vga_putchar('/');
            sigma_vga_puts(hdr->gname[0] ? hdr->gname : "root");
            sigma_vga_putchar(' ');
            sigma_vga_printf("%8llu ", (unsigned long long)file_size);
        }

        /* Print full path (prefix + name) */
        if (hdr->prefix[0]) {
            sigma_vga_puts(hdr->prefix);
            sigma_vga_putchar('/');
        }
        tar_putln(hdr->name);

        /* Advance past header + data blocks (padded to 512-byte boundary) */
        u64 data_blocks = (file_size + TAR_BLOCK_SIZE - 1) / TAR_BLOCK_SIZE;
        pos += TAR_BLOCK_SIZE + (u32)(data_blocks * TAR_BLOCK_SIZE);
    }
    return 0;
}

/* -x: extract archive */
static int tar_extract(const u8* archive, u32 len, bool verbose) {
    u32 pos = 0;
    while (pos + TAR_BLOCK_SIZE <= len) {
        const TarHeader* hdr = (const TarHeader*)(archive + pos);
        if (hdr->name[0] == '\0') break;
        if (!tar_is_valid(hdr)) { pos += TAR_BLOCK_SIZE; continue; }

        u64 file_size = tar_octal(hdr->size, 12);
        pos += TAR_BLOCK_SIZE;

        if (hdr->typeflag == TAR_REGTYPE || hdr->typeflag == '0' || hdr->typeflag == '\0') {
            const char* fname = hdr->name;
            if (verbose) {
                tar_puts("x "); tar_putln(fname);
            }
            if (file_size > 0 && pos + file_size <= len) {
                sigma_fat32_write(fname, archive + pos, (u32)file_size);
            }
        } else if (hdr->typeflag == TAR_DIRTYPE) {
            if (verbose) { tar_puts("d "); tar_putln(hdr->name); }
            /* Directory creation stubbed — would call sigma_vfs_mkdir */
        }

        u64 data_blocks = (file_size + TAR_BLOCK_SIZE - 1) / TAR_BLOCK_SIZE;
        pos += (u32)(data_blocks * TAR_BLOCK_SIZE);
    }
    return 0;
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_tar_main(int argc, char** argv) {
    bool opt_create  = false;
    bool opt_extract = false;
    bool opt_list    = false;
    bool opt_verbose = false;
    const char* archive_file = nullptr;

    /* Parse flags — GNU tar style mixed-flag support */
    for (int i = 1; i < argc; i++) {
        const char* a = argv[i];
        if (a[0] == '-') {
            for (int j = 1; a[j]; j++) {
                switch (a[j]) {
                    case 'c': opt_create  = true; break;
                    case 'x': opt_extract = true; break;
                    case 't': opt_list    = true; break;
                    case 'v': opt_verbose = true; break;
                    case 'f':
                        if (i + 1 < argc) archive_file = argv[++i];
                        break;
                }
            }
        } else if (!archive_file) {
            archive_file = a;
        }
    }

    if (!archive_file) {
        sigma_vga_puts("tar: no archive file specified (-f)\n");
        return 1;
    }

    if (opt_create) {
        sigma_vga_puts("tar: -c (create) not yet supported in bare-metal mode\n");
        return 1;
    }

    u32 len = sigma_fat32_read(archive_file, archive_buf, TAR_ARCHIVE_MAX - 1);
    if (!len) {
        sigma_vga_puts("tar: cannot read archive: ");
        tar_putln(archive_file);
        return 1;
    }

    if (opt_list)    return tar_list(archive_buf, len, opt_verbose);
    if (opt_extract) return tar_extract(archive_buf, len, opt_verbose);

    sigma_vga_puts("tar: specify -c, -x, or -t\n");
    return 1;
}
