/*
 * =========================================================================
 * Σ SIGMAOS — RISC-V 64 BOOTSTRAP: Sovereign RV64GC Boot Sequence
 * =========================================================================
 * Zero-Dependency: No Linux kernel, no OpenSBI reliance for runtime.
 * Absorbs: RISC-V Privileged Specification v1.12, SBI v2.0 concepts.
 *
 * Implements:
 *   - CLINT (Core Local Interruptor) timer initialization
 *   - PLIC (Platform-Level Interrupt Controller) configuration
 *   - SBI (Supervisor Binary Interface) ecall wrappers
 *   - Hart enumeration and secondary hart boot
 *   - Sv48 page table setup for virtual memory
 *   - CSR (Control and Status Register) management
 * =========================================================================
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef long long          s64;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_log_info(const char* fmt, ...);

namespace RISCV64 {

/* ─── MMIO Helpers ──────────────────────────────────────────────────── */

static inline void mmio_write32(u64 addr, u32 val) {
    *(volatile u32*)addr = val;
}

static inline u32 mmio_read32(u64 addr) {
    return *(volatile u32*)addr;
}

static inline void mmio_write64(u64 addr, u64 val) {
    *(volatile u64*)addr = val;
}

static inline u64 mmio_read64(u64 addr) {
    return *(volatile u64*)addr;
}

/* ─── CSR Access (via inline assembly) ──────────────────────────────── */

#if defined(__riscv)

static inline u64 read_mhartid() {
    u64 val;
    __asm__ volatile("csrr %0, mhartid" : "=r"(val));
    return val;
}

static inline u64 read_sstatus() {
    u64 val;
    __asm__ volatile("csrr %0, sstatus" : "=r"(val));
    return val;
}

static inline void write_sstatus(u64 val) {
    __asm__ volatile("csrw sstatus, %0" : : "r"(val));
}

static inline void write_stvec(u64 val) {
    __asm__ volatile("csrw stvec, %0" : : "r"(val));
}

static inline void write_sie(u64 val) {
    __asm__ volatile("csrw sie, %0" : : "r"(val));
}

static inline u64 read_sie() {
    u64 val;
    __asm__ volatile("csrr %0, sie" : "=r"(val));
    return val;
}

static inline void write_satp(u64 val) {
    __asm__ volatile("csrw satp, %0" : : "r"(val));
}

static inline u64 read_time() {
    u64 val;
    __asm__ volatile("csrr %0, time" : "=r"(val));
    return val;
}

static inline void sfence_vma() {
    __asm__ volatile("sfence.vma zero, zero");
}

static inline void wfi() {
    __asm__ volatile("wfi");
}

#else
/* Cross-compilation stubs */
static inline u64  read_mhartid()       { return 0; }
static inline u64  read_sstatus()       { return 0; }
static inline void write_sstatus(u64)   {}
static inline void write_stvec(u64)     {}
static inline void write_sie(u64)       {}
static inline u64  read_sie()           { return 0; }
static inline void write_satp(u64)      {}
static inline u64  read_time()          { return 0; }
static inline void sfence_vma()         {}
static inline void wfi()                {}
#endif

/* ─── SBI (Supervisor Binary Interface) ─────────────────────────────── */

/* SBI Extension IDs */
#define SBI_EXT_BASE            0x10
#define SBI_EXT_TIMER           0x54494D45  /* TIME */
#define SBI_EXT_IPI             0x735049    /* sPI  */
#define SBI_EXT_RFENCE          0x52464E43  /* RFNC */
#define SBI_EXT_HSM             0x48534D    /* HSM  */
#define SBI_EXT_SRST            0x53525354  /* SRST */

/* SBI Function IDs */
#define SBI_TIMER_SET_TIMER     0
#define SBI_IPI_SEND_IPI        0
#define SBI_HSM_HART_START      0
#define SBI_HSM_HART_STOP       1
#define SBI_HSM_HART_STATUS     2
#define SBI_SRST_SYSTEM_RESET   0

struct SBIReturn {
    s64 error;
    s64 value;
};

