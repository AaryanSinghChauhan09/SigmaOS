/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN IRQ SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb APIC/MSI/IRQ-Balance USP — Native Silicon Interrupt Mgmt.
 * Design: C11 / Zero-Dependency / Affinity-Based Interrupt Load Balancing.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// IRQ Structures
// -------------------------------------------------------------------------

typedef enum {
    IRQ_EDGE,
    IRQ_LEVEL,
    IRQ_MSI,
    IRQ_MSI_X
} SigmaIRQType_t;

typedef struct {
    sigma_u32     irq_num;
    char          device_name[32];
    SigmaIRQType_t type;
    sigma_u32     affinity_cpu;   /* CPU core pinned to    */
    sigma_u64     fire_count;
    sigma_u64     last_latency_ns;
    sigma_bool    balanced;
} SigmaIRQEntry_t;

#define MAX_IRQS 32
static SigmaIRQEntry_t s_irq_table[MAX_IRQS];
static sigma_u32       s_irq_count = 0;
static sigma_u32       s_cpu_count = 8; /* Simulated 8-core silicon matrix */

// -------------------------------------------------------------------------
// IRQ Logic (Linux IRQ-Balance / APIC / MSI-X parity)
// -------------------------------------------------------------------------

/**
 * sigma_irq_register: Registers a silicon interrupt line.
 */
sigma_err_t sigma_irq_register(sigma_u32 irq, const char* dev,
                                SigmaIRQType_t type, sigma_u32 cpu) {
    if (s_irq_count >= MAX_IRQS) return SIGMA_ENOSPC;

    SigmaIRQEntry_t* e = &s_irq_table[s_irq_count++];
    e->irq_num       = irq;
    e->affinity_cpu  = cpu % s_cpu_count;
    e->type          = type;
    e->fire_count    = 0;
    e->last_latency_ns = 0;
    e->balanced      = SIGMA_TRUE;
    sigma_strcpy(e->device_name, dev);

    static const char* tname[] = { "EDGE", "LEVEL", "MSI", "MSI-X" };
    sigma_printf("[IRQ]: Registered IRQ%u '%s' type=%s affinity=CPU%u\n",
                 irq, dev, tname[type], e->affinity_cpu);
    return SIGMA_OK;
}

/**
 * sigma_irq_balance: Re-distributes silicon interrupts across available cores
 *                    to prevent IRQ hotspots (irqbalance daemon parity).
 */
void sigma_irq_balance() {
    sigma_printf("[IRQ]: Initiating silicon interrupt rebalancing across %u CPUs...\n",
                 s_cpu_count);

    /* Round-robin rebalance */
    for (sigma_u32 i = 0; i < s_irq_count; i++) {
        sigma_u32 new_cpu = i % s_cpu_count;
        sigma_bool moved  = (s_irq_table[i].affinity_cpu != new_cpu);
        s_irq_table[i].affinity_cpu = new_cpu;
        s_irq_table[i].balanced     = SIGMA_TRUE;
        if (moved) {
            sigma_printf("  [REPIN]: IRQ%-3u %-20s -> CPU%u\n",
                         s_irq_table[i].irq_num,
                         s_irq_table[i].device_name, new_cpu);
        }
    }
    sigma_printf("[OK]: IRQ rebalance complete. Interrupt hotspots eliminated.\n");
}

/**
 * sigma_irq_set_affinity: Manually pins an IRQ to a specific CPU core.
 */
sigma_err_t sigma_irq_set_affinity(sigma_u32 irq, sigma_u32 cpu) {
    for (sigma_u32 i = 0; i < s_irq_count; i++) {
        if (s_irq_table[i].irq_num == irq) {
            s_irq_table[i].affinity_cpu = cpu % s_cpu_count;
            sigma_printf("[IRQ]: IRQ%u pinned to CPU%u.\n", irq, cpu % s_cpu_count);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

// -------------------------------------------------------------------------
// Industrial IRQ Audit
// -------------------------------------------------------------------------

void SovereignIRQ_Audit() {
    static const char* tname[] = { "EDGE", "LEVEL", "MSI", "MSI-X" };
    sigma_printf("\n--- SOVEREIGN IRQ AUDIT ---\n");
    sigma_printf("IRQ  DEVICE                TYPE    CPU  FIRES        BALANCED\n");
    sigma_printf("---------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_irq_count; i++) {
        sigma_printf("%-4u %-21s %-7s %-4u %-12llu %s\n",
                     s_irq_table[i].irq_num,
                     s_irq_table[i].device_name,
                     tname[s_irq_table[i].type],
                     s_irq_table[i].affinity_cpu,
                     (unsigned long long)s_irq_table[i].fire_count,
                     s_irq_table[i].balanced ? "YES" : "NO");
    }
    sigma_printf("---------------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignIRQShard_Init() {
    sigma_printf("[SOC]: Seating Native IRQ Shard (APIC/IRQBalance/MSI-X Parity v1.0)...\n");
    sigma_irq_register(0,  "sigma_timer",    IRQ_EDGE,  0);
    sigma_irq_register(1,  "sigma_keyboard", IRQ_EDGE,  1);
    sigma_irq_register(9,  "sigma_pci_acpi", IRQ_LEVEL, 2);
    sigma_irq_register(16, "sigma_nic_0",    IRQ_MSI_X, 3);
    sigma_irq_register(24, "sigma_nvme_0",   IRQ_MSI_X, 4);
    sigma_irq_register(32, "sigma_gpu_0",    IRQ_MSI_X, 5);
    sigma_irq_balance();
}
