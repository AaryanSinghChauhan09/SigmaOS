// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_acpi.cpp — ACPI table parser for SigmaOS
//
// Parses RSDP → XSDT → MADT / FADT / MCFG to discover:
//   • All CPU LAPIC IDs (for SMP boot)
//   • I/O APIC base address and IRQ override table
//   • PCIe MMCONFIG base address (for sigma_pci.cpp)
//   • Power management register blocks (for sigma_acpi_power.cpp)
//
// Inspired by:
//   • Linux drivers/acpi/tables.c, arch/x86/kernel/acpi/boot.c
//   • ACPI Specification 6.5, Chapters 5 (Tables) and 6 (Config)
//   • FreeBSD sys/dev/acpica/acpi_table.c

#include "sigma_acpi.h"
#include <stdint.h>
#include <string.h>

// ── ACPI table signatures ─────────────────────────────────────────────────────

#define ACPI_SIG_RSDP  "RSD PTR "
#define ACPI_SIG_XSDT  "XSDT"
#define ACPI_SIG_MADT  "APIC"
#define ACPI_SIG_FADT  "FACP"
#define ACPI_SIG_MCFG  "MCFG"

// ── Common ACPI structures ────────────────────────────────────────────────────

struct acpi_rsdp {
    char     signature[8];
    uint8_t  checksum;
    char     oem_id[6];
    uint8_t  revision;
    uint32_t rsdt_addr;
    // ACPI 2.0+:
    uint32_t length;
    uint64_t xsdt_addr;
    uint8_t  ext_checksum;
    uint8_t  reserved[3];
} __attribute__((packed));

struct acpi_sdt_header {
    char     signature[4];
    uint32_t length;
    uint8_t  revision;
    uint8_t  checksum;
    char     oem_id[6];
    char     oem_table_id[8];
    uint32_t oem_revision;
    uint32_t creator_id;
    uint32_t creator_revision;
} __attribute__((packed));

struct acpi_xsdt {
    struct acpi_sdt_header hdr;
    uint64_t               entries[];  // 64-bit pointers to other tables
} __attribute__((packed));

// ── MADT (Multiple APIC Description Table) ────────────────────────────────────

struct acpi_madt {
    struct acpi_sdt_header hdr;
    uint32_t lapic_addr;
    uint32_t flags;
    uint8_t  entries[];
} __attribute__((packed));

#define MADT_TYPE_LAPIC      0
#define MADT_TYPE_IOAPIC     1
#define MADT_TYPE_IRQ_OVERRIDE 2

struct madt_lapic {
    uint8_t  type;      // 0
    uint8_t  length;    // 8
    uint8_t  acpi_id;
    uint8_t  apic_id;
    uint32_t flags;     // bit 0: processor enabled
} __attribute__((packed));

struct madt_ioapic {
    uint8_t  type;      // 1
    uint8_t  length;    // 12
    uint8_t  ioapic_id;
    uint8_t  reserved;
    uint32_t ioapic_addr;
    uint32_t global_irq_base;
} __attribute__((packed));

// ── MCFG ─────────────────────────────────────────────────────────────────────

struct acpi_mcfg_entry {
    uint64_t base_addr;
    uint16_t segment;
    uint8_t  start_bus;
    uint8_t  end_bus;
    uint32_t reserved;
} __attribute__((packed));

struct acpi_mcfg {
    struct acpi_sdt_header hdr;
    uint64_t               reserved;
    struct acpi_mcfg_entry entries[];
} __attribute__((packed));

// ── Parsed results ────────────────────────────────────────────────────────────

#define MAX_LAPICS  256
#define MAX_IOAPICS 8

static uint32_t lapic_ids[MAX_LAPICS];
static uint32_t lapic_count = 0;
static uint32_t ioapic_addrs[MAX_IOAPICS];
static uint32_t ioapic_count = 0;
static uint64_t pcie_mmconfig_base = 0;
static uint32_t lapic_mmio_pa = 0xFEE00000;

// ── Checksum validation ───────────────────────────────────────────────────────

static int acpi_checksum(const void *table, uint32_t len) {
    const uint8_t *p = (const uint8_t *)table;
    uint8_t sum = 0;
    for (uint32_t i = 0; i < len; i++) sum += p[i];
    return sum == 0;
}

