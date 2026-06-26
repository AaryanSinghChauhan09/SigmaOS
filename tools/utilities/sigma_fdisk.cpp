/*
 * Σ SigmaOS — sigma_fdisk: Sovereign Disk Partitioner
 * Absorbs: util-linux fdisk(8), parted, Arch Linux cfdisk, Alpine apk disk tools
 * Features: list partitions (MBR+GPT), display geometry, basic sector info
 * Zero-Dependency: No libc. Raw ATA/SATA port I/O + sector reads.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);
/* sigma_ahci_read: reads 'count' sectors from 'lba' into 'buf' */
extern "C" int sigma_ahci_read(u32 port, u64 lba, u32 count, u8* buf);

/* ─────────────── String Helpers ─────────────── */
static void fd_puts(const char* s) { sigma_vga_puts(s); }
static void fd_putln(const char* s) { sigma_vga_puts(s); sigma_vga_putchar('\n'); }

static void fd_print_u64(u64 v) {
    if (v == 0) { sigma_vga_putchar('0'); return; }
    char buf[21]; int i = 0;
    while (v) { buf[i++] = '0' + (v % 10); v /= 10; }
    while (i > 0) sigma_vga_putchar(buf[--i]);
}

static void fd_print_size_mb(u64 sectors) {
    /* 1 sector = 512 bytes; 1 MB = 2048 sectors */
    u64 mb = sectors / 2048;
    fd_print_u64(mb);
    fd_puts(" MiB");
}

/* ─────────────── MBR Structures ─────────────── */
/* BIOS MBR partition table entry (16 bytes each, 4 entries at offset 0x1BE) */
struct __attribute__((packed)) MBRPartEntry {
    u8  status;        /* 0x80 = bootable, 0x00 = not */
    u8  chs_first[3];  /* CHS of first sector */
    u8  type;          /* Partition type */
    u8  chs_last[3];   /* CHS of last sector */
    u32 lba_start;     /* LBA start sector */
    u32 sector_count;  /* Number of sectors */
};

struct __attribute__((packed)) MBR {
    u8           bootstrap[446];  /* Boot code */
    MBRPartEntry partitions[4];   /* 4 primary entries */
    u16          signature;       /* 0xAA55 */
};

/* ─────────────── GPT Structures ─────────────── */
/* GUID Partition Table — from UEFI Spec 2.10 §5.3 */
#define GPT_HEADER_SIGNATURE 0x5452415020494645ULL  /* "EFI PART" */
#define GPT_HEADER_LBA       1
#define GPT_MAX_PARTITIONS   128

struct __attribute__((packed)) GPTHeader {
    u64 signature;        /* "EFI PART" */
    u32 revision;         /* 0x00010000 for v1.0 */
    u32 header_size;      /* Usually 92 bytes */
    u32 header_crc32;
    u32 reserved;
    u64 my_lba;           /* LBA of this header */
    u64 alternate_lba;    /* LBA of backup header */
    u64 first_usable_lba;
    u64 last_usable_lba;
    u8  disk_guid[16];
    u64 partition_entry_lba;
    u32 num_partitions;
    u32 partition_entry_size;
    u32 partition_array_crc32;
};

struct __attribute__((packed)) GPTPartEntry {
    u8  type_guid[16];    /* Partition type GUID */
    u8  unique_guid[16];  /* Unique partition GUID */
    u64 start_lba;
    u64 end_lba;
    u64 attributes;
    u16 name[36];         /* UTF-16LE partition name */
};

/* ─────────────── Partition Type Names ─────────────── */
struct PartTypeName {
    u8  id;
    const char* name;
};

static const PartTypeName mbr_types[] = {
    { 0x00, "Empty"               },
    { 0x05, "Extended"            },
    { 0x07, "NTFS/HPFS"           },
    { 0x0B, "FAT32"               },
    { 0x0C, "FAT32 LBA"           },
    { 0x0E, "FAT16 LBA"           },
    { 0x82, "Linux swap"          },
    { 0x83, "Linux ext2/3/4"      },
    { 0x84, "OS/2 hidden"         },
    { 0x85, "Linux extended"      },
    { 0x8E, "Linux LVM"           },
    { 0xA5, "FreeBSD"             },
    { 0xAF, "HFS+"                },
    { 0xEE, "GPT Protective MBR"  },
    { 0xEF, "EFI System"          },
    { 0xFB, "VMware VMFS"         },
    { 0x00, nullptr               }
};

static const char* mbr_type_name(u8 id) {
    for (u32 i = 0; mbr_types[i].name != nullptr; i++)
        if (mbr_types[i].id == id) return mbr_types[i].name;
    return "Unknown";
}

/* ─────────────── Buffers ─────────────── */
static u8 sector_buf[4 * 512]; /* Read up to 4 sectors at once */

