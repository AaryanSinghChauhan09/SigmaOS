/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S02_ZenithUI/shards/sigma_boot.h
 * =========================================================================
 * Sovereign Boot Subsystem — gap-closes:
 *   Linux  : GRUB2, systemd-boot, EFISTUB, kexec, initramfs
 *   Windows: Windows Boot Manager, BCD store, Secure Boot shims
 *   macOS  : Apple Silicon boot (iBoot), OpenFirmware, efibootmgr
 *   UEFI   : GUIDs, EFI System Table, LoadImage/StartImage protocols
 *   RISC-V : OpenSBI M-mode firmware, BBL bootloader
 * =========================================================================
 */

#ifndef SIGMA_BOOT_H
#define SIGMA_BOOT_H

typedef unsigned long long boot_u64;
typedef unsigned int       boot_u32;
typedef unsigned short     boot_u16;
typedef unsigned char      boot_u8;
typedef signed   int       boot_i32;
typedef unsigned char      boot_bool;
#define BOOT_TRUE  ((boot_bool)1)
#define BOOT_FALSE ((boot_bool)0)
#define BOOT_OK    ((boot_i32) 0)
#define BOOT_ERR   ((boot_i32)-1)

/* ── Boot phases ─────────────────────────────────────────────────────────── */
typedef enum {
    BOOT_PHASE_FIRMWARE   = 0,  /* UEFI/BIOS/iBoot initialization        */
    BOOT_PHASE_BOOTLOADER = 1,  /* GRUB/systemd-boot/SigmaBoot           */
    BOOT_PHASE_INITRAMFS  = 2,  /* early userspace, device enumeration   */
    BOOT_PHASE_KERNEL     = 3,  /* kernel subsystem init                 */
    BOOT_PHASE_USERSPACE  = 4,  /* PID-1 / sigma-init                   */
    BOOT_PHASE_COMPLETE   = 5   /* system fully operational              */
} sigma_boot_phase_t;

/* ── Memory map entry (UEFI EFI_MEMORY_DESCRIPTOR parity) ───────────────── */
typedef enum {
    MEM_CONVENTIONAL = 0,
    MEM_RESERVED     = 1,
    MEM_ACPI_RECLAIM = 2,
    MEM_ACPI_NVS     = 3,
    MEM_MMIO         = 4,
    MEM_BOOT_SRVCS   = 5,    /* reclaimable after ExitBootServices()   */
    MEM_SIGMA_KERN   = 6     /* SigmaOS kernel image                   */
} sigma_mem_type_t;

typedef struct {
    sigma_mem_type_t type;
    boot_u64         phys_start;
    boot_u64         virt_start;
    boot_u64         num_pages;   /* 4KB pages                          */
    boot_u64         attr;        /* UEFI EFI_MEMORY attributes         */
} sigma_mem_desc_t;

#define SIGMA_MEMMAP_MAX 256

/* ── Boot configuration ──────────────────────────────────────────────────── */
#define BOOT_CMDLINE_LEN 512
#define BOOT_INITRD_PATH 256

typedef struct {
    char     cmdline[BOOT_CMDLINE_LEN];  /* kernel command line         */
    char     initrd_path[BOOT_INITRD_PATH];
    boot_u64 kernel_base;                /* physical load address       */
    boot_u64 kernel_size;
    boot_u64 initrd_base;
    boot_u64 initrd_size;
    boot_u32 cpu_count;
    boot_u32 ram_mb;
    boot_bool secure_boot;               /* UEFI Secure Boot active     */
    boot_bool kaslr;                     /* Kernel ASLR enabled         */
    boot_bool pqc_attest;               /* TPM+ML-DSA boot attestation */
} sigma_boot_config_t;

/* ── EFI-style GUID ──────────────────────────────────────────────────────── */
typedef struct {
    boot_u32 data1;
    boot_u16 data2, data3;
    boot_u8  data4[8];
} sigma_guid_t;

/* ── Boot entry (BCD/EFI BootXXXX variable parity) ─────────────────────── */
typedef struct {
    char        description[64];
    boot_u64    load_addr;
    sigma_guid_t disk_guid;
    boot_bool   is_active;
} sigma_boot_entry_t;

#define SIGMA_BOOT_ENTRIES_MAX 8

/* ── Public API ─────────────────────────────────────────────────────────── */
void sigma_boot_init(sigma_boot_config_t *cfg);
void sigma_boot_phase_advance(sigma_boot_phase_t to);

/* Memory map */
boot_i32 sigma_boot_add_mem_region(sigma_mem_type_t type,
                                    boot_u64 phys, boot_u64 pages);
void     sigma_boot_print_memmap(void);
boot_u64 sigma_boot_total_ram_kb(void);

/* Boot entries (GRUB menu / BCD parity) */
boot_i32 sigma_boot_entry_add(const char *desc, boot_u64 load_addr);
void     sigma_boot_entry_list(void);
void     sigma_boot_entry_select(boot_u32 idx);

/* Secure boot + attestation */
boot_bool sigma_secure_boot_verify(const boot_u8 *img, boot_u64 img_len,
                                    const boot_u8 *sig, boot_u64 sig_len);
void      sigma_boot_attest_tpm(void);

/* kexec: jump to new kernel without reboot */
boot_i32 sigma_kexec_load(boot_u64 new_kernel_phys, boot_u64 new_initrd_phys);
void     sigma_kexec_execute(void);

void sigma_boot_report(void);

#endif /* SIGMA_BOOT_H */
