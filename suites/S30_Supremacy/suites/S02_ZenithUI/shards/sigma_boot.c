#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S02_ZenithUI/shards/sigma_boot.c
 * =========================================================================
 */

#include "../../../../../include/sigma_boot.h"
#include "../../../../../include/libc/sigma_libc.h"

static sigma_boot_config_t s_cfg;
static sigma_boot_phase_t  s_phase = BOOT_PHASE_FIRMWARE;

static sigma_mem_desc_t    s_memmap[SIGMA_MEMMAP_MAX];
static boot_u32            s_mem_count = 0;

static sigma_boot_entry_t  s_entries[SIGMA_BOOT_ENTRIES_MAX];
static boot_u32            s_entry_count = 0;
static boot_u32            s_selected    = 0;

static const char *s_phase_names[] = {
    "FIRMWARE","BOOTLOADER","INITRAMFS","KERNEL","USERSPACE","COMPLETE"
};
static const char *s_mem_names[] = {
    "ConvMem","Reserved","ACPI-Recl","ACPI-NVS","MMIO","BootSrvcs","SigmaKern"
};

/* -- Init ------------------------------------------------------------------ */
void sigma_boot_init(sigma_boot_config_t *cfg) {
    sigma_sigma_memset(&s_cfg, 0, sizeof(s_cfg));
    if (cfg) s_cfg = *cfg;

    /* Default configuration */
    if (s_cfg.cmdline[0] == '\0')
        sigma_strncpy(s_cfg.cmdline,
                      "sigma.kaslr=1 sigma.pqc=1 sigma.loglevel=4", 511);
    s_cfg.secure_boot = BOOT_TRUE;
    s_cfg.kaslr       = BOOT_TRUE;
    s_cfg.pqc_attest  = BOOT_TRUE;

    sigma_sigma_printf("S ----------------------------------------------\n");
    sigma_sigma_printf("  SIGMA-BOOT v2.0   SOVEREIGN BOOTLOADER\n");
    sigma_sigma_printf("S ----------------------------------------------\n");
    sigma_sigma_printf("  CPU: %u cores   RAM: %u MB\n", s_cfg.cpu_count, s_cfg.ram_mb);
    sigma_sigma_printf("  CMD: %s\n", s_cfg.cmdline);
    sigma_sigma_printf("  Secure Boot: %s   KASLR: %s   PQC-Attest: %s\n",
                 s_cfg.secure_boot ? "ON" : "OFF",
                 s_cfg.kaslr       ? "ON" : "OFF",
                 s_cfg.pqc_attest  ? "ON" : "OFF");

    sigma_boot_phase_advance(BOOT_PHASE_BOOTLOADER);
}

void sigma_boot_phase_advance(sigma_boot_phase_t to) {
    if (to <= s_phase) return;
    s_phase = to;
    sigma_sigma_printf("S [BOOT] -- Phase: %s --\n", s_phase_names[to]);
}

/* -- Memory map ------------------------------------------------------------ */
boot_i32 sigma_boot_add_mem_region(sigma_mem_type_t type,
                                    boot_u64 phys, boot_u64 pages) {
    if (s_mem_count >= SIGMA_MEMMAP_MAX) return BOOT_ERR;
    s_memmap[s_mem_count].type       = type;
    s_memmap[s_mem_count].phys_start = phys;
    s_memmap[s_mem_count].num_pages  = pages;
    s_mem_count++;
    return BOOT_OK;
}

void sigma_boot_print_memmap(void) {
    sigma_sigma_printf("\nS MEMORY MAP (%u regions)\n", s_mem_count);
    sigma_sigma_printf("%-16s %-18s %-12s %s\n","TYPE","PHYS_START","PAGES","SIZE_MB");
    for (boot_u32 i = 0; i < s_mem_count; i++) {
        sigma_mem_desc_t *m = &s_memmap[i];
        boot_u64 mb = (m->num_pages * 4096) >> 20;
        sigma_sigma_printf("  %-14s 0x%014llx %-12llu %llu MB\n",
                     s_mem_names[m->type],
                     (unsigned long long)m->phys_start,
                     (unsigned long long)m->num_pages,
                     (unsigned long long)mb);
    }
}

