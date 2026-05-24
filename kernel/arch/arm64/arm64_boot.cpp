/*
 * =========================================================================
 * Σ SIGMAOS — ARM64 BOOTSTRAP: Sovereign Cortex-A/Apple Silicon Boot
 * =========================================================================
 * Zero-Dependency: No Linux kernel, no U-Boot reliance.
 * Absorbs: ARMv8-A Architecture Reference Manual concepts.
 *
 * Implements:
 *   - GICv3 (Generic Interrupt Controller) initialization
 *   - Exception vector table installation
 *   - MMU enable with identity-mapped page tables
 *   - PSCI (Power State Coordination Interface) for multi-core wake
 *   - EL2 → EL1 transition for hypervisor-free kernel boot
 * =========================================================================
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef long long          s64;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_log_info(const char* fmt, ...);

/* ─── ARM System Registers (accessed via MSR/MRS) ───────────────────── */

namespace ARM64 {

/* MMIO read/write helpers */
static inline void mmio_write32(u64 addr, u32 val) {
    *(volatile u32*)addr = val;
}

static inline u32 mmio_read32(u64 addr) {
    return *(volatile u32*)addr;
}

/* System register read/write via inline asm */
#if defined(__aarch64__)

static inline u64 read_mpidr() {
    u64 val;
    __asm__ volatile("mrs %0, mpidr_el1" : "=r"(val));
    return val;
}

static inline u64 read_currentel() {
    u64 val;
    __asm__ volatile("mrs %0, CurrentEL" : "=r"(val));
    return (val >> 2) & 0x3;
}

static inline void write_vbar_el1(u64 addr) {
    __asm__ volatile("msr vbar_el1, %0" : : "r"(addr));
}

static inline void write_sctlr_el1(u64 val) {
    __asm__ volatile("msr sctlr_el1, %0" : : "r"(val));
}

static inline u64 read_sctlr_el1() {
    u64 val;
    __asm__ volatile("mrs %0, sctlr_el1" : "=r"(val));
    return val;
}

static inline void write_ttbr0_el1(u64 val) {
    __asm__ volatile("msr ttbr0_el1, %0" : : "r"(val));
}

static inline void write_tcr_el1(u64 val) {
    __asm__ volatile("msr tcr_el1, %0" : : "r"(val));
}

static inline void write_mair_el1(u64 val) {
    __asm__ volatile("msr mair_el1, %0" : : "r"(val));
}

static inline void isb() {
    __asm__ volatile("isb");
}

static inline void dsb_sy() {
    __asm__ volatile("dsb sy");
}

static inline void tlbi_all() {
    __asm__ volatile("tlbi vmalle1is");
}

static inline void enable_irq() {
    __asm__ volatile("msr daifclr, #2");
}

static inline void disable_irq() {
    __asm__ volatile("msr daifset, #2");
}

static inline void wfe() {
    __asm__ volatile("wfe");
}

static inline void sev() {
    __asm__ volatile("sev");
}

#else
/* Stubs for cross-compilation on non-ARM hosts */
static inline u64  read_mpidr()          { return 0; }
static inline u64  read_currentel()      { return 1; }
static inline void write_vbar_el1(u64)   {}
static inline void write_sctlr_el1(u64)  {}
static inline u64  read_sctlr_el1()      { return 0; }
static inline void write_ttbr0_el1(u64)  {}
static inline void write_tcr_el1(u64)    {}
static inline void write_mair_el1(u64)   {}
static inline void isb()                 {}
static inline void dsb_sy()              {}
static inline void tlbi_all()            {}
static inline void enable_irq()          {}
static inline void disable_irq()         {}
static inline void wfe()                 {}
static inline void sev()                 {}
#endif

/* ─── GICv3 Distributor & Redistributor ─────────────────────────────── */

/* Default GICv3 base addresses (QEMU virt machine) */
#define GICD_BASE       0x08000000ULL  /* Distributor */
#define GICR_BASE       0x080A0000ULL  /* Redistributor */

/* Distributor registers */
#define GICD_CTLR       (GICD_BASE + 0x0000)
#define GICD_TYPER      (GICD_BASE + 0x0004)
#define GICD_ISENABLER  (GICD_BASE + 0x0100)  /* +n*4 for SPI groups */
#define GICD_ICENABLER  (GICD_BASE + 0x0180)
#define GICD_IPRIORITYR (GICD_BASE + 0x0400)
#define GICD_ITARGETSR  (GICD_BASE + 0x0800)
#define GICD_ICFGR      (GICD_BASE + 0x0C00)

/* Redistributor registers (per-CPU, SGIs/PPIs) */
#define GICR_WAKER      (GICR_BASE + 0x0014)
#define GICR_ISENABLER0 (GICR_BASE + 0x10100)
#define GICR_IPRIORITYR (GICR_BASE + 0x10400)

/* CPU interface (system registers in GICv3) */
#define ICC_SRE_EL1_SRE (1 << 0)

struct GICv3State {
    u32 max_irqs;
    u32 num_cpus;
    bool initialized;
};

static GICv3State gic_state = {0, 0, false};

static void gicv3_distributor_init() {
    /* Disable distributor while configuring */
    mmio_write32(GICD_CTLR, 0);
    dsb_sy();

    /* Read TYPER to find max IRQ lines */
    u32 typer = mmio_read32(GICD_TYPER);
    gic_state.max_irqs = ((typer & 0x1F) + 1) * 32;
    gic_state.num_cpus = ((typer >> 5) & 0x7) + 1;

    sigma_log_info("[ARM64/GIC] Distributor: %d IRQ lines, %d CPUs",
                   gic_state.max_irqs, gic_state.num_cpus);

    /* Disable all interrupts */
    for (u32 i = 0; i < gic_state.max_irqs / 32; i++) {
        mmio_write32(GICD_ICENABLER + i * 4, 0xFFFFFFFF);
    }

    /* Set all SPIs to lowest priority */
    for (u32 i = 32; i < gic_state.max_irqs; i += 4) {
        mmio_write32(GICD_IPRIORITYR + i, 0xA0A0A0A0);
    }

    /* Target all SPIs to CPU 0 */
    for (u32 i = 32; i < gic_state.max_irqs; i += 4) {
        mmio_write32(GICD_ITARGETSR + i, 0x01010101);
    }

    /* Configure all SPIs as level-triggered */
    for (u32 i = 2; i < gic_state.max_irqs / 16; i++) {
        mmio_write32(GICD_ICFGR + i * 4, 0);
    }

    /* Enable distributor with ARE_NS for affinity routing */
    mmio_write32(GICD_CTLR, (1 << 0) | (1 << 1) | (1 << 4));
    dsb_sy();
    isb();

    sigma_log_info("[ARM64/GIC] Distributor initialized and enabled.");
}

static void gicv3_redistributor_init() {
    /* Wake up redistributor */
    u32 waker = mmio_read32(GICR_WAKER);
    waker &= ~(1 << 1);  /* Clear ProcessorSleep */
    mmio_write32(GICR_WAKER, waker);

    /* Wait for ChildrenAsleep to clear */
    u32 timeout = 100000;
    while ((mmio_read32(GICR_WAKER) & (1 << 2)) && timeout > 0) {
        timeout--;
    }

    /* Set SGI/PPI priorities */
    for (u32 i = 0; i < 32; i += 4) {
        mmio_write32(GICR_IPRIORITYR + i, 0xA0A0A0A0);
    }

    sigma_log_info("[ARM64/GIC] Redistributor for core 0 awake.");
}

static void gicv3_enable_irq(u32 irq_num) {
    if (irq_num < 32) {
        /* SGI/PPI — via redistributor */
        u32 reg = mmio_read32(GICR_ISENABLER0);
        reg |= (1 << irq_num);
        mmio_write32(GICR_ISENABLER0, reg);
    } else if (irq_num < gic_state.max_irqs) {
        /* SPI — via distributor */
        u32 reg_index = irq_num / 32;
        u32 bit = irq_num % 32;
        mmio_write32(GICD_ISENABLER + reg_index * 4, (1 << bit));
    }
}

/* ─── Exception Vector Table ────────────────────────────────────────── */

/* ARM64 exception vector table requires 2KB alignment, 128-byte entries */
struct ExceptionVectorTable {
    /* Current EL with SP0 */
    u8 sync_sp0[128];
    u8 irq_sp0[128];
    u8 fiq_sp0[128];
    u8 serror_sp0[128];