// ── MADT parsing ─────────────────────────────────────────────────────────────

static void parse_madt(struct acpi_madt *madt) {
    if (madt->lapic_addr) lapic_mmio_pa = madt->lapic_addr;

    uint8_t *p   = madt->entries;
    uint8_t *end = (uint8_t *)madt + madt->hdr.length;

    while (p < end) {
        uint8_t type = p[0];
        uint8_t len  = p[1];
        if (len < 2) break;

        if (type == MADT_TYPE_LAPIC) {
            struct madt_lapic *l = (struct madt_lapic *)p;
            if ((l->flags & 1) && lapic_count < MAX_LAPICS) {
                lapic_ids[lapic_count++] = l->apic_id;
            }
        } else if (type == MADT_TYPE_IOAPIC) {
            struct madt_ioapic *io = (struct madt_ioapic *)p;
            if (ioapic_count < MAX_IOAPICS)
                ioapic_addrs[ioapic_count++] = io->ioapic_addr;
        }
        p += len;
    }
}

static void parse_mcfg(struct acpi_mcfg *mcfg) {
    uint32_t n = (mcfg->hdr.length - sizeof(*mcfg)) / sizeof(mcfg->entries[0]);
    if (n > 0) pcie_mmconfig_base = mcfg->entries[0].base_addr;
}

// ── XSDT traversal ────────────────────────────────────────────────────────────

static void parse_xsdt(struct acpi_xsdt *xsdt) {
    uint32_t n = (xsdt->hdr.length - sizeof(xsdt->hdr)) / 8;
    for (uint32_t i = 0; i < n; i++) {
        struct acpi_sdt_header *hdr =
            (struct acpi_sdt_header *)(uintptr_t)xsdt->entries[i];
        if (!acpi_checksum(hdr, hdr->length)) continue;

        if (memcmp(hdr->signature, ACPI_SIG_MADT, 4) == 0)
            parse_madt((struct acpi_madt *)hdr);
        else if (memcmp(hdr->signature, ACPI_SIG_MCFG, 4) == 0)
            parse_mcfg((struct acpi_mcfg *)hdr);
    }
}

// ── RSDP scan ────────────────────────────────────────────────────────────────

static struct acpi_rsdp *find_rsdp(void) {
    // Scan EBDA and BIOS ROM areas for "RSD PTR " signature
    const uint8_t *search_areas[] = {
        (const uint8_t *)0x000E0000,   // BIOS ROM
        (const uint8_t *)(*(uint16_t *)0x0040E << 4),  // EBDA
    };
    for (int a = 0; a < 2; a++) {
        const uint8_t *p = search_areas[a];
        for (int i = 0; i < 0x20000; i += 16) {
            if (memcmp(p + i, ACPI_SIG_RSDP, 8) == 0) {
                struct acpi_rsdp *r = (struct acpi_rsdp *)(p + i);
                if (acpi_checksum(r, 20)) return r;
            }
        }
    }
    return NULL;
}

// ── Public API ────────────────────────────────────────────────────────────────

int sigma_acpi_init(void) {
    struct acpi_rsdp *rsdp = find_rsdp();
    if (!rsdp) return -1;

    if (rsdp->revision >= 2 && rsdp->xsdt_addr) {
        parse_xsdt((struct acpi_xsdt *)(uintptr_t)rsdp->xsdt_addr);
    }
    return 0;
}

uint32_t sigma_acpi_lapic_count(void)              { return lapic_count; }
uint32_t sigma_acpi_lapic_id(uint32_t i)           { return (i < lapic_count) ? lapic_ids[i] : 0; }
uint32_t sigma_acpi_ioapic_addr(uint32_t i)        { return (i < ioapic_count) ? ioapic_addrs[i] : 0; }
uint32_t sigma_acpi_ioapic_count(void)             { return ioapic_count; }
uint64_t sigma_acpi_pcie_mmconfig_base(void)       { return pcie_mmconfig_base; }
uint32_t sigma_acpi_lapic_mmio_pa(void)            { return lapic_mmio_pa; }
