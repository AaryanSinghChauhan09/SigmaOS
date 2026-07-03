/* SPDX-License-Identifier: MIT
 * tests/unit/kernel/pci_scanner_property_test.c
 *
 * Property 3: PCI Device Field Capture Completeness
 *   Verify that vendor_id, device_id, class_code, subclass, and bar[0..5]
 *   are all captured correctly from a mock PCI config table.
 *
 * Property 4: Empty PCI Slot Exclusion
 *   Verify that slots reporting vendor_id == 0xFFFF are excluded from results.
 *
 * Uses a simulated pci_read_config that reads from a local mock table, so
 * no real hardware access occurs during testing.
 *
 * Build (host runner):
 *   gcc -DTEST_HOST_RUNNER -nostdlib -ffreestanding \
 *       pci_scanner_property_test.c -lc -o pci_scanner_property_test
 *
 * Run:
 *   ./pci_scanner_property_test
 */

#include <stdint.h>
#include <stddef.h>

#ifdef TEST_HOST_RUNNER
#  include <stdio.h>
#  include <stdlib.h>
#  include <string.h>
#endif

/* ── PCI constants ─────────────────────────────────────────────────────────── */

#define PCI_MAX_BUS      256
#define PCI_MAX_SLOT      32
#define PCI_MAX_FUNC       8
#define PCI_INVALID_VENDOR 0xFFFFu

/* Config-space byte offsets (DWORD aligned) */
#define PCI_OFF_VENDOR_DEVICE  0x00u  /* [15:0]=vendor, [31:16]=device */
#define PCI_OFF_CLASS          0x08u  /* [31:24]=class, [23:16]=subclass */
#define PCI_OFF_HEADER         0x0Cu  /* [23:16]=header_type */
#define PCI_OFF_BAR0           0x10u
#define PCI_OFF_BAR1           0x14u
#define PCI_OFF_BAR2           0x18u
#define PCI_OFF_BAR3           0x1Cu
#define PCI_OFF_BAR4           0x20u
#define PCI_OFF_BAR5           0x24u

/* ── PciDevice struct (mirrors kernel/arch/x86_64/pci_scanner.rs) ────────── */

typedef struct __attribute__((packed)) {
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t  class_code;
    uint8_t  subclass;
    uint16_t _pad;
    uint32_t bar[6];
} PciDevice;

/* ── Mock PCI config entry ─────────────────────────────────────────────────── */

/*
 * One entry per (bus, slot, func) triple that the test wants to simulate.
 * The table is terminated by a sentinel with bus=0xFF, slot=0xFF, func=0xFF.
 */
typedef struct {
    uint8_t  bus;
    uint8_t  slot;
    uint8_t  func;
    /* raw DWORD at each relevant config offset */
    uint32_t vendor_device;   /* offset 0x00 */
    uint32_t class_dword;     /* offset 0x08 */
    uint32_t header_dword;    /* offset 0x0C */
    uint32_t bars[6];         /* offsets 0x10..0x24 */
} MockPciEntry;

/* Sentinel value marking end of mock table */
#define MOCK_SENTINEL_BUS  0xFFu
#define MOCK_SENTINEL_SLOT 0xFFu
#define MOCK_SENTINEL_FUNC 0xFFu

/* ── Global mock table (set by each test case) ─────────────────────────────── */

static const MockPciEntry *g_mock_table = NULL;
static size_t              g_mock_count = 0;

/*
 * Simulated pci_read_config.
 * Looks up (bus, slot, func, offset) in g_mock_table.
 * Returns 0xFFFFFFFF if the entry is not found (empty slot behaviour).
 */