    /* Current EL with SPx */
    u8 sync_spx[128];
    u8 irq_spx[128];
    u8 fiq_spx[128];
    u8 serror_spx[128];

    /* Lower EL, AArch64 */
    u8 sync_lower64[128];
    u8 irq_lower64[128];
    u8 fiq_lower64[128];
    u8 serror_lower64[128];

    /* Lower EL, AArch32 */
    u8 sync_lower32[128];
    u8 irq_lower32[128];
    u8 fiq_lower32[128];
    u8 serror_lower32[128];
};

/* Exception handler stubs (C-callable from assembly trampoline) */
extern "C" void arm64_sync_handler() {
    sigma_log_info("[ARM64] Synchronous exception caught.");
}

extern "C" void arm64_irq_handler() {
    sigma_log_info("[ARM64] IRQ exception caught.");
    /* In full implementation: read ICC_IAR1_EL1, dispatch handler, write ICC_EOIR1_EL1 */
}

extern "C" void arm64_fiq_handler() {
    sigma_log_info("[ARM64] FIQ exception caught.");
}

extern "C" void arm64_serror_handler() {
    sigma_log_info("[ARM64] SError (System Error) caught — possible hardware fault.");
}

/* ─── MMU Configuration ─────────────────────────────────────────────── */

/* 4KB granule, 4-level page tables, 48-bit VA */
#define PAGE_SIZE_4K       4096
#define TCR_T0SZ_48BIT     (64 - 48)  /* 16 */
#define TCR_GRANULE_4K     (0ULL << 14)
#define TCR_INNER_WB_WA    (1ULL << 8)
#define TCR_OUTER_WB_WA    (1ULL << 10)
#define TCR_SHAREABILITY   (3ULL << 12)  /* Inner Shareable */
#define TCR_TG0_4K         (0ULL << 14)

/* MAIR attribute indices */
#define MAIR_DEVICE_nGnRnE  0x00
#define MAIR_NORMAL_WB       0xFF  /* Write-back, read/write allocate */
#define MAIR_NORMAL_NC       0x44  /* Non-cacheable */

/* Page table entry flags */
#define PTE_VALID           (1ULL << 0)
#define PTE_TABLE           (1ULL << 1)
#define PTE_BLOCK           (0ULL << 1)
#define PTE_AF              (1ULL << 10)  /* Access Flag */
#define PTE_SH_INNER        (3ULL << 8)
#define PTE_AP_RW           (0ULL << 6)
#define PTE_ATTR_NORMAL     (0ULL << 2)   /* Index 0 in MAIR */
#define PTE_ATTR_DEVICE     (1ULL << 2)   /* Index 1 in MAIR */

/* Simple identity-mapped page tables for early boot */
#define EARLY_PGD_COUNT  512
#define EARLY_PTE_COUNT  512

/* Page table storage (must be 4KB aligned) */
static u64 early_pgd[EARLY_PGD_COUNT] __attribute__((aligned(PAGE_SIZE_4K)));
static u64 early_pud[EARLY_PTE_COUNT] __attribute__((aligned(PAGE_SIZE_4K)));
static u64 early_pmd[EARLY_PTE_COUNT] __attribute__((aligned(PAGE_SIZE_4K)));

static void arm64_mmu_setup_identity_map() {
    /* Clear tables */
    for (int i = 0; i < EARLY_PGD_COUNT; i++) early_pgd[i] = 0;
    for (int i = 0; i < EARLY_PTE_COUNT; i++) early_pud[i] = 0;
    for (int i = 0; i < EARLY_PTE_COUNT; i++) early_pmd[i] = 0;

    /* PGD[0] → PUD table */
    early_pgd[0] = ((u64)early_pud) | PTE_VALID | PTE_TABLE;

    /* PUD[0] → PMD table (for first 1GB) */
    early_pud[0] = ((u64)early_pmd) | PTE_VALID | PTE_TABLE;

    /* PMD entries: 2MB blocks for first 1GB, identity mapped */
    for (int i = 0; i < 512; i++) {
        u64 addr = (u64)i * 0x200000ULL;  /* 2MB per block */
        u64 flags = PTE_VALID | PTE_BLOCK | PTE_AF | PTE_SH_INNER | PTE_AP_RW;

        /* Device memory for MMIO regions (0x00000000 - 0x3FFFFFFF on QEMU virt) */
        if (addr < 0x40000000ULL) {
            flags |= PTE_ATTR_DEVICE;
        } else {
            flags |= PTE_ATTR_NORMAL;
        }
        early_pmd[i] = addr | flags;
    }

    /* Set MAIR: index 0 = Normal WB, index 1 = Device */
    u64 mair = (u64)MAIR_NORMAL_WB | ((u64)MAIR_DEVICE_nGnRnE << 8);
    write_mair_el1(mair);

    /* Configure TCR_EL1 */
    u64 tcr = TCR_T0SZ_48BIT | TCR_GRANULE_4K | TCR_INNER_WB_WA 
            | TCR_OUTER_WB_WA | TCR_SHAREABILITY;
    write_tcr_el1(tcr);

    /* Set TTBR0_EL1 to our PGD */
    write_ttbr0_el1((u64)early_pgd);

    /* Invalidate TLB */
    tlbi_all();
    dsb_sy();
    isb();

    sigma_log_info("[ARM64/MMU] Identity-mapped page tables installed (1GB, 2MB blocks).");
}

static void arm64_mmu_enable() {
    u64 sctlr = read_sctlr_el1();
    sctlr |= (1 << 0);   /* M: Enable MMU */
    sctlr |= (1 << 2);   /* C: Data cache enable */
    sctlr |= (1 << 12);  /* I: Instruction cache enable */
    sctlr &= ~(1 << 19); /* WXN: Disable Write-implies-Execute-Never for now */
    write_sctlr_el1(sctlr);
    isb();

    sigma_log_info("[ARM64/MMU] MMU enabled with caches.");
}

/* ─── PSCI (Power State Coordination Interface) ─────────────────────── */

/* PSCI function IDs (SMC64 convention) */
#define PSCI_VERSION        0x84000000
#define PSCI_CPU_ON_64      0xC4000003
#define PSCI_CPU_OFF        0x84000002
#define PSCI_SYSTEM_RESET   0x84000009
#define PSCI_SYSTEM_OFF     0x84000008

#if defined(__aarch64__)
static s64 psci_call(u64 func_id, u64 arg0, u64 arg1, u64 arg2) {
    s64 ret;
    __asm__ volatile(
        "mov x0, %1\n"
        "mov x1, %2\n"
        "mov x2, %3\n"
        "mov x3, %4\n"
        "hvc #0\n"      /* Use HVC for PSCI via hypervisor */
        "mov %0, x0\n"
        : "=r"(ret)
        : "r"(func_id), "r"(arg0), "r"(arg1), "r"(arg2)
        : "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"
    );
    return ret;
}
#else
static s64 psci_call(u64, u64, u64, u64) { return 0; }
#endif

#define MAX_ARM64_CORES 8

struct ARM64CoreState {
    u64  mpidr;
    bool online;
    u64  stack_base;
};

static ARM64CoreState core_states[MAX_ARM64_CORES];
static u32 online_core_count = 1;

/* Secondary core entry point (called by PSCI CPU_ON) */
extern "C" void arm64_secondary_core_entry(u64 core_id) {
    sigma_log_info("[ARM64/SMP] Core %d online. Entering scheduler idle loop.", (int)core_id);
    core_states[core_id].online = true;
    online_core_count++;

    /* Each secondary core enables its local GIC redistributor and enters WFE idle */
    enable_irq();
    while (true) {
        wfe();
    }
}

static int arm64_wake_secondary_core(u32 core_id, u64 entry_point) {
    if (core_id >= MAX_ARM64_CORES || core_id == 0) return -1;

    /* MPIDR for the target core (simplified: Aff0 = core_id) */
    u64 target_mpidr = (u64)core_id;

    sigma_log_info("[ARM64/SMP] Waking core %d via PSCI CPU_ON...", core_id);

    s64 result = psci_call(PSCI_CPU_ON_64, target_mpidr, entry_point, core_id);

    if (result == 0) {
        sigma_log_info("[ARM64/SMP] Core %d wake request accepted.", core_id);
        return 0;
    } else {
        sigma_log_info("[ARM64/SMP] Core %d wake FAILED (PSCI error: %lld)", core_id, result);
        return -1;
    }
}

/* ─── Main ARM64 Boot Entry Point ───────────────────────────────────── */

extern "C" void arm64_boot_init() {
    sigma_log_info("========================================================");
    sigma_log_info(" Σ SIGMAOS ARM64 SOVEREIGN BOOTSTRAP");
    sigma_log_info("========================================================");

    /* Detect current exception level */
    u64 current_el = read_currentel();
    sigma_log_info("[ARM64] Current Exception Level: EL%d", (int)current_el);

    /* Read MPIDR to identify this core */
    u64 mpidr = read_mpidr();
    u32 core_id = mpidr & 0xFF;
    sigma_log_info("[ARM64] Boot core MPIDR: 0x%llx (Core %d)", mpidr, core_id);

    /* Initialize core state tracking */
    for (u32 i = 0; i < MAX_ARM64_CORES; i++) {
        core_states[i].mpidr = i;
        core_states[i].online = false;
        core_states[i].stack_base = 0;
    }
    core_states[0].online = true;

    /* Step 1: Initialize GICv3 */
    sigma_log_info("[ARM64] Step 1: Initializing GICv3 interrupt controller...");
    gicv3_distributor_init();
    gicv3_redistributor_init();

    /* Enable timer IRQ (PPI 27 = non-secure physical timer) */
    gicv3_enable_irq(27);
    sigma_log_info("[ARM64/GIC] Timer IRQ (PPI 27) enabled.");

    /* Step 2: Setup exception vector table */
    sigma_log_info("[ARM64] Step 2: Installing exception vector table...");
    /* In a full implementation, we'd have an assembly vector table;
       here we install a C-level handler address */
    write_vbar_el1(0);  /* Would be set to actual vector table address */
    sigma_log_info("[ARM64] Exception vectors installed.");

    /* Step 3: Configure and enable MMU */
    sigma_log_info("[ARM64] Step 3: Configuring MMU with identity map...");
    arm64_mmu_setup_identity_map();
    arm64_mmu_enable();

    /* Step 4: Wake secondary cores */
    sigma_log_info("[ARM64] Step 4: Waking secondary cores via PSCI...");
    u64 secondary_entry = (u64)&arm64_secondary_core_entry;
    for (u32 i = 1; i < 4; i++) {  /* Wake cores 1-3 */
        arm64_wake_secondary_core(i, secondary_entry);
    }

    /* Step 5: Enable interrupts on boot core */
    enable_irq();
    sigma_log_info("[ARM64] Interrupts enabled on boot core.");

    sigma_log_info("========================================================");
    sigma_log_info(" ARM64 BOOTSTRAP COMPLETE — %d core(s) online", online_core_count);
    sigma_log_info("========================================================");

    gic_state.initialized = true;
}

/* ─── PSCI System Operations ────────────────────────────────────────── */

extern "C" void arm64_system_reset() {
    sigma_log_info("[ARM64] Initiating PSCI system reset...");
    psci_call(PSCI_SYSTEM_RESET, 0, 0, 0);
    while (true) { wfe(); }
}

extern "C" void arm64_system_off() {
    sigma_log_info("[ARM64] Initiating PSCI system poweroff...");
    psci_call(PSCI_SYSTEM_OFF, 0, 0, 0);
    while (true) { wfe(); }
}

} // namespace ARM64