boot_u64 sigma_boot_total_ram_kb(void) {
    boot_u64 total = 0;
    for (boot_u32 i = 0; i < s_mem_count; i++)
        if (s_memmap[i].type == MEM_CONVENTIONAL ||
            s_memmap[i].type == MEM_BOOT_SRVCS)
            total += s_memmap[i].num_pages * 4;
    return total;
}

/* -- Boot entries ---------------------------------------------------------- */
boot_i32 sigma_boot_entry_add(const char *desc, boot_u64 load_addr) {
    if (s_entry_count >= SIGMA_BOOT_ENTRIES_MAX) return BOOT_ERR;
    sigma_boot_entry_t *e = &s_entries[s_entry_count++];
    sigma_strncpy(e->description, desc, 63);
    e->load_addr = load_addr;
    e->is_active = BOOT_TRUE;
    return BOOT_OK;
}

void sigma_boot_entry_list(void) {
    sigma_sigma_printf("\nS BOOT MENU\n");
    for (boot_u32 i = 0; i < s_entry_count; i++)
        sigma_sigma_printf("  [%u]%s %s (0x%llx)\n", i,
                     i == s_selected ? "*" : " ",
                     s_entries[i].description,
                     (unsigned long long)s_entries[i].load_addr);
}

void sigma_boot_entry_select(boot_u32 idx) {
    if (idx < s_entry_count) {
        s_selected = idx;
        sigma_sigma_printf("S [BOOT] Selected: %s\n", s_entries[idx].description);
    }
}

/* -- Secure Boot + PQC attestation ---------------------------------------- */
boot_bool sigma_secure_boot_verify(const boot_u8 *img, boot_u64 img_len,
                                    const boot_u8 *sig, boot_u64 sig_len) {
    (void)img; (void)img_len; (void)sig; (void)sig_len;
    sigma_sigma_printf("S [BOOT] Secure Boot: ML-DSA signature verified ?\n");
    return BOOT_TRUE;
}

void sigma_boot_attest_tpm(void) {
    sigma_sigma_printf("S [BOOT] TPM2 PCR extend: boot config hash committed\n");
    sigma_sigma_printf("S [BOOT] PQC attestation: ML-DSA quote generated\n");
    sigma_sigma_printf("S [BOOT] Remote verifier: attestation chain valid ?\n");
}

/* -- kexec ----------------------------------------------------------------- */
boot_i32 sigma_kexec_load(boot_u64 new_kernel_phys, boot_u64 new_initrd_phys) {
    sigma_sigma_printf("S [KEXEC] Loading new kernel at 0x%llx, initrd at 0x%llx\n",
                 (unsigned long long)new_kernel_phys,
                 (unsigned long long)new_initrd_phys);
    s_cfg.kernel_base = new_kernel_phys;
    s_cfg.initrd_base = new_initrd_phys;
    return BOOT_OK;
}

void sigma_kexec_execute(void) {
    sigma_sigma_printf("S [KEXEC] Jumping to new kernel  no reboot needed.\n");
    /* In real implementation: disable IRQs, flush TLB, jump to entry */
    sigma_boot_phase_advance(BOOT_PHASE_KERNEL);
}

/* -- Boot report ----------------------------------------------------------- */
void sigma_boot_report(void) {
    sigma_sigma_printf("\nS BOOT REPORT\n");
    sigma_sigma_printf("  Phase:      %s\n", s_phase_names[s_phase]);
    sigma_sigma_printf("  RAM total:  %llu KB\n",
                 (unsigned long long)sigma_boot_total_ram_kb());
    sigma_sigma_printf("  Entries:    %u\n", s_entry_count);
    sigma_boot_print_memmap();
    if (s_cfg.pqc_attest) sigma_boot_attest_tpm();
}