static uint32_t sim_pci_read_config(uint8_t bus, uint8_t slot,
                                    uint8_t func, uint8_t offset)
{
    for (size_t i = 0; i < g_mock_count; i++) {
        const MockPciEntry *e = &g_mock_table[i];
        if (e->bus != bus || e->slot != slot || e->func != func) {
            continue;
        }
        switch (offset) {
        case PCI_OFF_VENDOR_DEVICE: return e->vendor_device;
        case PCI_OFF_CLASS:         return e->class_dword;
        case PCI_OFF_HEADER:        return e->header_dword;
        case PCI_OFF_BAR0:          return e->bars[0];
        case PCI_OFF_BAR1:          return e->bars[1];
        case PCI_OFF_BAR2:          return e->bars[2];
        case PCI_OFF_BAR3:          return e->bars[3];
        case PCI_OFF_BAR4:          return e->bars[4];
        case PCI_OFF_BAR5:          return e->bars[5];
        default:                    return 0x00000000u;
        }
    }
    /* Not found → empty slot */
    return 0xFFFFFFFFu;
}

/* ── Simulated scanner (re-implementation using sim_pci_read_config) ──────── */
/*
 * This is a self-contained reimplementation of the scanning logic from
 * pci_scanner.rs, expressed in C using sim_pci_read_config instead of real
 * port I/O.  Properties 3 and 4 are verified against its output.
 */

static size_t sim_pci_scan_devices(PciDevice *devices, size_t max)
{
    size_t count = 0;

    for (int bus = 0; bus < PCI_MAX_BUS && count < max; bus++) {
        for (int slot = 0; slot < PCI_MAX_SLOT && count < max; slot++) {

            /* Probe function 0 */
            uint32_t vd0 = sim_pci_read_config((uint8_t)bus, (uint8_t)slot, 0,
                                               PCI_OFF_VENDOR_DEVICE);
            uint16_t vendor0 = (uint16_t)(vd0 & 0xFFFFu);

            if (vendor0 == PCI_INVALID_VENDOR) {
                continue; /* empty slot */
            }

            /* Populate function 0 */
            {
                uint32_t cls = sim_pci_read_config((uint8_t)bus, (uint8_t)slot,
                                                   0, PCI_OFF_CLASS);
                PciDevice *d = &devices[count++];
                d->vendor_id  = vendor0;
                d->device_id  = (uint16_t)((vd0 >> 16) & 0xFFFFu);
                d->class_code = (uint8_t)((cls >> 24) & 0xFFu);
                d->subclass   = (uint8_t)((cls >> 16) & 0xFFu);
                d->_pad       = 0;
                for (int b = 0; b < 6 && count <= max; b++) {
                    d->bar[b] = sim_pci_read_config((uint8_t)bus, (uint8_t)slot,
                                                    0,
                                                    (uint8_t)(PCI_OFF_BAR0 + b * 4));
                }
            }

            /* Check multi-function bit */
            uint32_t hdr_dword = sim_pci_read_config((uint8_t)bus, (uint8_t)slot,
                                                      0, PCI_OFF_HEADER);
            uint8_t hdr_type = (uint8_t)((hdr_dword >> 16) & 0xFFu);
            int multifunc = (hdr_type & 0x80) != 0;

            if (multifunc) {
                for (int func = 1; func < PCI_MAX_FUNC && count < max; func++) {
                    uint32_t vd = sim_pci_read_config((uint8_t)bus, (uint8_t)slot,
                                                      (uint8_t)func,
                                                      PCI_OFF_VENDOR_DEVICE);
                    uint16_t vendor = (uint16_t)(vd & 0xFFFFu);
                    if (vendor == PCI_INVALID_VENDOR) {
                        continue;
                    }
                    uint32_t cls = sim_pci_read_config((uint8_t)bus, (uint8_t)slot,
                                                       (uint8_t)func,
                                                       PCI_OFF_CLASS);
                    PciDevice *d = &devices[count++];
                    d->vendor_id  = vendor;
                    d->device_id  = (uint16_t)((vd >> 16) & 0xFFFFu);
                    d->class_code = (uint8_t)((cls >> 24) & 0xFFu);
                    d->subclass   = (uint8_t)((cls >> 16) & 0xFFu);
                    d->_pad       = 0;
                    for (int b = 0; b < 6; b++) {
                        d->bar[b] = sim_pci_read_config((uint8_t)bus, (uint8_t)slot,
                                                        (uint8_t)func,
                                                        (uint8_t)(PCI_OFF_BAR0 + b * 4));
                    }
                }
            }
        }
    }
    return count;
}

