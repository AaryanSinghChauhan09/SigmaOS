/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef SIGMA_SMP_H
#define SIGMA_SMP_H

#include <stdint.h>
#include <stdatomic.h>

#define SIGMA_MAX_CPUS   256
#define SIGMA_CPU_ONLINE (1u << 0)
#define SIGMA_CPU_BSP    (1u << 1)
#define SIGMA_CPU_DEAD   (1u << 2)

struct sigma_cpu_state {
    uint32_t  cpu_id;
    uint32_t  apic_id;
    uint32_t  flags;
    uint32_t  _pad;
    void     *run_queue;   /* per-CPU scheduler run queue */
    void     *idle_thread;
    uint64_t  syscall_stack_top;
    uint64_t  tsc_freq_hz;
    /* Cache-line pad to avoid false sharing between CPUs */
    uint8_t   _cacheline_pad[64 - sizeof(uint32_t)*4 - sizeof(void*)*2 - sizeof(uint64_t)*2];
} __attribute__((aligned(64)));

extern struct sigma_cpu_state sigma_cpus[SIGMA_MAX_CPUS];

int      sigma_smp_init(void);
uint32_t sigma_smp_cpu_count(void);
uint32_t sigma_smp_current_cpu(void);

/* AP entry — called from trampoline */
__attribute__((noreturn)) void sigma_ap_entry(uint32_t cpu_id);

/* Forward declarations satisfied by sigma_lapic.cpp */
uint32_t sigma_lapic_id(void);
uint32_t sigma_lapic_id_to_cpu(uint32_t apic_id);
uintptr_t sigma_lapic_base(void);
void     sigma_lapic_enable(void);
void     sigma_lapic_timer_init(uint32_t period_us);

/* Forward declarations satisfied by sigma_acpi.cpp */
uint32_t sigma_acpi_lapic_count(void);
uint32_t sigma_acpi_lapic_id(uint32_t index);

/* Forward declarations satisfied by sigma_percpu.cpp */
void sigma_percpu_alloc(uint32_t cpu_id);
void sigma_percpu_load(uint32_t cpu_id);

/* Forward declaration satisfied by sigma_sched_sovereign.cpp */
__attribute__((noreturn)) void sigma_sched_ap_start(uint32_t cpu_id);

#endif /* SIGMA_SMP_H */