static SBIReturn sbi_ecall(u64 ext, u64 func, u64 a0 = 0, u64 a1 = 0, u64 a2 = 0) {
    SBIReturn ret;
#if defined(__riscv)
    register u64 r_a0 __asm__("a0") = a0;
    register u64 r_a1 __asm__("a1") = a1;
    register u64 r_a2 __asm__("a2") = a2;
    register u64 r_a6 __asm__("a6") = func;
    register u64 r_a7 __asm__("a7") = ext;

    __asm__ volatile(
        "ecall"
        : "+r"(r_a0), "+r"(r_a1)
        : "r"(r_a2), "r"(r_a6), "r"(r_a7)
        : "memory"
    );

    ret.error = (s64)r_a0;
    ret.value = (s64)r_a1;
#else
    ret.error = 0;
    ret.value = 0;
#endif
    return ret;
}

static void sbi_set_timer(u64 stime_value) {
    sbi_ecall(SBI_EXT_TIMER, SBI_TIMER_SET_TIMER, stime_value);
}

static void sbi_send_ipi(u64 hart_mask, u64 hart_mask_base) {
    sbi_ecall(SBI_EXT_IPI, SBI_IPI_SEND_IPI, hart_mask, hart_mask_base);
}

static s64 sbi_hart_start(u64 hartid, u64 start_addr, u64 opaque) {
    SBIReturn r = sbi_ecall(SBI_EXT_HSM, SBI_HSM_HART_START, hartid, start_addr, opaque);
    return r.error;
}

static s64 sbi_hart_get_status(u64 hartid) {
    SBIReturn r = sbi_ecall(SBI_EXT_HSM, SBI_HSM_HART_STATUS, hartid);
    if (r.error == 0) return r.value;
    return -1;
}

/* ─── CLINT (Core Local Interruptor) ────────────────────────────────── */

/* QEMU virt defaults */
#define CLINT_BASE       0x02000000ULL
#define CLINT_MSIP(hart) (CLINT_BASE + 0x0000 + (hart) * 4)
#define CLINT_MTIMECMP(hart) (CLINT_BASE + 0x4000 + (hart) * 8)
#define CLINT_MTIME      (CLINT_BASE + 0xBFF8)

/* Timer frequency: 10 MHz on QEMU virt */
#define TIMER_FREQ_HZ    10000000ULL
#define TICK_INTERVAL_MS  10

static void clint_set_timer(u32 hart_id, u64 delta_ms) {
    u64 current_time = mmio_read64(CLINT_MTIME);
    u64 target = current_time + (TIMER_FREQ_HZ * delta_ms / 1000);

    /* Use SBI to set timer (S-mode uses SBI, not direct CLINT access) */
    sbi_set_timer(target);

    sigma_log_info("[RISCV/CLINT] Timer set for hart %d: +%dms (target=%llu)",
                   hart_id, (int)delta_ms, target);
}

/* ─── PLIC (Platform-Level Interrupt Controller) ────────────────────── */

#define PLIC_BASE               0x0C000000ULL
#define PLIC_PRIORITY(irq)      (PLIC_BASE + (irq) * 4)
#define PLIC_PENDING(word)      (PLIC_BASE + 0x1000 + (word) * 4)
#define PLIC_ENABLE(ctx, word)  (PLIC_BASE + 0x2000 + (ctx) * 0x80 + (word) * 4)
#define PLIC_THRESHOLD(ctx)     (PLIC_BASE + 0x200000 + (ctx) * 0x1000)
#define PLIC_CLAIM(ctx)         (PLIC_BASE + 0x200004 + (ctx) * 0x1000)

#define PLIC_MAX_IRQS   1024
#define PLIC_MAX_CTXS   16

struct PLICState {
    u32 max_irqs;
    bool initialized;
};

static PLICState plic_state = {0, false};

static void plic_init() {
    sigma_log_info("[RISCV/PLIC] Initializing Platform-Level Interrupt Controller...");

    /* Set all IRQ priorities to 0 (disabled) */
    for (u32 i = 1; i < PLIC_MAX_IRQS; i++) {
        mmio_write32(PLIC_PRIORITY(i), 0);
    }

    /* S-mode context for hart 0 is context 1 (M-mode = 0, S-mode = 1) */
    u32 ctx = 1;

    /* Disable all IRQs for this context */
    for (u32 word = 0; word < PLIC_MAX_IRQS / 32; word++) {
        mmio_write32(PLIC_ENABLE(ctx, word), 0);
    }

    /* Set threshold to 0 (accept all priorities > 0) */
    mmio_write32(PLIC_THRESHOLD(ctx), 0);

    plic_state.max_irqs = PLIC_MAX_IRQS;
    plic_state.initialized = true;

    sigma_log_info("[RISCV/PLIC] PLIC initialized. Threshold set to 0.");
}