/* ── Helper macros ─────────────────────────────────────────────────────────── */

#define ARRAY_LEN(a) (sizeof(a) / sizeof((a)[0]))

/* ── Property 3: Field Capture Completeness ────────────────────────────────── */

/*
 * Build a mock table with N well-defined devices and verify that:
 *   (a) vendor_id is captured correctly
 *   (b) device_id is captured correctly
 *   (c) class_code is captured correctly
 *   (d) subclass is captured correctly
 *   (e) all six BAR values are captured correctly
 */
static int prop3_field_capture_completeness(void)
{
    int failures = 0;

    /* --- Sub-test 3a: single device, every field distinct ------------------- */
    {
        static const MockPciEntry table[] = {
            {
                .bus  = 0, .slot = 1, .func = 0,
                /* vendor=0x8086, device=0x1234 */
                .vendor_device = 0x12348086u,
                /* class=0x01, subclass=0x06 (storage/SATA) */
                .class_dword   = 0x01060000u,
                /* header_type=0x00 (single-function) */
                .header_dword  = 0x00000000u,
                .bars = {
                    0xFEBC0000u, 0xFEBD0000u, 0xFEBE0000u,
                    0xFEBF0000u, 0xFEC00000u, 0xFEC10000u
                }
            }
        };
        g_mock_table = table;
        g_mock_count = ARRAY_LEN(table);

        PciDevice results[8];
        size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

        if (count != 1) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] 3a: expected 1 device, got %zu\n", count);
#endif
            failures++;
        } else {
            const PciDevice *d = &results[0];

            if (d->vendor_id != 0x8086u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3a: vendor_id=0x%04X (expected 0x8086)\n", d->vendor_id);
#endif
                failures++;
            }
            if (d->device_id != 0x1234u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3a: device_id=0x%04X (expected 0x1234)\n", d->device_id);
#endif
                failures++;
            }
            if (d->class_code != 0x01u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3a: class_code=0x%02X (expected 0x01)\n", d->class_code);
#endif
                failures++;
            }
            if (d->subclass != 0x06u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3a: subclass=0x%02X (expected 0x06)\n", d->subclass);
#endif
                failures++;
            }
            for (int b = 0; b < 6; b++) {
                uint32_t expected_bar = 0xFEBC0000u + (uint32_t)(b * 0x00010000u);
                if (d->bar[b] != expected_bar) {
#ifdef TEST_HOST_RUNNER
                    printf("  [FAIL] 3a: bar[%d]=0x%08X (expected 0x%08X)\n",
                           b, d->bar[b], expected_bar);
#endif
                    failures++;
                }
            }
        }
    }

    /* --- Sub-test 3b: multiple devices on different slots/buses ------------- */
    {
        static const MockPciEntry table[] = {
            /* Bus 0, Slot 2, Func 0: network controller */
            {
                .bus=0, .slot=2, .func=0,
                .vendor_device = 0xABCD10ECu, /* vendor=10EC (Realtek), dev=ABCD */
                .class_dword   = 0x02000000u, /* class=0x02 (network) */
                .header_dword  = 0x00000000u,
                .bars = { 0x1000, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000 }
            },
            /* Bus 1, Slot 0, Func 0: display controller */
            {
                .bus=1, .slot=0, .func=0,
                .vendor_device = 0x9999DEF0u, /* vendor=DEF0, dev=9999 */
                .class_dword   = 0x03000000u, /* class=0x03 (display) */
                .header_dword  = 0x00000000u,
                .bars = { 0xA000, 0xB000, 0xC000, 0xD000, 0xE000, 0xF000 }
            },
        };
        g_mock_table = table;
        g_mock_count = ARRAY_LEN(table);

        PciDevice results[16];
        size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

        if (count != 2) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] 3b: expected 2 devices, got %zu\n", count);