/* ─────────────── MBR Listing ─────────────── */
static void fdisk_list_mbr(u32 port, const MBR* mbr) {
    fd_puts("\nDisk label type: MBR (DOS)\n");
    fd_puts("Device     Boot    Start       End  Sectors  Size Type\n");

    for (u32 i = 0; i < 4; i++) {
        const MBRPartEntry* p = &mbr->partitions[i];
        if (p->type == 0x00 && p->lba_start == 0) continue; /* Empty */

        /* Device name */
        fd_puts("/dev/sda"); sigma_vga_putchar('1' + i);

        /* Bootable flag */
        sigma_vga_puts((p->status == 0x80) ? " *" : "  ");
        sigma_vga_puts("  ");

        /* Start / End / Sectors */
        fd_print_u64(p->lba_start);
        sigma_vga_putchar(' ');
        fd_print_u64(p->lba_start + p->sector_count - 1);
        sigma_vga_putchar(' ');
        fd_print_u64(p->sector_count);
        sigma_vga_putchar(' ');
        fd_print_size_mb(p->sector_count);
        sigma_vga_putchar(' ');

        /* Type */
        fd_putln(mbr_type_name(p->type));
    }
}

/* ─────────────── GPT Listing ─────────────── */
static u8 gpt_entries_buf[128 * 128]; /* 128 entries × 128 bytes max each */

static void print_guid(const u8* guid) {
    /* Print GUID in 8-4-4-4-12 format */
    static const char hex[] = "0123456789ABCDEF";
    /* GPT GUIDs are mixed-endian: first 3 groups are little-endian */
    int order[] = {3,2,1,0, -1, 5,4, -1, 7,6, -1, 8,9, -1, 10,11,12,13,14,15};
    for (u32 i = 0; i < 20; i++) {
        if (order[i] == -1) { sigma_vga_putchar('-'); continue; }
        sigma_vga_putchar(hex[guid[order[i]] >> 4]);
        sigma_vga_putchar(hex[guid[order[i]] & 0xF]);
    }
}

static void print_utf16_name(const u16* name, u32 max) {
    for (u32 i = 0; i < max && name[i]; i++) {
        /* ASCII-range only for terminal output */
        char c = (name[i] < 128) ? (char)name[i] : '?';
        sigma_vga_putchar(c);
    }
}

static void fdisk_list_gpt(u32 port, const GPTHeader* gpt) {
    fd_puts("\nDisk label type: GPT\n");
    sigma_vga_printf("First usable LBA: %llu\n", (unsigned long long)gpt->first_usable_lba);
    sigma_vga_printf("Last  usable LBA: %llu\n", (unsigned long long)gpt->last_usable_lba);
    sigma_vga_printf("Partitions:       %u\n",   gpt->num_partitions);

    /* Load partition entries */
    u32 entry_sectors = (gpt->num_partitions * gpt->partition_entry_size + 511) / 512;
    if (entry_sectors > sizeof(gpt_entries_buf) / 512) entry_sectors = sizeof(gpt_entries_buf) / 512;
    sigma_ahci_read(port, gpt->partition_entry_lba, entry_sectors, gpt_entries_buf);

    fd_puts("\nDevice  Start         End    Sectors  Size Type\n");

    for (u32 i = 0; i < gpt->num_partitions; i++) {
        const GPTPartEntry* e = (const GPTPartEntry*)(gpt_entries_buf +
                                i * gpt->partition_entry_size);

        /* Check if partition is used (type GUID not all-zero) */
        bool used = false;
        for (u32 j = 0; j < 16; j++) if (e->type_guid[j]) { used = true; break; }
        if (!used) continue;

        fd_puts("/dev/sda"); fd_print_u64(i + 1);
        sigma_vga_putchar(' ');
        fd_print_u64(e->start_lba); sigma_vga_putchar(' ');
        fd_print_u64(e->end_lba);   sigma_vga_putchar(' ');
        fd_print_u64(e->end_lba - e->start_lba + 1); sigma_vga_putchar(' ');
        fd_print_size_mb(e->end_lba - e->start_lba + 1); sigma_vga_putchar(' ');
        print_utf16_name(e->name, 36);
        sigma_vga_putchar('\n');
    }
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_fdisk_main(int argc, char** argv) {
    bool opt_list = false;
    u32  port = 0; /* AHCI port 0 = first drive */

    for (int i = 1; i < argc; i++) {
        if (argv[i][0] == '-') {
            for (int j = 1; argv[i][j]; j++) {
                if (argv[i][j] == 'l') opt_list = true;
            }
        }
    }

    /* Read LBA 0 (MBR sector) */
    if (sigma_ahci_read(port, 0, 1, sector_buf) != 0) {
        fd_putln("fdisk: cannot read disk — no AHCI device?");
        return 1;
    }

    MBR* mbr = (MBR*)sector_buf;
    if (mbr->signature != 0xAA55) {
        fd_putln("fdisk: no valid MBR found");
        return 1;
    }

    /* Print disk header */
    fd_puts("Disk /dev/sda\n");
    fd_puts("Sector size (logical/physical): 512 bytes / 512 bytes\n");

    /* Check if protective MBR (GPT disk) */
    bool is_gpt = (mbr->partitions[0].type == 0xEE);
    if (is_gpt) {
        /* Read GPT header at LBA 1 */
        sigma_ahci_read(port, 1, 1, sector_buf);
        GPTHeader* gpt = (GPTHeader*)sector_buf;
        if (gpt->signature == GPT_HEADER_SIGNATURE) {
            fdisk_list_gpt(port, gpt);
        } else {
            fd_putln("fdisk: GPT signature mismatch");
            return 1;
        }
    } else {
        fdisk_list_mbr(port, mbr);
    }

    return 0;
}