static void plic_enable_irq(u32 irq, u32 priority) {
    if (irq == 0 || irq >= PLIC_MAX_IRQS) return;

    /* Set priority */
    mmio_write32(PLIC_PRIORITY(irq), priority);

    /* Enable in S-mode context (context 1 for hart 0) */
    u32 ctx = 1;
    u32 word = irq / 32;
    u32 bit = irq % 32;
    u32 val = mmio_read32(PLIC_ENABLE(ctx, word));
    val |= (1 << bit);
    mmio_write32(PLIC_ENABLE(ctx, word), val);

    sigma_log_info("[RISCV/PLIC] IRQ %d enabled at priority %d.", irq, priority);
}

static u32 plic_claim() {
    return mmio_read32(PLIC_CLAIM(1));
}

static void plic_complete(u32 irq) {
    mmio_write32(PLIC_CLAIM(1), irq);
}

/* ─── Sv48 Page Table Setup ─────────────────────────────────────────── */

/* Sv48: 4-level page table, 48-bit virtual address space */
#define SATP_MODE_SV48  (9ULL << 60)
#define PAGE_SIZE_4K    4096

/* Page table entry flags */
#define PTE_V    (1ULL << 0)   /* Valid */
#define PTE_R    (1ULL << 1)   /* Read */
#define PTE_W    (1ULL << 2)   /* Write */
#define PTE_X    (1ULL << 3)   /* Execute */
#define PTE_U    (1ULL << 4)   /* User */
#define PTE_G    (1ULL << 5)   /* Global */
#define PTE_A    (1ULL << 6)   /* Accessed */
#define PTE_D    (1ULL << 7)   /* Dirty */

/* 1GB superpages for early identity map */
#define SUPERPAGE_1G   0x40000000ULL

/* Page table storage */
static u64 root_page_table[512] __attribute__((aligned(PAGE_SIZE_4K)));

static void sv48_setup_identity_map() {
    /* Clear root table */
    for (int i = 0; i < 512; i++) root_page_table[i] = 0;

    /* Map first 4GB using 1GB superpages (level-2 leaf entries) */
    /* In Sv48, a superpage at level 2 (third level from root) maps 1GB */
    for (u64 i = 0; i < 4; i++) {
        u64 phys_addr = i * SUPERPAGE_1G;
        /* PPN stored in bits [53:10], physical page number = phys_addr >> 12 */
        u64 ppn = phys_addr >> 12;
        root_page_table[i] = (ppn << 10) | PTE_V | PTE_R | PTE_W | PTE_X | PTE_A | PTE_D | PTE_G;
    }

    sigma_log_info("[RISCV/MMU] Sv48 identity map: 4GB mapped via 1GB superpages.");
}

static void sv48_enable() {
    u64 root_ppn = ((u64)root_page_table) >> 12;
    u64 satp_val = SATP_MODE_SV48 | root_ppn;

    sfence_vma();
    write_satp(satp_val);
    sfence_vma();

    sigma_log_info("[RISCV/MMU] Sv48 MMU enabled. SATP = 0x%llx", satp_val);
}

/* ─── Hart Management ───────────────────────────────────────────────── */

#define MAX_HARTS 8

struct HartState {
    u64  hart_id;
    bool online;
    u64  stack_base;
};

static HartState hart_states[MAX_HARTS];
static u32 online_hart_count = 1;

/* Secondary hart entry point */
extern "C" void riscv64_secondary_hart_entry(u64 hart_id) {
    sigma_log_info("[RISCV/SMP] Hart %d online. Entering idle loop.", (int)hart_id);
    hart_states[hart_id].online = true;
    online_hart_count++;

    /* Enable S-mode interrupts */
    u64 sie = read_sie();
    sie |= (1 << 1) | (1 << 5) | (1 << 9);  /* SSIE, STIE, SEIE */
    write_sie(sie);

    /* Enable global interrupts */
    u64 sstatus = read_sstatus();
    sstatus |= (1 << 1);  /* SIE bit */
    write_sstatus(sstatus);

    while (true) {
        wfi();
    }
}