#endif
            failures++;
        } else {
            /* Device 0 — network controller */
            if (results[0].vendor_id != 0x10ECu || results[0].device_id != 0xABCDu) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3b: device[0] VID/DID mismatch\n");
#endif
                failures++;
            }
            if (results[0].class_code != 0x02u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3b: device[0] class_code=%02X (expected 02)\n",
                       results[0].class_code);
#endif
                failures++;
            }
            if (results[0].bar[0] != 0x1000u || results[0].bar[5] != 0x6000u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3b: device[0] BAR0/BAR5 mismatch\n");
#endif
                failures++;
            }

            /* Device 1 — display controller */
            if (results[1].vendor_id != 0xDEF0u || results[1].device_id != 0x9999u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3b: device[1] VID/DID mismatch\n");
#endif
                failures++;
            }
            if (results[1].class_code != 0x03u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3b: device[1] class_code=%02X (expected 03)\n",
                       results[1].class_code);
#endif
                failures++;
            }
        }
    }

    /* --- Sub-test 3c: multi-function device, per-function fields distinct --- */
    {
        static const MockPciEntry table[] = {
            /* Function 0 */
            {
                .bus=0, .slot=5, .func=0,
                .vendor_device = 0x00018086u,
                .class_dword   = 0x0C030000u, /* USB 3.0 */
                /* header_type bit 7 set → multi-function */
                .header_dword  = 0x00800000u,
                .bars = { 0x10, 0x20, 0x30, 0x40, 0x50, 0x60 }
            },
            /* Function 1 */
            {
                .bus=0, .slot=5, .func=1,
                .vendor_device = 0x00028086u,
                .class_dword   = 0x0C030100u, /* USB 3.1 */
                .header_dword  = 0x00000000u,
                .bars = { 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0 }
            },
        };
        g_mock_table = table;
        g_mock_count = ARRAY_LEN(table);

        PciDevice results[16];
        size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

        if (count != 2) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] 3c: expected 2 functions, got %zu\n", count);
#endif
            failures++;
        } else {
            /* Function 0 */
            if (results[0].vendor_id != 0x8086u || results[0].device_id != 0x0001u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3c: func0 VID/DID mismatch\n");
#endif
                failures++;
            }
            if (results[0].subclass != 0x03u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3c: func0 subclass=%02X (expected 03)\n",
                       results[0].subclass);
#endif
                failures++;
            }
            /* Function 1 */
            if (results[1].device_id != 0x0002u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3c: func1 device_id=0x%04X (expected 0x0002)\n",
                       results[1].device_id);
#endif
                failures++;
            }
            if (results[1].bar[0] != 0xA0u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 3c: func1 bar[0]=0x%X (expected 0xA0)\n",
                       results[1].bar[0]);
#endif
                failures++;
            }
        }
    }

    return failures == 0;
}

/* ── Property 4: Empty PCI Slot Exclusion ──────────────────────────────────── */

/*
 * Verify that:
 *   (a) a table with only 0xFFFF vendors yields count == 0
 *   (b) a mixed table (some 0xFFFF, some real) yields only the real devices
 *   (c) a device immediately after an empty slot is not skipped
 *   (d) generating N empty slots and M real devices yields exactly M results
 */
static int prop4_empty_slot_exclusion(void)
{
    int failures = 0;

    /* --- Sub-test 4a: all-empty table ---------------------------------------- */
    {
        /* No entries → sim_pci_read_config returns 0xFFFFFFFF everywhere */
        g_mock_table = NULL;
        g_mock_count = 0;

        PciDevice results[8];
        size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

        if (count != 0) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] 4a: expected 0 devices, got %zu\n", count);
#endif
            failures++;
        }
    }

    /* --- Sub-test 4b: one real device surrounded by empty slots -------------- */
    {
        static const MockPciEntry table[] = {
            {
                .bus=0, .slot=10, .func=0,
                .vendor_device = 0x43218765u,
                .class_dword   = 0x04010000u,
                .header_dword  = 0x00000000u,
                .bars = { 1, 2, 3, 4, 5, 6 }
            }
        };
        g_mock_table = table;
        g_mock_count = ARRAY_LEN(table);

        PciDevice results[64];
        size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

        if (count != 1) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] 4b: expected 1 device, got %zu\n", count);
#endif
            failures++;
        } else {
            if (results[0].vendor_id != 0x8765u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 4b: vendor_id=0x%04X (expected 0x8765)\n",
                       results[0].vendor_id);
#endif
                failures++;
            }
        }
    }

    /* --- Sub-test 4c: adjacent slots, first empty, second real -------------- */
    {
        static const MockPciEntry table[] = {
            /* Slot 3 is present; slot 2 is implicitly absent (0xFFFF vendor) */
            {
                .bus=0, .slot=3, .func=0,
                .vendor_device = 0xBEEF1DE0u,
                .class_dword   = 0x05020000u,
                .header_dword  = 0x00000000u,
                .bars = { 0xDEAD, 0xBEEF, 0xCAFE, 0xBABE, 0xF00D, 0xACE0 }
            }
        };
        g_mock_table = table;
        g_mock_count = ARRAY_LEN(table);

        PciDevice results[8];
        size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

        if (count != 1) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] 4c: expected 1 device, got %zu\n", count);
#endif
            failures++;
        } else {
            if (results[0].vendor_id != 0x1DE0u) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 4c: vendor_id=0x%04X (expected 0x1DE0)\n",
                       results[0].vendor_id);
#endif
                failures++;
            }
        }
    }

    /* --- Sub-test 4d: theft-style sweep — vary M real devices 1..8 ---------- */
    /* Each sweep places M devices at fixed slots and M "gaps" in between,
     * then verifies the count equals M exactly. */
    {
        static MockPciEntry table[8];
        const uint16_t vendors[8] = {
            0x1000, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000, 0x8000
        };
        const uint16_t devices_ids[8] = {
            0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008
        };

        for (int m = 1; m <= 8; m++) {
            /* Place m devices at slots 0, 2, 4 … (2*m)-2 on bus 0 */
            for (int i = 0; i < m; i++) {
                table[i].bus  = 0;
                table[i].slot = (uint8_t)(i * 2); /* even slots only */
                table[i].func = 0;
                table[i].vendor_device =
                    ((uint32_t)devices_ids[i] << 16) | vendors[i];
                table[i].class_dword  = 0x06040000u;
                table[i].header_dword = 0x00000000u;
                for (int b = 0; b < 6; b++) {
                    table[i].bars[b] = (uint32_t)(i * 100 + b);
                }
            }

            g_mock_table = table;
            g_mock_count = (size_t)m;

            PciDevice results[32];
            size_t count = sim_pci_scan_devices(results, ARRAY_LEN(results));

            if (count != (size_t)m) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] 4d: m=%d expected %d devices, got %zu\n",
                       m, m, count);
#endif
                failures++;
            }

            /* Verify no result has vendor == 0xFFFF */
            for (size_t k = 0; k < count; k++) {
                if (results[k].vendor_id == PCI_INVALID_VENDOR) {
#ifdef TEST_HOST_RUNNER
                    printf("  [FAIL] 4d: results[%zu] has invalid vendor 0xFFFF\n", k);
#endif
                    failures++;
                }
            }
        }
    }

    return failures == 0;
}

/* ── Test runner ────────────────────────────────────────────────────────────── */

#ifdef TEST_HOST_RUNNER

#define RUN(name, fn)                                                  \
    do {                                                               \
        int _ok = (fn);                                                \
        if (_ok) { printf("[PASS] %s\n", name); passed++; }           \
        else     { printf("[FAIL] %s\n", name); failed++;  }          \
    } while (0)

int main(void)
{
    int passed = 0, failed = 0;

    printf("=== PCI Scanner Property Tests ===\n");

    RUN("Property 3: PCI Device Field Capture Completeness",
        prop3_field_capture_completeness());

    RUN("Property 4: Empty PCI Slot Exclusion",
        prop4_empty_slot_exclusion());

    printf("\nResults: %d passed, %d failed\n", passed, failed);
    return failed == 0 ? 0 : 1;
}

#endif /* TEST_HOST_RUNNER */