static int riscv64_wake_hart(u32 hart_id, u64 entry_point) {
    if (hart_id >= MAX_HARTS || hart_id == 0) return -1;

    sigma_log_info("[RISCV/SMP] Starting hart %d via SBI HSM...", hart_id);

    s64 result = sbi_hart_start(hart_id, entry_point, hart_id);

    if (result == 0) {
        sigma_log_info("[RISCV/SMP] Hart %d start request accepted.", hart_id);
        return 0;
    } else {
        sigma_log_info("[RISCV/SMP] Hart %d start FAILED (SBI error: %lld)", hart_id, result);
        return -1;
    }
}

/* ─── Exception/Interrupt Vector Handler ────────────────────────────── */

extern "C" void riscv64_trap_handler() {
    /* Read scause to determine trap type */
    sigma_log_info("[RISCV] Trap handler invoked.");

    /* In full implementation:
     * - Read scause (interrupt vs exception, cause code)
     * - For timer interrupt: clear timer, call scheduler tick
     * - For external interrupt: PLIC claim → dispatch → complete
     * - For exceptions: page fault handling, illegal instruction, etc.
     */
}

/* ─── Main RISC-V Boot Entry Point ──────────────────────────────────── */

extern "C" void riscv64_boot_init() {
    sigma_log_info("========================================================");
    sigma_log_info(" Σ SIGMAOS RISC-V 64 SOVEREIGN BOOTSTRAP");
    sigma_log_info("========================================================");

    /* Read hart ID */
    u64 hart_id = read_mhartid();
    sigma_log_info("[RISCV] Boot hart ID: %d", (int)hart_id);

    /* Initialize hart state tracking */
    for (u32 i = 0; i < MAX_HARTS; i++) {
        hart_states[i].hart_id = i;
        hart_states[i].online = false;
        hart_states[i].stack_base = 0;
    }
    hart_states[hart_id].online = true;

    /* Step 1: Initialize PLIC */
    sigma_log_info("[RISCV] Step 1: Initializing PLIC...");
    plic_init();

    /* Enable UART IRQ (typically IRQ 10 on QEMU virt) */
    plic_enable_irq(10, 1);

    /* Step 2: Setup CLINT timer */
    sigma_log_info("[RISCV] Step 2: Configuring CLINT timer...");
    clint_set_timer((u32)hart_id, TICK_INTERVAL_MS);

    /* Step 3: Install trap vector */
    sigma_log_info("[RISCV] Step 3: Installing S-mode trap vector...");
    write_stvec((u64)&riscv64_trap_handler);
    sigma_log_info("[RISCV] Trap vector installed at 0x%llx",
                   (u64)&riscv64_trap_handler);

    /* Step 4: Configure Sv48 page tables */
    sigma_log_info("[RISCV] Step 4: Setting up Sv48 virtual memory...");
    sv48_setup_identity_map();
    sv48_enable();

    /* Step 5: Enable S-mode interrupts */
    sigma_log_info("[RISCV] Step 5: Enabling S-mode interrupts...");
    u64 sie = read_sie();
    sie |= (1 << 1);   /* SSIE: S-mode software interrupt */
    sie |= (1 << 5);   /* STIE: S-mode timer interrupt */
    sie |= (1 << 9);   /* SEIE: S-mode external interrupt */
    write_sie(sie);

    u64 sstatus = read_sstatus();
    sstatus |= (1 << 1);  /* SIE: Enable supervisor interrupts globally */
    write_sstatus(sstatus);
    sigma_log_info("[RISCV] S-mode interrupts enabled (SIE | STIE | SEIE).");

    /* Step 6: Wake secondary harts */
    sigma_log_info("[RISCV] Step 6: Waking secondary harts...");
    u64 secondary_entry = (u64)&riscv64_secondary_hart_entry;
    for (u32 i = 1; i < 4; i++) {
        riscv64_wake_hart(i, secondary_entry);
    }

    sigma_log_info("========================================================");
    sigma_log_info(" RISC-V BOOTSTRAP COMPLETE — %d hart(s) online", online_hart_count);
    sigma_log_info("========================================================");
}

/* ─── SBI System Operations ─────────────────────────────────────────── */

extern "C" void riscv64_system_reset() {
    sigma_log_info("[RISCV] Initiating SBI system reset...");
    sbi_ecall(SBI_EXT_SRST, SBI_SRST_SYSTEM_RESET, 0, 0);
    while (true) { wfi(); }
}

} // namespace RISCV64
